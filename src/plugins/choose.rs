use rand::seq::SliceRandom;
use crate::msgfmts;
use crate::parser::IRCMessage;
use crate::SonicObject;
//use acid_store::repo::value::ValueRepo;
pub fn permissionlevel() -> u8 {
    1
}
pub fn minargs() -> u8 {
    1
}
pub fn syntax() -> String {
    "choose <choices seperated by ' or ' without the quotes>".to_string()
}
pub fn help() -> String {
    "Chooses a random choice from <choices>.".to_string()
}
pub fn main(ircmsg: IRCMessage, db: &mut SonicObject, essentials: SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    let choices = ircmsg.argstring.as_ref().unwrap().split(" or ").collect::<Vec<&str>>();
    let choice = choices.choose(&mut rand::thread_rng()).unwrap().to_string();
    returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("I choose {}.", choice)));
    returnmsgs
}
