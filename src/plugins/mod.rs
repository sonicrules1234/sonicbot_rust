use sonicmacros::pluginmacro;
pluginmacro!();
/*use std::collections::HashMap;
use crate::parser::IRCMessage;
use crate::SonicObject;
use crate::msgfmts;
pub mod choose;

pub struct ModList {
    pub modnames: Vec<String>,
    pub mainfunctions: HashMap<String, (Box<dyn Fn(&IRCMessage, &mut SonicObject) -> Vec<msgfmts::Message>>, u8)>,
}
impl ModList {
    pub fn new() -> Self {
        let mut mainfunctions: HashMap<String, (Box<dyn Fn(&IRCMessage, &mut SonicObject) -> Vec<msgfmts::Message>>, u8)> = HashMap::new();
        mainfunctions.insert("choose".to_string(), (Box::new(choose::main), choose::permissionlevel()));
        let modnamesstr = mainfunctions.keys().collect::<Vec<&String>>();
        let mut modnames: Vec<String> = Vec::new();
        for modname in modnamesstr {
            modnames.push(modname.to_string());
        }
        Self {
            modnames: modnames,
            mainfunctions: mainfunctions,
        }
    }
}*/