use crate::msgfmts;
use crate::parser::IRCMessage;
use crate::SonicObject;
pub fn permissionlevel() -> u8 {
    5
}
pub fn minargs() -> u8 {
    1
}
pub fn syntax() -> String {
    "join <channel>".to_string()
}
pub fn help() -> String {
    "Joins <channel>.".to_string()
}
pub fn main(ircmsg: IRCMessage, db: &mut SonicObject, essentials: SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    let channel = &ircmsg.commandargs.as_ref().unwrap()[0];
    //let choice = choices.choose(&mut rand::thread_rng()).unwrap().to_string();
    returnmsgs.push(msgfmts::Message::JoinChannel(channel.to_string()));
    returnmsgs
}
