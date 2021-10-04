use crate::msgfmts;
use crate::parser::IRCMessage;
use crate::SonicObject;
//use rand::seq::SliceRandom;
use sha2::{Sha512, Digest};
//use serde_json::json;
//use acid_store::repo::value::ValueRepo;
pub fn permissionlevel() -> u8 {
    2
}
pub fn minargs() -> u8 {
    1
}
pub fn syntax() -> String {
    "addhost <password>".to_string()
}
pub fn help() -> String {
    "Adds your current hostname to your account.".to_string()
}
pub fn main(ircmsg: IRCMessage, _db: &mut SonicObject, _essentials: SonicObject, userdata: &mut SonicObject) -> Vec<msgfmts::Message> {
    let mut returnmsgs: Vec<msgfmts::Message> = Vec::new();
    if userdata.contains(ircmsg.sender.as_ref().unwrap()) {
        let saltstring = userdata.get(ircmsg.sender.as_ref().unwrap()).unwrap().getvalue("salt").unwrap();
        let mut unhashed = ircmsg.commandargs.as_ref().unwrap()[0].to_string();
        unhashed.push_str(saltstring.as_str().unwrap());
        let mut hasher = Sha512::new();
        hasher.update(unhashed.as_bytes());
        let result = hasher.finalize();
        let hexresult = hex::encode(result);
        if hexresult == userdata.get(ircmsg.sender.as_ref().unwrap()).unwrap().getvalue("hash").unwrap().as_str().unwrap() {
            let mut thisuser = userdata.get(ircmsg.sender.as_ref().unwrap()).unwrap();
            let mut hostnames = thisuser.get("hostnames").unwrap();
            hostnames.push(ircmsg.hostname.as_ref().unwrap().to_string());
            thisuser.insert("hostnames", hostnames.value);
            returnmsgs.push(msgfmts::Message::SaveUserData(userdata.clone()));    
            returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.sender.as_ref().unwrap().to_string(), format!("Successfully added your hostname.")));
        } else {
            returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.sender.as_ref().unwrap().to_string(), format!("Incorrect password.")));
        }        
    } else {
        returnmsgs.push(msgfmts::Message::SendMsg(ircmsg.sender.as_ref().unwrap().to_string(), format!("This nick has not been registered.")));
    }
    returnmsgs
}