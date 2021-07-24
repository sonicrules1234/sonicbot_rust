use sonicbot::SonicbotData;

use std::path::{Path, PathBuf};
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::fs;
use std::thread;
//use futures::executor::block_on;
use std::sync::mpsc::{channel, TryRecvError};
//use futures::{
//    future::FutureExt, // for `.fuse()`
//    pin_mut,
//    select_biased,
//};

#[derive(Serialize, Deserialize)]
struct SonicbotConfig {
    host: String,
    port: u16,
    nick: String,
    ssl: bool,
    ident: String,
    realname: String,
    ownernick: String,
    ownerhost: String,
    trigger: String,
    channels: Vec<String>,
    password: String,
    hostlabel: String,
}
#[cfg(target_os = "android")]
#[macroquad::main("sonicbot")]
async fn main() {
    use macroquad::prelude::*;
    let (tx, rx) = channel::<String>();
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    let defaultdata = include_str!("../conf.json.dist").to_string();
    let mut onandroid = Path::new("/storage/emulated/0/Android/data/rust.sonicbot/").exists();
    let datadir: &Path;
    if onandroid {
        datadir = Path::new("/storage/emulated/0/Android/media/rust.sonicbot");
    } else {
        datadir = Path::new(".");
    }
    if !datadir.exists() {
        std::fs::create_dir(datadir).unwrap();
    }
    //onandroid = true;
    let datadirbuf = datadir.to_path_buf();
    let confpathbuf = datadir.join("conf.json");
    let confpath = confpathbuf.as_path();
    if !confpath.exists() {
        let mut confdist = fs::File::create(&confpath).unwrap();
        confdist.write_all(defaultdata.as_bytes()).unwrap();
        return;
    }
    let wholeversion: String = format!("sonicbot_rust_v{}", VERSION.unwrap());
    let data = fs::read_to_string(confpath.to_str().unwrap()).unwrap();
    let p: SonicbotConfig = serde_json::from_str(data.as_str()).unwrap();
    let password = p.password.as_str().to_string();
    let channels = p.channels.to_vec();
    //onandroid = true;
    let mut sbot = SonicbotData::new(p.host, p.port, p.nick, p.ssl, p.ident, p.realname, p.ownernick, p.ownerhost, p.trigger, p.hostlabel, wholeversion, datadirbuf, onandroid, tx.clone());
    //let mut sonicb = sbot.as_mut();
    if onandroid {
        use linewrapper::LineWrapper;
        let mut linew = LineWrapper::new();
        thread::spawn(move || {
            sbot.start(password, channels);
        });
        loop {
            match rx.try_recv() {
                Ok(rx) => linew.println(rx),
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => (),
            };
            //&sbot.showlines();
            //tx.clone().send("showlines").unwrap();
            linew.showlines();
            next_frame().await;
        }
    } else {
        sbot.start(p.password, p.channels);        
    }
//    $crate::main();
}

#[cfg(not(target_os = "android"))]
fn main() {
    let (tx, rx) = channel::<String>();
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    let defaultdata = include_str!("../conf.json.dist").to_string();
    let onandroid = false;
    //let mut onandroid = Path::new("/storage/emulated/0/Android/data/rust.sonicbot/").exists();
    let datadir: &Path;
    if onandroid {
        datadir = Path::new("/storage/emulated/0/Android/media/rust.sonicbot");
    } else {
        datadir = Path::new(".");
    }
    if !datadir.exists() {
        std::fs::create_dir(datadir).unwrap();
    }
    //onandroid = true;
    let datadirbuf = datadir.to_path_buf();
    let confpathbuf = datadir.join("conf.json");
    let confpath = confpathbuf.as_path();
    if !confpath.exists() {
        let mut confdist = fs::File::create(&confpath).unwrap();
        confdist.write_all(defaultdata.as_bytes()).unwrap();
        return;
    }
    let wholeversion: String = format!("sonicbot_rust_v{}", VERSION.unwrap());
    let data = fs::read_to_string(confpath.to_str().unwrap()).unwrap();
    let p: SonicbotConfig = serde_json::from_str(data.as_str()).unwrap();
    let password = p.password.as_str().to_string();
    let channels = p.channels.to_vec();
    //onandroid = true;
    let mut sbot = SonicbotData::new(p.host, p.port, p.nick, p.ssl, p.ident, p.realname, p.ownernick, p.ownerhost, p.trigger, p.hostlabel, wholeversion, datadirbuf, onandroid, tx.clone());
    //let mut sonicb = sbot.as_mut();
    sbot.start(p.password, p.channels);
//    $crate::main();
}