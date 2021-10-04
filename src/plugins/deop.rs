use crate::msgfmts;
use crate::parser::IRCMessage;
use sonicobject::SonicObject;
pub fn permissionlevel() -> u8 {
    5
}
pub fn minargs() -> u8 {
    0
}
pub fn syntax() -> String {
    "deop [nick]".to_string()
}
pub fn help() -> String {
    "Deops nick in the current channel.  If nick is not specified, will deop the one who sent the command.".to_string()
}
pub fn main(ircmsg: IRCMessage, _db: &mut SonicObject, _essentials: SonicObject, _userdata: &mut SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    let nick: String;
    if ircmsg.commandargs.is_some() {
        nick = ircmsg.commandargs.as_ref().unwrap()[0].as_str().to_string();
    } else {
        nick = ircmsg.sender.as_ref().unwrap().to_string();
    }
    returnmsgs.push(msgfmts::Message::SendRawData(format!("MODE {} -o {}\n", ircmsg.channel.as_ref().unwrap(), nick)));
    returnmsgs
}
