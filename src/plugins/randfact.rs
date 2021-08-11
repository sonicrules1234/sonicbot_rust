use crate::msgfmts;
use crate::parser::IRCMessage;
use crate::SonicObject;
use regex::Regex;
pub fn permissionlevel() -> u8 {
    1
}
pub fn minargs() -> u8 {
    0
}
pub fn syntax() -> String {
    "randfact".to_string()
}
pub fn help() -> String {
    "Returns a random fact.".to_string()
}
pub fn main(ircmsg: IRCMessage, _db: &mut SonicObject, _essentials: SonicObject, _userdata: &mut SonicObject) -> Vec<msgfmts::Message> {
    let data = reqwest::blocking::get("http://randomfunfacts.com/").unwrap().text().unwrap();
    let matcher = Regex::new(r#"<strong><i>(.*)</i></strong>"#).unwrap();
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("{}: {}", ircmsg.sender.as_ref().unwrap(), matcher.captures(data.as_str()).unwrap()[1].to_string())));
    returnmsgs
}
