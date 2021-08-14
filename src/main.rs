use std::thread::sleep;
use std::time::Duration;
use sonicbot::SonicbotData;
use std::path::{Path};
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::fs;
use serde_json::Value;
use std::thread;
//use futures::executor::block_on;
use std::sync::mpsc::channel;
//use futures::{
//    future::FutureExt, // for `.fuse()`
//    pin_mut,
//    select_biased,
//};

#[derive(Serialize, Deserialize, Clone)]
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
    use linewrapper::LineWrapper;
    let (tx, rx) = channel::<String>();
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    let defaultdata = include_str!("../conf.json.dist").to_string();
    let onandroid = true;
    //let mut onandroid = Path::new("/storage/emulated/0/Android/data/rust.sonicbot/").exists();
    let datadir = Path::new("/storage/emulated/0/Android/media/rust.sonicbot");
    if !datadir.exists() {
        std::fs::create_dir(datadir).unwrap();
    }
    let datadirbuf = datadir.to_path_buf();
    let confpathbuf = datadir.join("conf.json");
    let confpath = confpathbuf.as_path();
    let mut firstrun = false;
    if !confpath.exists() {
        let mut confdist = fs::File::create(&confpath).unwrap();
        confdist.write_all(defaultdata.as_bytes()).unwrap();
        firstrun = true;
        //return;
    }
    //let wholeversion: String = format!("sonicbot_rust_v{}", VERSION.unwrap());
    let data = fs::read_to_string(confpath.to_str().unwrap()).unwrap();
    let mut networklist: Vec<SonicbotConfig> = Vec::new();
    let v: Value = serde_json::from_str(data.as_str());
    for val in v.as_array().unwrap().to_vec() {
        networklist.push(serde_json::from_value::<SonicbotConfig>(val).unwrap());
    }
    let mut linew = LineWrapper::new();
    if !firstrun {
        for p in networklist {
            let ddirbuf = datadirbuf.clone();
            let mut clonedtx = tx.clone();
            let wholeversion: String = format!("sonicbot_rust_v{}", VERSION.unwrap());
            let password = p.password.as_str().to_string();
            let channels = p.channels.to_vec();            
            thread::spawn(move || {
                let mut sbot = SonicbotData::new(p.host, p.port, p.nick, p.ssl, p.ident, p.realname, p.ownernick, p.ownerhost, p.trigger, p.hostlabel, wholeversion, ddirbuf, onandroid, clonedtx);
                sbot.start(password, channels);
            });
        }
    } else {
        linew.println(format!("Blank config created at {}.  Please go and edit it before running the bot again.", confpath.display()));
    }
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
//    $crate::main();
}

#[cfg(not(target_os = "android"))]
fn main() {
    let (tx, _rx) = channel::<String>();
    let (tx2, rx2) = channel::<String>();
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    let defaultdata = include_str!("../conf.json.dist").to_string();
    let onandroid = false;
    //let mut onandroid = Path::new("/storage/emulated/0/Android/data/rust.sonicbot/").exists();
    let datadir = Path::new(".");
    //onandroid = true;
    let datadirbuf = datadir.to_path_buf();
    let confpathbuf = datadir.join("conf.json");
    let confpath = confpathbuf.as_path();
    //let mut firstrun = false;
    if !confpath.exists() {
        let mut confdist = fs::File::create(&confpath).unwrap();
        confdist.write_all(defaultdata.as_bytes()).unwrap();
        println!("Blank config created at {}.  Please go and edit it before running the bot again.", confpath.display());
        return;
    }
    let data = fs::read_to_string(confpath.to_str().unwrap()).unwrap();
    let mut networklist: Vec<SonicbotConfig> = Vec::new();
    let v: Value = serde_json::from_str(data.as_str()).unwrap();
    for val in v.as_array().unwrap().to_vec() {
        networklist.push(serde_json::from_value::<SonicbotConfig>(val).unwrap());
    }
    for p in networklist {
        let ddirbuf = datadirbuf.clone();
        let clonedtx = tx.clone();
        let clonedtx2 = tx2.clone();
        let wholeversion: String = format!("sonicbot_rust_v{}", VERSION.unwrap());
        let password = p.password.as_str().to_string();
        let channels = p.channels.to_vec();            
        thread::spawn(move || {
            clonedtx2.send("START".into()).unwrap();
            let mut sbot = SonicbotData::new(p.host, p.port, p.nick, p.ssl, p.ident, p.realname, p.ownernick, p.ownerhost, p.trigger, p.hostlabel, wholeversion, ddirbuf, onandroid, clonedtx);
            sbot.start(password, channels);
            clonedtx2.send("STOP".into()).unwrap();
        });
    }
    //let p: SonicbotConfig = serde_json::from_str(data.as_str()).unwrap();
    //let password = p.password.as_str().to_string();
    //let channels = p.channels.to_vec();
    //onandroid = true;
    //let mut sbot = SonicbotData::new(p.host, p.port, p.nick, p.ssl, p.ident, p.realname, p.ownernick, p.ownerhost, p.trigger, p.hostlabel, wholeversion, datadirbuf, onandroid, tx.clone());
    //let mut sonicb = sbot.as_mut();
    //sbot.start(p.password, p.channels);
    //let running = Arc::new(AtomicBool::new(true));
    //let r = running.clone();
    sleep(Duration::new(1, 0));
    let mut concount: u8 = 0;
    ctrlc::set_handler(move || {
        tx2.send("QUIT".into()).unwrap();
    }).expect("Error setting Ctrl-C handler");
    let mut result = match rx2.try_recv() {
        Ok(x) => x,
        Err(_) => "ERROR".into(),
    };
    if result == "START".to_string() {
        concount = concount + 1;
    }
    //println!("Waiting for Ctrl-C...");
    while concount > 0 && result != "QUIT" {
        result = match rx2.try_recv() {
            Ok(x) => x,
            Err(_) => "ERROR".into(),
        };
        if result == "START".to_string() {
            concount = concount + 1;
        } else if result == "STOP".to_string() {
            concount = concount - 1;
        }
    }
    //println!("Got it! Exiting...");
}
