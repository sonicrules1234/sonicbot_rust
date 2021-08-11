use crate::msgfmts;
use crate::parser::IRCMessage;
use crate::SonicObject;
//use acid_store::repo::value::ValueRepo;
pub fn permissionlevel() -> u8 {
    5
}
pub fn minargs() -> u8 {
    1
}
pub fn syntax() -> String {
    "part <channel> [reason]".to_string()
}
pub fn help() -> String {
    "Parts from <channel> with [reason] as a reason if it is given.".to_string()
}
pub fn main(ircmsg: IRCMessage, _db: &mut SonicObject, _essentials: SonicObject, _userdata: &mut SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    let reason: Option<String>;
    if ircmsg.commandargs.as_ref().unwrap().len() > 1 {
        reason = Some(ircmsg.commandargs.as_ref().unwrap()[1..].join(" "));

    } else {
        reason = None;
    }
    let channel = &ircmsg.commandargs.as_ref().unwrap()[0];
    //let choice = choices.choose(&mut rand::thread_rng()).unwrap().to_string();
    returnmsgs.push(msgfmts::Message::PartChannel(channel.to_string(), reason));
    returnmsgs
}
