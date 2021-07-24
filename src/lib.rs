mod socketwrapper;
mod parser;
mod plugins;
mod msgfmts;
//use std::collections::BTreeMap;
//use std::collections::HashMap;
use std::path::{Path, PathBuf};
use parser::{IRCMessage};
use socketwrapper::SocketWrapper;
//use macroquad::prelude::*;
use std::sync::mpsc::TryRecvError;
//use acid_store::repo::{OpenOptions, value::ValueRepo, OpenMode};
//use acid_store::store::{DirectoryConfig};
//use acid_store::store::MemoryStore;
//use rand::seq::SliceRandom;
use std::io::Write;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::fs;
//use linewrapper::LineWrapper;
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
    datadir: PathBuf,
    db: sled::Db,
    onandroid: bool,
    tx: std::sync::mpsc::Sender<String>,
}
impl SonicbotData {
    pub fn new(host: String, port: u16, nick: String, ssl: bool, ident: String, realname: String, ownernick: String, ownerhost: String, comprefix: String, hostlabel: String, wholeversion: String, datadir: PathBuf, onandroid: bool, tx: std::sync::mpsc::Sender<String>) -> Self {
        //let rep = OpenOptions::new().mode(OpenMode::Create).open(&DirectoryConfig{ path: datadir.as_path().join("sonicbotdata") }).unwrap();//.create::<ValueRepo<String, _>>().unwrap();
        //let rep = OpenOptions::new(MemoryStore::new()).create::<ValueRepo<String, _>>().unwrap();
        let db: sled::Db = sled::open(datadir.as_path().join("storage.db")).unwrap();
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
            datadir: datadir,
            db: db,
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
            println!("[OUT] {}", msg.as_str().to_string().replace("\r", "").replace("\n", ""));
        } else {
            self.tx.send(format!("[OUT] {}", msg.as_str().to_string().replace("\r", "").replace("\n", ""))).unwrap();
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
                        println!("[IN] {}", line);
                    } else {
                        println!("[IN] {}", line);
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
    pub async fn recv(&mut self, initialchannels: Vec<String>) -> () {
        let mut exitwith: Option<String> = None;
        //let mut drawframe = false;
        //while exitwith.as_ref().is_none() {
        let lines = self.swrapper.read_to_strings();
        //println!("[IN] {}", line);
        for line in lines {
            if line != String::new() {
                self.tx.send(format!("[IN] {}", line)).unwrap();
                let ircmsg = parser::parse(line.to_string(), self.nick.clone(), self.comprefix.clone());
                exitwith = self.takeaction(ircmsg, initialchannels.as_ref());
                //drawframe = true;
            }
        }
        
        if exitwith.is_some() {
            if exitwith.unwrap() == "QUIT" {
                self.rawsend("QUIT :Got quit command!\r\n".to_string());
            }
        }
        //self.showlines();
        //drawframe
    }

    fn sendpm(&mut self, recipient: String, message: String) -> () {
        self.rawsend(format!("PRIVMSG {} :{}\r\n", recipient, message));
    }
    fn sendnotice(&mut self, recipient: String, message: String) -> () {
        self.rawsend(format!("NOTICE {} :{}\r\n", recipient, message));
    }
    fn sendmsg(&mut self, recipient: String, message: String) -> () {
        if recipient.starts_with("#") {
            self.sendpm(recipient, message);
        } else {
            self.sendnotice(recipient, message);
        }
    }
    fn haspermission(&mut self, ircmsg: &IRCMessage, permlevel: u8) -> bool {
        if permlevel == 5 {
            if ircmsg.sender.as_ref().unwrap() == self.ownernick.as_str() && ircmsg.hostname.as_ref().unwrap() == self.ownerhost.as_str() {
                return true;
            } else {
                //println!("'{}' != '{}' and '{}' != '{}'", ircmsg.sender.as_ref().unwrap(), self.ownernick.as_str(), ircmsg.hostname.as_ref().unwrap(), self.ownerhost.as_str());
                return false;
            }
        } else {
            return true;
        }
    }
    fn commandok(&mut self, command: &str, permlevel: u8, ircmsg: &IRCMessage) -> bool {
        ircmsg.command.as_ref().unwrap() == command && self.haspermission(ircmsg, permlevel)
    }
    fn runplugin(&mut self, commands: Vec<msgfmts::Message>) -> () {
        for msg in commands {
            match msg {
                msgfmts::Message::SendMsg(recipient, message) => self.sendmsg(recipient, message),
            };
        }
    }
    fn takeaction(&mut self, ircmsgorig: IRCMessage, initialchannels: &Vec<String>) -> Option<String> {
        let ircmsg = &ircmsgorig;
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
            if self.commandok("greet", 0, ircmsg) {
                self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("Hello {}.", ircmsg.sender.as_ref().unwrap()));
            } else if self.commandok("choose", 1, ircmsg) {
                //let choices = ircmsg.argstring.as_ref().unwrap().split(" or ").collect::<Vec<&str>>();
                //let choice = choices.choose(&mut rand::thread_rng()).unwrap().to_string();
                //self.sendmsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("I choose {}.", choice)); 
                self.runplugin(plugins::choose::main(&ircmsg));
            } else if self.commandok("quit", 5, ircmsg) {
                //self.rawsend("QUIT: Got quit command!\r\n".to_string());
                return Some("QUIT".to_string());
            } else if self.commandok("join", 5, ircmsg) {
                self.joinchannel(ircmsg.commandargs.as_ref().unwrap()[0].to_string());
            } else if self.commandok("part", 5, ircmsg) {
                let reason;
                if ircmsg.commandargs.as_ref().unwrap().len() > 1 {
                    reason = Some(ircmsg.commandargs.as_ref().unwrap()[1..].join(" "));

                } else {
                    reason = None;
                }
                self.partchannel(ircmsg.commandargs.as_ref().unwrap()[0].to_string(), reason);
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
