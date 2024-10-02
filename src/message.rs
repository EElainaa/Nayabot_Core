use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgData{
    pub data:Value,
    pub r#type:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text{
    pub text:String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Image{
    pub file:String,
    pub file_id:String,
    file_size:String,
    sub_type:i8,
    pub url:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mface{//表情包
    pub emoji_id:String,
    emoji_package_id:String,
    pub key:String,
    pub summary:String,
    pub url:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct At{
    pub name:String,
    pub qq:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Json{
    pub data:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forward{
    content:Value,
    pub id:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video{
    pub file:String,
    pub file_id:String,
    pub file_size:String,
    pub path:String,
    pub url:String
}

/// 构建一条JSON格式的消息
/// ```rust
/// let msg = MessageBuilder::new().text("Hello, World!").build();
/// ```
pub struct MessageBuilder{
    msg:Value
}

#[allow(dead_code)]
impl MessageBuilder {
    pub fn new()->MessageBuilder{MessageBuilder{msg:Value::Array(vec![])}}
    pub fn build(self)->Value{self.msg}
    pub fn text<T:ToString>(mut self,s:T)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "text","data": {"text": s.to_string()}}));self}
    pub fn face(mut self,s:u32)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "face","data": {"id": s.to_string()}}));self}
    pub fn image<T:ToString>(mut self,s:T)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "image","data": {"file": s.to_string()}}));self}
    pub fn record<T:ToString>(mut self,s:T)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "record","data": {"file": s.to_string()}}));self}
    pub fn video<T:ToString>(mut self,s:T)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "video","data": {"file": s.to_string()}}));self}
    pub fn at<T:ToString>(mut self,s:T)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "at","data": {"qq": s.to_string()}}));self}
    pub fn rps<T:ToString>(mut self)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "rps"}));self}
    pub fn reply<T:ToString>(mut self,s:T)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "reply","data": {"id": s.to_string()}}));self}
    pub fn forward<T:ToString>(mut self,s:T)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "forward","data": {"id": s.to_string()}}));self}
}

impl Display for MessageBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json=format!("{}",serde_json::json!(self.msg));
        write!(f,"{}", json)
    }
}