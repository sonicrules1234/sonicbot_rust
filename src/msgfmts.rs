use crate::SonicObject;
pub enum Message {
    SendMsg(String, String),
    SaveData(String, SonicObject),
    PartChannel(String, Option<String>),
    JoinChannel(String),
    SendRawData(String),
    SendPM(String, String),
}
//pub struct SendMsg {
//    recipient: String,
//    message: String,
//}
//impl SendMsg {
//    pub fn new(recipient: String, message: String) -> Self {
//         Self {
//             recipient: recipient,
//             message: message,
//         }
//    }
//}
