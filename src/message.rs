use std::{fmt::Display, ops::Index};
use serde::{Deserialize, Deserializer, Serialize};
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
    pub qq:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply{
    pub id:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Json{
    pub data:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forward{
    pub content:Value,
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
/// let msg = Message::new().text("Hello, World!");
/// ```
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Message{
    #[serde(deserialize_with = "deserialize_msg")]
    pub msg:Vec<Value>
}

fn deserialize_msg<'de, D>(deserializer: D) -> Result<Vec<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Array(msg) => Ok(msg),
        _=>{Ok(vec![])}
    }
}
impl Index<usize> for Message{
    type Output = Value;

    fn index(&self, index:usize) -> &Self::Output {
        &self.msg[index]
    }
}
impl From<&str> for Message {
    fn from(value:&str) -> Self {
        Message::new().text(value)
    }
}

#[allow(dead_code)]
impl Message {
    pub fn new()->Message{Message{msg:vec![]}}
    pub fn text<T:ToString>(mut self,s:T)->Message{self.msg.push(serde_json::json!({"type": "text","data": {"text": s.to_string()}}));self}
    pub fn face(mut self,s:u32)->Message{self.msg.push(serde_json::json!({"type": "face","data": {"id": s.to_string()}}));self}
    pub fn image<T:ToString>(mut self,s:T)->Message{self.msg.push(serde_json::json!({"type": "image","data": {"file": s.to_string()}}));self}
    pub fn record<T:ToString>(mut self,s:T)->Message{self.msg.push(serde_json::json!({"type": "record","data": {"file": s.to_string()}}));self}
    pub fn video<T:ToString>(mut self,s:T)->Message{self.msg.push(serde_json::json!({"type": "video","data": {"file": s.to_string()}}));self}
    pub fn at<T:ToString>(mut self,s:T)->Message{self.msg.push(serde_json::json!({"type": "at","data": {"qq": s.to_string()}}));self}
    pub fn rps<T:ToString>(mut self)->Message{self.msg.push(serde_json::json!({"type": "rps"}));self}
    pub fn reply<T:ToString>(mut self,s:T)->Message{self.msg.push(serde_json::json!({"type": "reply","data": {"id": s.to_string()}}));self}
    pub fn forward<T:ToString>(mut self,s:T)->Message{self.msg.push(serde_json::json!({"type": "forward","data": {"id": s.to_string()}}));self}
    /// 转换为JSON格式字符串
    pub fn to_string(&self)->String{format!("{self}")}
    pub fn to_vec(&self)->Vec<Value>{self.msg.clone()}
    pub fn to_array(&self)->Value{Value::Array(self.msg.clone())}

    /// 获取该消息回复
    pub fn get_reply(&self)->Option<Reply>{
        for v in self.msg.clone(){
            if v.get("type") == Some(&Value::String("reply".to_string())){
                if let Some(data) = v.get("data"){
                    if let Some(id) = data.get("id"){
                        return Some(Reply{id:id.to_string()});
                    }
                }
            }
        }
        None
    }

    /// 获取该消息转发消息
    pub fn get_forward(&self)->Option<Forward>{
        for v in self.msg.clone(){
            if v.get("type") == Some(&Value::String("reply".to_string())){
                if let Some(data) = v.get("data"){
                    if let (Some(content),Some(id)) = (data.get("content"),data.get("id")){
                        return Some(Forward{content:content.clone(),id:id.to_string()});
                    }
                }
            }
        }
        None
    }

    /// 获取该消息第一段文字内容
    pub fn get_first_text(&self)->Option<String>{
        for v in self.msg.clone(){
            if v.get("type") == Some(&Value::String("text".to_string())){
                if let Some(data) = v.get("data"){
                    if let Some(text) = data.get("text"){
                        return Some(text.as_str().unwrap().to_string());
                    }
                }
            }
        }
        None
    }

    /// 将另一条消息的内容追加到此消息中
    pub fn push(&mut self,msg:&Message){
        for m in msg.msg.to_vec(){
            match m.get("type").unwrap().as_str().unwrap() {
                "text"|"image"|"at"|"face"=>self.msg.push(m),
                _=>{}
            }
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json=format!("{}",serde_json::json!(self.msg));
        write!(f,"{}", json)
    }
}