use crate::msgfmts;
use crate::parser::IRCMessage;
use crate::SonicObject;
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
pub fn main(ircmsg: IRCMessage, db: &mut SonicObject, essentials: SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    //let channel = &ircmsg.commandargs.as_ref().unwrap()[0];
    //let choice = choices.choose(&mut rand::thread_rng()).unwrap().to_string();
    returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("Hello {}.", ircmsg.sender.as_ref().unwrap())));
    returnmsgs
}
