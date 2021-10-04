use serde::{Deserialize, Serialize};
//use crate::SonicbotData;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IRCMessage {
    pub raw: String,
    //pub rawmsg: String,
    pub word1: Option<String>,
    pub msgtype: Option<String>,
    pub numeric: Option<String>,
    pub whois: Option<String>,
    pub sender: Option<String>,
    pub hostname: Option<String>,
    pub channel: Option<String>,
    pub message: Option<String>,
    pub command: Option<String>,
    pub commandargs: Option<Vec<String>>,
    pub argstring: Option<String>,
    pub pong: Option<String>,
    pub ctcp: Option<String>,
    pub aimlcommand: Option<String>,
}

//pub fn parse(mut upperself: &mut SonicbotData, line: String) -> () {
//    let ircmsg: IRCMessage = parseirc(line);
//    upperself.rawsend("Blah".to_string())
//}
pub fn parse(line: String, nick: String, comprefix: String, botnick: String) -> IRCMessage {
    let raw = line.clone();
    let rawwords = raw.split(" ").collect::<Vec<&str>>();
    //let rawmsg = rawwords.join(" ");
    let mut msgtype = None;
    let mut word1 = None;
    let mut aimlcommand = None;
    let mut whois = None;
    let mut sender = None;
    let mut hostname = None;
    let mut channel = None;
    let mut message = None;
    let mut command = None;
    let mut commandargs = None;
    let mut argstring = None;
    let mut commandarglist: Vec<String> = Vec::new();
    let mut pong = None;
    let mut ctcp = None;
    if rawwords[0] == ":PING" || rawwords[0] == "PING" {
        pong = Some(rawwords[1].to_string());
    }
    let linedata: String = (line.clone())[1..].to_string();
    let words = linedata.split(" ").collect::<Vec<&str>>();
    //println!("[WORDS] {}", words[1].to_string());
    let numeric = match words[1].parse::<u16>() {
        Ok(_rnum) => Some(words[1].to_string()),
        Err(_) => None,
    };
    if numeric.is_none() {
        msgtype = Some(words[1].to_string());
    }
    if numeric.is_some() || msgtype.as_ref().is_some() {
        word1 = Some(words[1].to_string());
    }
    if msgtype.as_ref().is_some() {
        whois = Some(words[0].to_owned());
        if whois.as_ref().unwrap().contains("!") {
            sender = Some(whois.as_ref().unwrap().split("!").collect::<Vec<&str>>()[0].to_string());
            hostname = Some(whois.as_ref().unwrap().split("@").collect::<Vec<&str>>()[1].to_string());
            channel = Some(words[2].to_string().replace(":", ""));
            if channel.as_ref().unwrap() == nick.as_str() {
                channel = Some(sender.as_ref().unwrap().to_string());
            }
        }
                //if sender.as_ref().unwrap() == nick.as_str() {
                //    channel = Some(sender.as_ref().unwrap().to_string());
                //} else {
                //    channel = Some(words[2].to_string().replace(":", ""));
                //}
        if vec!("PRIVMSG".to_string()).contains(msgtype.as_ref().unwrap()) {
            if words[3].contains(":") {
                message = Some(words[3..].join(" ")[1..].to_string())
            } else {
                message = Some(words[3..].join(" "))
            }
            if message.as_ref().unwrap().starts_with("\x01") && message.as_ref().unwrap().ends_with("\x01") {
                //println!("{:?}", message.as_ref().unwrap()[1..(message.as_ref().unwrap().len())].to_string());
                ctcp = Some(message.as_ref().unwrap()[1..(message.as_ref().unwrap().len() - 1)].to_string());
            }
            if message.as_ref().unwrap().starts_with(format!("{}: ", botnick).as_str()) {
                aimlcommand = Some(message.as_ref().unwrap().split_once(": ").unwrap().1.to_string());
            }
            if message.as_ref().unwrap().starts_with(&comprefix) {
                command = Some(message.as_ref().unwrap().split(" ").collect::<Vec<&str>>()[0].to_string()[1..].to_string())
            }
            if command.as_ref().is_some() && message.as_ref().unwrap().split(" ").collect::<Vec<&str>>().len() > 1 {
                for carg in words[4..].into_iter() {
                    commandarglist.push(carg.to_string());
                }
                commandargs = Some(commandarglist);
                argstring = Some(commandargs.as_ref().unwrap().join(" "));
            }
        }
    }
    IRCMessage {
        raw: raw,
        //rawmsg: rawmsg,
        word1: word1,
        msgtype: msgtype,
        numeric: numeric,
        whois: whois,
        sender: sender,
        hostname: hostname,
        channel: channel,
        message: message,
        command: command,
        commandargs: commandargs,
        argstring: argstring,
        pong: pong,
        ctcp: ctcp,
        aimlcommand: aimlcommand,
    }
}
