//mod lib;
//use sonicbot::SonicbotData;
//use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use std::fs;
//#[derive(Serialize, Deserialize)]
//struct SonicbotConfig {
//    host: String,
//    port: u16,
//    nick: String,
//    ssl: bool,
//    ident: String,
//    realname: String,
//    ownernick: String,
//    ownerhost: String,
//    trigger: String,
//    channels: Vec<String>,
//    password: String,
//}
//#[cfg_attr(
//    target_os = "android",
//    ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "hello-world"))
//)]
fn main() {
    //let data = fs::read_to_string("conf.json").unwrap();
    //let data = include_str!("conf.json").to_string();
    //let p: SonicbotConfig = serde_json::from_str(data.as_str()).unwrap();
    //let mut sbot = SonicbotData::new(p.host, p.port, p.nick, p.ssl, p.ident, p.realname, p.ownernick, p.ownerhost, p.trigger);
    //sbot.start(p.password, p.channels);
    sonicbot::main();
}
