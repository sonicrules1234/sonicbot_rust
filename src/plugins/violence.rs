use rand::seq::SliceRandom;
use crate::msgfmts;
use crate::parser::IRCMessage;
use crate::SonicObject;
pub fn permissionlevel() -> u8 {
    1
}
pub fn minargs() -> u8 {
    1
}
pub fn syntax() -> String {
    "violence <nick>".to_string()
}
pub fn help() -> String {
    "Generates a random attack".to_string()
}
pub fn main(ircmsg: IRCMessage, _db: &mut SonicObject, _essentials: SonicObject, _userdata: &mut SonicObject) -> Vec<msgfmts::Message> {
    let mut rseed = rand::thread_rng();
    let verbs = vec!["slaps", "kicks", "barfs on", "punches", "force feeds", "squishes", "stomps on", "bodyslams", "shoots", "smacks", "compresses", "crunches"];
    let adjectives = vec!["sweet", "dirty", "corny", "ugly", "magical", "smelly", "gross old", "old", "tasty", "messy", "blue", "red", "yellow", "pink", "purple", "green", "classic", "stinky"];
    let nouns = vec!["man", "woman", "admin", "IRCop", "car", "fish", "bomb", "missile", "computer", "keyboard", "football", "set of speakers", "monopoly set"];
    let noun = nouns.choose(&mut rseed).unwrap().to_string();
    let adjective = adjectives.choose(&mut rseed).unwrap().to_string();
    let verb = verbs.choose(&mut rseed).unwrap().to_string();
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("\x01ACTION {} {} with a {} {}.\x01", verb, ircmsg.commandargs.as_ref().unwrap()[0], adjective, noun)));
    returnmsgs
}
