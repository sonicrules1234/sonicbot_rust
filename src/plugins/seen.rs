use crate::msgfmts;
use crate::parser::IRCMessage;
use crate::SonicObject;
use crate::SystemTime;
use humantime::format_duration;
//use std::ops::Sub;
//use std::ops::Add;

//use acid_store::repo::value::ValueRepo;
pub fn permissionlevel() -> u8 {
    1
}
pub fn minargs() -> u8 {
    1
}
pub fn syntax() -> String {
    "seen <nick>".to_string()
}
pub fn help() -> String {
    "Returns time since <nick> was seen.".to_string()
}
pub fn main(ircmsg: IRCMessage, db: &mut SonicObject, essentials: SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    if essentials.get("PRIVMSG").get("seen").contains(ircmsg.commandargs.as_ref().unwrap()[0].as_str()) {
        let dur = essentials.get("PRIVMSG").get("seen").getvalue(ircmsg.commandargs.as_ref().unwrap()[0].as_str());
        let then: SystemTime = SystemTime::UNIX_EPOCH.checked_add(std::time::Duration::new(dur["secs_since_epoch"].as_u64().unwrap(), dur["nanos_since_epoch"].as_u64().unwrap().to_string().as_str().parse().unwrap())).unwrap();
        let elapsed = then.elapsed().unwrap();
        let humanduration = format_duration(elapsed).to_string();
        returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("I last saw {} {} ago.", ircmsg.commandargs.as_ref().unwrap()[0], humanduration)));
    } else {
        returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("I can't find {} in my database.", ircmsg.commandargs.as_ref().unwrap()[0])));        
    }    
    returnmsgs
}
