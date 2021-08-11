mod socketwrapper;
mod parser;
mod plugins;
mod msgfmts;
//use std::collections::BTreeMap;
//use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use parser::{IRCMessage};
use socketwrapper::SocketWrapper;
//use macroquad::prelude::*;
//use std::sync::mpsc::TryRecvError;
//use acid_store::repo::{OpenOptions, value::ValueRepo, OpenMode};
//use acid_store::store::{DirectoryConfig};
//use acid_store::store::MemoryStore;
//use rand::seq::SliceRandom;
//use sled::Db;
//use std::io::Write;
//use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use std::fs;
//use linewrapper::LineWrapper;
use sonicmacros::backinsert;
use sonicobject::{SonicPersistObject, SonicObject};
pub enum CommandErrorReason {
    PermissionError(u8),
    MinArgsError(u8),
    NoSuchCommand(String),
    NoCommandMatch(bool),
}
pub enum PermissionErrorReason {
    HostMismatch(bool),
    NotOwner(bool),
    NotChannelOwner(String),
    NotAdmin(bool),
    NotRegistered(bool),
    LevelTooLow(u8),
}
//#[allow(dead_code)]
pub struct SonicbotData {
    swrapper: Box<SocketWrapper>,
    //ssl: bool,
    ident: String,
    realname: String,
    nick: String,
    //repository: ValueRepo<String>,
    ownernick: String,
    ownerhost: String,
    comprefix: String,
    wholeversion: String,
    hostlabel: String,
    //datadir: PathBuf,
    db: SonicPersistObject,
    networkdata: SonicObject,
    essentialslist: Vec<String>,
    onandroid: bool,
    tx: std::sync::mpsc::Sender<String>,
}
impl SonicbotData {
    pub fn new(host: String, port: u16, nick: String, ssl: bool, ident: String, realname: String, ownernick: String, ownerhost: String, comprefix: String, hostlabel: String, wholeversion: String, datadir: PathBuf, onandroid: bool, tx: std::sync::mpsc::Sender<String>) -> Self {
        //let mut rep = OpenOptions::new().mode(OpenMode::Create).open(&DirectoryConfig{ path: datadir.as_path().join(format!("sonicbotdata_{}", hostlabel).as_str()) }).unwrap();//.create::<ValueRepo<String, _>>().unwrap();
        //let rep = OpenOptions::new(MemoryStore::new()).create::<ValueRepo<String, _>>().unwrap();
        let mut db = SonicPersistObject::new(datadir.as_path().join(format!("sonicbotdata_{}", hostlabel).as_str()));
        let modlist = plugins::ModList::new();
        let essentialslist = vec!["PRIVMSG".to_string(), "JOIN".to_string(), "PART".to_string(), "NICK".to_string()];
        if !db.contains("essentials") {
            db.insert("essentials", sonicobject::getemptyvalue());
        }
        for essentialname in &essentialslist {
            if !db.get("essentials").contains(essentialname.as_str()) {
                let mut newinsert = db.get("essentials");
                newinsert.insert(essentialname.as_str(), sonicobject::getemptyvalue());
                db.insert("essentials", newinsert.value);
            }
        }
        if !db.contains("plugins") {
            db.insert("plugins", sonicobject::getemptyvalue());
        }
        for pluginname in modlist.modnames {
            if !db.get("plugins").contains(pluginname.as_str()) {
                let mut newinsert = db.get("plugins");
                newinsert.insert(pluginname.as_str(), sonicobject::getemptyvalue());
                db.insert("plugins", newinsert.value);
            }
        }
        if !db.contains("users") {
            db.insert("users", sonicobject::getemptyvalue());
        }
        let mut networkdata = SonicObject::new(sonicobject::getemptyvalue());
        networkdata.insert("channels", sonicobject::getemptyvalue());
        networkdata.insert("nicks", sonicobject::getemptyvalue());
        //let mut linew = LineWrapper::new();
        Self {
            swrapper: Box::new(SocketWrapper::new(host.to_string(), port, ssl)),
            //ssl: ssl,
            ident: ident.to_string(),
            realname: realname.to_string(),
            nick: nick.to_string(),
            //repository: rep,
            ownernick: ownernick.to_string(),
            ownerhost: ownerhost.to_string(),
            comprefix: comprefix.to_string(),
            wholeversion: wholeversion,
            hostlabel: hostlabel,
            //datadir: datadir,
            db: db,
            networkdata: networkdata,
            essentialslist: essentialslist, 
            onandroid: onandroid,
            tx: tx.clone(),
        }
    }
    fn connect(&mut self) -> () {
        self.swrapper.connect();
        //self.repository.insert("nick".to_string(), &self.nick).unwrap();
        //self.repository.commit().unwrap();
        //let gotnick: String = self.repository.get(&"nick".to_string()).unwrap();
        //println!("Retrieved nick = '{}'", gotnick);
    }
    fn passsend(&mut self, password: String) -> () {
        self.rawsend(format!("PASS {}\r\n", password));
    }
    fn rawsend(&mut self, msg: String) -> () {
        self.swrapper.write_all(msg.as_str().to_string());
        if !self.onandroid {
            println!("[OUT {}] {}", self.hostlabel, msg.as_str().to_string().replace("\r", "").replace("\n", ""));
        } else {
            self.tx.send(format!("[OUT {}] {}", self.hostlabel, msg.as_str().to_string().replace("\r", "").replace("\n", ""))).unwrap();
        }
    }
    fn nicksend(&mut self) -> () {
        self.rawsend(format!("NICK {}\r\n", self.nick));
    }
    fn usersend(&mut self) -> () {
        self.rawsend(format!("USER {} * * :{}\r\n", self.ident, self.realname));
    }
    fn recvloop(&mut self, initialchannels: Vec<String>) -> () {
        let mut exitwith: Option<String> = None;
        while exitwith.as_ref().is_none() {
            let lines = self.swrapper.read_to_strings();
            //println!("Test 1");
            //println!("[IN] {}", line);
            for line in lines {
                if line != String::new() {
                    if self.onandroid {
                        self.tx.send(format!("[IN] {}", line)).unwrap();
                        println!("[IN {}] {}", self.hostlabel, line);
                    } else {
                        println!("[IN {}] {}", self.hostlabel, line);
                    }
                    let ircmsg = parser::parse(line.to_string(), self.nick.clone(), self.comprefix.clone());
                    exitwith = self.takeaction(ircmsg, initialchannels.as_ref());
                    //let status = crate::keepgoing(rx);
                    //if status == "showlines" {
                    //    self.showlines();
                    //tx.send(self.linew);
                    //}
                }
            }
        }
        if exitwith.as_ref().unwrap() == "QUIT" {
            self.rawsend("QUIT :Got quit command!\r\n".to_string());
            //self.swrapper.shutdown();
        }
    }

