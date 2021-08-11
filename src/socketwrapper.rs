//use rustls::ClientSession;
use std::io::BufWriter;
use std::io::BufReader;
use rustls::ClientSession;
use std::io::prelude::*;
use std::net::TcpStream;


pub struct SocketWrapper {
    host: String,
    port: u16,
    ssl: bool,
    clearsock: Option<TcpStream>,
    encsock: Option<rustls::StreamOwned<ClientSession, TcpStream>>,
}
#[allow(dead_code)]
impl SocketWrapper {
    pub fn new(host: String, port: u16, ssl: bool) -> Self {
        Self {
            host: host,
            port: port,
            ssl: ssl,
            clearsock: None,
            encsock: None,
        }
    }
    pub fn get_host(self) -> String {
        self.host
    }
    pub fn get_port(self) -> u16 {
        self.port
    }
    pub fn connect(&mut self) -> () {
        println!("{}:{}", self.host, self.port.to_string());
        let socket = TcpStream::connect(format!("{}:{}", self.host, self.port.to_string())).unwrap();
        socket.set_read_timeout(Some(std::time::Duration::from_millis(500))).unwrap();
        //socket.set_nodelay(true).unwrap();
        println!("Connected");
        if self.ssl {
            //let connector = RustlsConnector::new_with_native_certs().unwrap();
            //self.encsock = Some(connector.connect(self.host.as_str(), socket).unwrap());
            let mut config = rustls::ClientConfig::new();
            config
                .root_store
                .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            let arc = std::sync::Arc::new(config);
            let dns_name = webpki::DNSNameRef::try_from_ascii_str(self.host.as_str()).unwrap();
            let client = rustls::ClientSession::new(&arc, dns_name);
            //let stream = rustls::Stream::new(&mut client, &mut socket);
            self.encsock = Some(rustls::StreamOwned::new(client, socket));
        } else {
            self.clearsock = Some(socket);
        }
        //if self.ssl {
        //    self.readbuffer = Some(Box::new(BufReader::new(self.encsock.unwrap())));
        //} else {
        //    self.readbuffer = Some(Box::new(BufReader::new(self.clearsock.unwrap())));
        //    
        //}
    }
    pub fn read_to_strings(&mut self) -> Vec<String> {
        //let mut currbuffer = Vec::new();
        let currbuffer: Box<dyn std::io::BufRead>;
        //let mut inlines: Vec<String> = Vec::new();
        //let mut count = 1;
        let mut lines: Vec<String> = Vec::new();
        //let totalbuffer = self.buffer.push_str(curr)
        //let stringbuffer = String::from_utf8_lossy(&currbuffer).to_string();
        //println!("{}", stringbuffer);
        //let stringbuflines = stringbuffer.split("\n").collect::<Vec<&str>>();
        //let linecount = stringbuflines.len();
        //for line in stringbuflines {
        //    if count == 1 {
        //        inlines.push(format!("{}{}", self.buffer, line.replace("\r", "")));
        //        self.buffer = String::new();
        //    } else if count == linecount {
        //        self.buffer = line.replace("\r", "").to_string();
        //    } else {
        //        inlines.push(line.replace("\r", ""));
        //    }
        //    count = count + 1;
        //}
        if self.ssl {
            currbuffer = Box::new(BufReader::new(self.encsock.as_mut().unwrap()));
        } else {
            currbuffer = Box::new(BufReader::new(self.clearsock.as_mut().unwrap()));
        }
        for line in currbuffer.lines() {
            let goodline = match line {
                Ok(line) => line.to_string(),
                Err(_) => String::new(),
            };
            if goodline == String::new() {
                break;
            }
            lines.push(goodline);
        }
        lines
        //currbuffer.read_line(&mut line).unwrap();
        //line.to_string()
    }
    pub fn write_all(&mut self, text: String) -> () {
        let mut currbuffer: Box<dyn std::io::Write>;
        //if self.ssl {
        //    self.encsock.as_mut().unwrap().write_all(text.as_bytes());
        //} else {
        //    self.clearsock.as_mut().unwrap().write_all(text.as_bytes());
        //}
        if self.ssl {
            currbuffer = Box::new(BufWriter::new(self.encsock.as_mut().unwrap()));
        } else {
            currbuffer = Box::new(BufWriter::new(self.clearsock.as_mut().unwrap()));
        }
        currbuffer.write(text.as_bytes()).unwrap();
        currbuffer.flush().unwrap();
    }
}
