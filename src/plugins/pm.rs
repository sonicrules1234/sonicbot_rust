use crate::msgfmts;
use crate::parser::IRCMessage;
use sonicobject::SonicObject;
pub fn permissionlevel() -> u8 {
    5
}
pub fn minargs() -> u8 {
    2
}
pub fn syntax() -> String {
    "pm <channel> <message>".to_string()
}
pub fn help() -> String {
    "Sends a pm to <channel> with a <message>".to_string()
}
pub fn main(ircmsg: IRCMessage, _db: &mut SonicObject, _essentials: SonicObject, _userdata: &mut SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    returnmsgs.push(msgfmts::Message::SendPM(ircmsg.commandargs.as_ref().unwrap()[0].to_string(), format!("{}", ircmsg.commandargs.as_ref().unwrap()[1..].join(" ").to_string())));
    returnmsgs
}