    fn sendpm(&mut self, recipient: String, message: String) -> () {
        let mut msgs: Vec<String> = Vec::new();
        for msg in message.split("\n") {
            let filled = textwrap::fill(msg.to_string().as_str(), 400 as usize);
            for line in filled.split("\n") {
                msgs.push(line.to_string())
            }
        }
        for line in msgs {
            self.rawsend(format!("PRIVMSG {} :{}\r\n", recipient, line));
        }
    }
    fn sendnotice(&mut self, recipient: String, message: String) -> () {
        let mut msgs: Vec<String> = Vec::new();
        for msg in message.split("\n") {
            let filled = textwrap::fill(msg.to_string().as_str(), 400 as usize);
            for line in filled.split("\n") {
                msgs.push(line.to_string())
            }
        }
        for line in msgs {
            self.rawsend(format!("NOTICE {} :{}\r\n", recipient, line));
        }
    }
    fn sendmsg(&mut self, recipient: String, message: String) -> () {
        if recipient.starts_with("#") {
            self.sendpm(recipient, message);
        } else {
            self.sendnotice(recipient, message);
        }
    }
    fn haspermission(&mut self, ircmsg: &IRCMessage, permlevel: u8) -> Result<bool, PermissionErrorReason> {
        if ircmsg.sender.as_ref().unwrap() == self.ownernick.as_str() && ircmsg.hostname.as_ref().unwrap() == self.ownerhost.as_str() {
            return Ok(true);
        } else if permlevel == 5 {
            if ircmsg.sender.as_ref().unwrap() == self.ownernick.as_str() && ircmsg.hostname.as_ref().unwrap() == self.ownerhost.as_str() {
                return Ok(true);
            } else {
                //println!("'{}' != '{}' and '{}' != '{}'", ircmsg.sender.as_ref().unwrap(), self.ownernick.as_str(), ircmsg.hostname.as_ref().unwrap(), self.ownerhost.as_str());
                return Err(PermissionErrorReason::NotOwner(false));
            }
        } else if permlevel == 4 {
            return Err(PermissionErrorReason::NotAdmin(false));
        } else if permlevel == 3 {
            return Err(PermissionErrorReason::NotChannelOwner(ircmsg.channel.as_ref().unwrap().to_string()))
        } else if permlevel == 2 {
            if self.db.get("users").contains(ircmsg.sender.as_ref().unwrap()) {    
                let mut thisuser = self.db.get("users").get(ircmsg.sender.as_ref().unwrap());
                let hostvec: Vec<String> = thisuser.getvalue("hostnames").as_array().unwrap().to_vec().iter().filter_map(|x| Some(x.as_str().unwrap().to_string())).collect();
                if hostvec.contains(ircmsg.hostname.as_ref().unwrap()) {
                    return Ok(true);
                } else {
                    return Err(PermissionErrorReason::HostMismatch(false));
                }
            } else {
                return Err(PermissionErrorReason::NotRegistered(false));
            }
        } else if permlevel == 1 {
            return Ok(true);
        } else {
            return Err(PermissionErrorReason::LevelTooLow(permlevel));
        }
    }
    fn handle_haspermission(&mut self, ircmsg: &IRCMessage, permlevel: u8) -> bool {
        match self.haspermission(ircmsg, permlevel) {
            Ok(x) => return x,
            Err(PermissionErrorReason::HostMismatch(_x)) => {
                self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), "Your hostname does not match any of the registered hostnames for your nick.  If it is really you, try using the addhost command.".to_string());
                return false;
            },
            Err(PermissionErrorReason::NotOwner(_x)) => {
                self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), "You are not the owner of this bot.".to_string());
                return false;
            },
            Err(PermissionErrorReason::NotAdmin(_x)) => {
                self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), "You are not an admin of this bot.".to_string());
                return false;
            },
            Err(PermissionErrorReason::NotChannelOwner(_x)) => {
                self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), "You are not registered with this bot as owner of this channel.".to_string());
                return false;
            },
            Err(PermissionErrorReason::NotRegistered(_x)) => {
                self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), "This command requires you to be registered with this bot.  Use the nickreg command to register your nick.".to_string());
                return false;
            },
            Err(PermissionErrorReason::LevelTooLow(x)) => {
                self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("Your user level is too low.  This command requires a minimum user level of {}.", x));
                return false;
            },
        }
    }
    fn handle_haspermission_noreason(&mut self, ircmsg: &IRCMessage, permlevel: u8) -> bool {
        match self.haspermission(ircmsg, permlevel) {
            Ok(_x) => return true,
            Err(_) => {
                //self.sendmsg(ircmsg.sender.as_ref().unwrap().to_string(), "Your hostname does not match any of the registered hostnames for your nick.  If it is really you, try using the addhost command.".to_string());
                return false;
            },
        }
    }
    fn commandok(&mut self, command: &str, permlevel: u8, ircmsg: &IRCMessage, minargs: u8) -> Result<bool, CommandErrorReason> {
        let modlist = plugins::ModList::new();
        if modlist.modnames.clone().contains(&command.to_string()) {
            if ircmsg.command.as_ref().unwrap() == command && self.handle_haspermission(ircmsg, permlevel) {
                if minargs == 0 {
                    Ok(true)
                } else if ircmsg.commandargs.is_some() {
                    if ircmsg.commandargs.as_ref().unwrap().len() >= minargs.into() {
                        Ok(true)
                    } else {
                        Err(CommandErrorReason::MinArgsError(minargs))
                    }
                } else {
                    Err(CommandErrorReason::MinArgsError(minargs))
                }
            } else if ircmsg.command.as_ref().unwrap() != command {
                Err(CommandErrorReason::NoCommandMatch(false))
            } else {
                Err(CommandErrorReason::PermissionError(permlevel))
            }
        } else {
            Err(CommandErrorReason::NoSuchCommand(command.to_string()))
        }
    }
    fn handle_commandok(&mut self, command: &str, permlevel: u8, ircmsg: &IRCMessage, minargs: u8) -> bool {
        match self.commandok(command, permlevel, ircmsg, minargs) {
            Ok(x) => return x,
            Err(CommandErrorReason::NoSuchCommand(_x)) => {
                self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("No such command: {}.", command));
                return false;
            },
            Err(CommandErrorReason::PermissionError(_x)) => {
                //self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("{}: You do not have the required permissions to run this command.", ircmsg.sender.as_ref().unwrap()));
                return false;
            },
            Err(CommandErrorReason::MinArgsError(x)) => {
                if minargs > 1 {
                    self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("{}: Minimum number of arguments not met.  The {} command requires at least {} arguments.", ircmsg.sender.as_ref().unwrap(), command, x));
                } else {
                    self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("{}: Minimum number of arguments not met.  The {} command requires at least {} argument.", ircmsg.sender.as_ref().unwrap(), command, x));
                }
                return false;
            },
            Err(CommandErrorReason::NoCommandMatch(_x)) => return false,
        };
    }
    fn handle_commandok_notthere(&mut self, command: &str, permlevel: u8, ircmsg: &IRCMessage, minargs: u8) -> bool {
        if ircmsg.command.as_ref().unwrap() == command && self.handle_haspermission_noreason(ircmsg, permlevel) {
            if minargs == 0 {
                true
            } else if ircmsg.commandargs.is_some() {
                if ircmsg.commandargs.as_ref().unwrap().len() >= minargs.into() {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    //fn handle_commandok_noreason(&mut self, command: &str, permlevel: u8, ircmsg: &IRCMessage, minargs: u8) -> bool {
    //    match self.commandok(command, permlevel, ircmsg, minargs) {
    //        Ok(x) => return x,
    //        Err(CommandErrorReason::MinArgsError(_x)) => return false,
    //        Err(CommandErrorReason::PermissionError(_x)) => return false,
    //        Err(CommandErrorReason::NoSuchCommand(_x)) => return false,
    //        Err(CommandErrorReason::NoCommandMatch(_x)) => return false,
    //    }        
    //}
    fn runplugin(&mut self, commands: Vec<msgfmts::Message>) -> () {
        for msg in commands {
            match msg {
                msgfmts::Message::SendMsg(recipient, message) => self.sendmsg(recipient, message),
                msgfmts::Message::SavePluginData(pluginname, data) => {
                    let mut newinsert = self.db.get("plugins");
                    newinsert.insert(pluginname.as_str(), data.value);
                    self.db.insert("plugins", newinsert.value);
                },
                msgfmts::Message::SaveUserData(data) => self.db.insert("users", data.value),
                msgfmts::Message::SendRawData(rawdata) => self.rawsend(rawdata),
                msgfmts::Message::JoinChannel(channelname) => self.joinchannel(channelname),
                msgfmts::Message::PartChannel(channelname, reason) => self.partchannel(channelname, reason),
                msgfmts::Message::SendPM(channel, message) => self.sendpm(channel, message), 
            };
        }
    }
    fn takeaction(&mut self, ircmsgorig: IRCMessage, initialchannels: &Vec<String>) -> Option<String> {
        let ircmsg = &ircmsgorig;
        if ircmsg.word1.is_some() {
            if self.essentialslist.contains(&ircmsg.word1.as_ref().unwrap().to_string()) {
                if ircmsg.word1.as_ref().unwrap().as_str() == "PRIVMSG" {
                    if !self.db.get("essentials").get("PRIVMSG").contains("seen") {
                        //let mut newinsert = self.db.get("essentials").get("PRIVMSG");
                        //newinsert.insert("seen", sonicobject::getemptyvalue());
                        let mut xobj = self.db.get("essentials");
                        //self.db.insert("essentials", );
                        let emptyval = sonicobject::getemptyvalue();
                        backinsert!(["PRIVMSG", "seen", emptyval]);
                        self.db.insert("essentials", xobj.value);
                    } else if ircmsg.channel.is_some() {
                        let mut xobj = self.db.get("essentials");
                        let sender = ircmsg.sender.as_ref().unwrap();
                        let datetime = SystemTime::now();
                        backinsert!(["PRIVMSG", "seen", sender, datetime]);
                        self.db.insert("essentials", xobj.value);
                    }
                }
            }
        }
        if ircmsg.numeric.is_some() {
            if ircmsg.numeric.as_ref().unwrap() == "001" {
                for channel in initialchannels {
                    self.joinchannel(channel.to_string());
                }
            }
        } else if ircmsg.ctcp.is_some() {
            if ircmsg.ctcp.as_ref().unwrap() == "VERSION" {
                self.sendpm(ircmsg.sender.as_ref().unwrap().to_string(), format!("\x01VERSION {}\x01.", self.wholeversion));
            }
        } else if ircmsg.command.is_some() {
            let modlist = plugins::ModList::new();
            if modlist.modnames.clone().contains(ircmsg.command.as_ref().unwrap()) {
                if self.handle_commandok(ircmsg.command.as_ref().unwrap(), modlist.permissions[ircmsg.command.as_ref().unwrap()], ircmsg, modlist.minargs[ircmsg.command.as_ref().unwrap()]) {
                    self.runplugin(modlist.mainfunctions[ircmsg.command.as_ref().unwrap()](ircmsg.clone(), &mut self.db.get("plugins").get(ircmsg.command.as_ref().unwrap().as_str()), self.db.get("essentials"), &mut self.db.get("users")));
                }
            }
            if self.handle_commandok_notthere("quit", 5, ircmsg, 0) {
                //self.rawsend("QUIT: Got quit command!\r\n".to_string());
                return Some("QUIT".to_string());
            } else if self.handle_commandok_notthere("detailedhelp", 1, ircmsg, 1) {
                if modlist.modnames.clone().contains(&ircmsg.commandargs.as_ref().unwrap()[0]) {
                    self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("Detailed help for {}: {}", ircmsg.commandargs.as_ref().unwrap()[0], modlist.helps[&ircmsg.commandargs.as_ref().unwrap()[0]]));
                } else {
                    self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("{} not found in command list or {} has no detailed help.", ircmsg.commandargs.as_ref().unwrap()[0], ircmsg.commandargs.as_ref().unwrap()[0]))
                }
            } else if self.handle_commandok_notthere("help", 1, ircmsg, 1) {
                if modlist.modnames.clone().contains(&ircmsg.commandargs.as_ref().unwrap()[0]) {
                    self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("Syntax for {}: {}{}", ircmsg.commandargs.as_ref().unwrap()[0], self.comprefix, modlist.syntaxes[&ircmsg.commandargs.as_ref().unwrap()[0]]));
                } else {
                    self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("{} not found in command list or {} has no detailed help.", ircmsg.commandargs.as_ref().unwrap()[0], ircmsg.commandargs.as_ref().unwrap()[0]))
                }
            } else if self.handle_commandok_notthere("list", 1, ircmsg, 0) {
                self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("{}: I'll send you the list of commands in a notice.", ircmsg.sender.as_ref().unwrap()));
                let mut commands: Vec<String> = modlist.modnames.clone();
                commands.push("quit".to_string());
                commands.push("help".to_string());
                commands.push("detailedhelp".to_string());
                commands.push("list".to_string());
                commands.sort();
                self.sendnotice(ircmsg.sender.as_ref().unwrap().to_string(), format!("List of commands: {}", commands.join(", ")))                
            }
        } else if ircmsg.pong.is_some() {
            self.rawsend(format!("PONG {}\r\n", ircmsg.pong.as_ref().unwrap()));
        }
        None
    }
    fn joinchannel(&mut self, channel: String) -> () {
        self.rawsend(format!("JOIN {} \r\n", channel));
    }
    fn partchannel(&mut self, channel: String, reason: Option<String>) -> () {
        let partextra: String;
        if reason.is_some() {
            partextra = format!(" :{}", reason.unwrap());
        } else {
            partextra = String::new();
        }
        self.rawsend(format!("PART {}{}\r\n", channel, partextra));
    }
    pub fn start(&mut self, password: String, initialchannels: Vec<String>) -> () {
        self.connect();
        self.passsend(password);
        self.nicksend();
        self.usersend();
        self.recvloop(initialchannels);
    }
}
//fn createconnection(host: String, port: u32) -> 
//#[derive(Serialize, Deserialize)]
//struct SonicbotConfig {
//    host: String,
//    port: u16,
//    nick: String,
//    ssl: bool,
//    ident: String,
//    realname: String,
//    ownernick: String,
//    ownerhost: String,
//    trigger: String,
//    channels: Vec<String>,
//    password: String,
//    hostlabel: String,
//}
//#[cfg_attr(
//    target_os = "android",
//    ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "hello-world"))
//)]
//#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
//pub fn main() {
//    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
//    let defaultdata = include_str!("../conf.json.dist").to_string();
//    let onandroid = Path::new("/storage/emulated/0/Android/data/rust.sonicbot/").exists();
//    let datadir: &Path;
//    if onandroid {
//        datadir = Path::new("/storage/emulated/0/Android/media/rust.sonicbot");
//    } else {
//        datadir = Path::new(".");
//    }
//    if !datadir.exists() {
//        std::fs::create_dir(datadir).unwrap();
//    }
//    let datadirbuf = datadir.to_path_buf();
//    let confpathbuf = datadir.join("conf.json");
//    let confpath = confpathbuf.as_path();
//    if !confpath.exists() {
//        let mut confdist = fs::File::create(&confpath).unwrap();
//        confdist.write_all(defaultdata.as_bytes()).unwrap();
//        return;
//    }
//    let wholeversion: String = format!("sonicbot_rust_v{}", VERSION.unwrap());
//    let data = fs::read_to_string(confpath.to_str().unwrap()).unwrap();
//    let p: SonicbotConfig = serde_json::from_str(data.as_str()).unwrap();
//    let mut sbot = SonicbotData::new(p.host, p.port, p.nick, p.ssl, p.ident, p.realname, p.ownernick, p.ownerhost, p.trigger, p.hostlabel, wholeversion, datadirbuf);
//    sbot.start(p.password, p.channels);
//    $crate::main();
//}
//pub fn keepgoing<'a>(rx: &std::sync::mpsc::Receiver<&'a str>) -> &'a str {
//    let status = match rx.try_recv() {
//        Ok(rx) => rx,
//        Err(TryRecvError::Empty) => "empty",
//        Err(TryRecvError::Disconnected) => "disconnected",
//    };
//    status
//}

