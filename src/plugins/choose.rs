use rand::seq::SliceRandom;
use crate::msgfmts;
use crate::parser::IRCMessage;
pub fn main(ircmsg: &IRCMessage) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    let choices = ircmsg.argstring.as_ref().unwrap().split(" or ").collect::<Vec<&str>>();
    let choice = choices.choose(&mut rand::thread_rng()).unwrap().to_string();
    returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.channel.as_ref().unwrap().to_string(), format!("I choose {}.", choice)));
    returnmsgs
}
