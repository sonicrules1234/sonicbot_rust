use crate::msgfmts;
use crate::parser::IRCMessage;
use crate::SonicObject;
use rand::seq::SliceRandom;
use sha2::{Sha512, Digest};
use serde_json::json;
//use acid_store::repo::value::ValueRepo;
pub fn permissionlevel() -> u8 {
    1
}
pub fn minargs() -> u8 {
    1
}
pub fn syntax() -> String {
    "nickreg <password>".to_string()
}
pub fn help() -> String {
    "Registers your nick.".to_string()
}
pub fn main(ircmsg: IRCMessage, _db: &mut SonicObject, _essentials: SonicObject, userdata: &mut SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    if !userdata.contains(ircmsg.sender.as_ref().unwrap()) {
        let posssalt: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()_+=-\\|[]:;,./<>?:\"{}".chars().collect();
        let mut rseed = rand::thread_rng();
        let mut saltstring = String::new();
        for _x in 1..6 {
            saltstring.push(*posssalt.choose(&mut rseed).unwrap())
        }
        let mut unhashed = ircmsg.commandargs.as_ref().unwrap()[0].to_string();
        unhashed.push_str(saltstring.as_str());
        let mut hasher = Sha512::new();
        hasher.update(unhashed.as_bytes());
        let result = hasher.finalize();
        let hexresult = hex::encode(result);
        let mut thisuser = SonicObject::new(sonicobject::getemptyvalue());
        thisuser.insert("salt", saltstring);
        thisuser.insert("hash", hexresult);
        thisuser.insert("hostnames", json!([ircmsg.hostname.as_ref().unwrap()]));
        thisuser.insert("userlevel", 2);
        userdata.insert(ircmsg.sender.as_ref().unwrap(), thisuser.value);
        returnmsgs.push(msgfmts::Message::SaveUserData(userdata.clone()));
        returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.sender.as_ref().unwrap().to_string(), format!("Successfully registered {}.", ircmsg.sender.as_ref().unwrap().to_string())));
    } else {
        returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.sender.as_ref().unwrap().to_string(), format!("This username has already been taken.")));
    }
    returnmsgs
}