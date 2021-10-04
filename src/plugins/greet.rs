use crate::msgfmts;
use crate::parser::IRCMessage;
use sonicobject::SonicObject;
pub fn permissionlevel() -> u8 {
    1
}
pub fn minargs() -> u8 {
    0
}
pub fn syntax() -> String {
    "greet".to_string()
}
pub fn help() -> String {
    "Greets you.".to_string()
}
pub fn main(ircmsg: IRCMessage, _db: &mut SonicObject, _essentials: SonicObject, _userdata: &mut SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("Hello {}.", ircmsg.sender.as_ref().unwrap())));
    returnmsgs
}
