
use std::fmt;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::funs::{array_to_string, time_to_string};

#[derive(Debug)]
pub enum Event {
    LifecycleEvent{event:LifecycleEvent},
    HeartbeatEvent{event:HeartbeatEvent},
    GroupMsgEvent{event:GroupMsgEvent},
    PrivateMsgEvent{event:PrivateMsgEvent}
}

impl Event {
    pub fn from(s:&str)-> Result<Event,String>{
        if let Ok(event) = serde_json::from_str::<LifecycleEvent>(s) {
            return Ok(Event::LifecycleEvent { event })
        }else if let Ok(event) = serde_json::from_str::<HeartbeatEvent>(s) {
            return Ok(Event::HeartbeatEvent { event })
        }else if let Ok(event) = serde_json::from_str::<GroupMsgEvent>(s) {
            return Ok(Event::GroupMsgEvent { event })
        }else if let Ok(event) = serde_json::from_str::<PrivateMsgEvent>(s) {
            return Ok(Event::PrivateMsgEvent { event })
        }
        Err("获取事件类型失败".to_string())
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub enum PostType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "message_sent")]
    MessageSent,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "meta_event")]
    MetaEvent
}

#[derive(Debug,Serialize, Deserialize)]
pub enum MessageType{
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "private")]
    Private
}

#[derive(Debug,Serialize, Deserialize)]
pub enum MetaEventType{
    #[serde(rename = "heartbeat")]
    Heartbeat,
    #[serde(rename = "lifecycle")]
    Lifecycle
}

#[derive(Debug,Serialize, Deserialize)]
pub struct LifecycleEvent{//生命周期
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub meta_event_type:String,
    pub sub_type:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct HeartbeatEvent{//心跳
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub meta_event_type:String,
    pub status:Value,
    pub interval:i32
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupMsgEvent{
    pub self_id:i64,
    pub user_id:i64,
    pub time:i64,
    pub message_id:i64,
    pub message_seq:i64,
    pub real_id:i64,
    pub message_type:MessageType,
    pub sender:GroupMsgSender,
    pub raw_message:String,
    pub font:i16,
    pub sub_type:String,
    pub message:Vec<Value>,
    pub message_format:String,
    pub post_type:PostType,
    pub group_id:i64
}

impl fmt::Display for GroupMsgEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"[{}]{}[{: >11}] | {}({}):{} ",self.self_id.to_string().blue(),"群".purple(),self.group_id.to_string().purple(),self.sender.nickname,self.user_id,array_to_string(&self.message))
    }
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupMsgSender{
    pub user_id:i64,
    pub nickname:String,
    pub card:String,
    pub role:GroupRole
}

#[derive(Debug,Serialize, Deserialize)]
pub enum GroupRole{
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member
}

#[derive(Debug,Serialize, Deserialize)]
pub enum GroupSubType {
    Normal,
    Anonymous,
    Notice
}

#[derive(Debug,Serialize, Deserialize)]
pub struct PrivateMsgEvent{
    pub self_id:i64,
    pub user_id:i64,
    pub time:i64,
    pub message_id:i64,
    pub message_seq:i64,
    pub real_id:i64,
    pub message_type:String,
    pub sender:Value,
    pub raw_message:String,
    pub font:i16,
    pub sub_type:String,
    pub message:Value,
    pub message_format:String,
    pub post_type:PostType
}

impl fmt::Display for PrivateMsgEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} [{}][{}]{}[{: >11}] | {}({}):{} ",time_to_string(self.time),"Inf".green(),self.self_id.to_string().blue(),"私".purple(),self.user_id.to_string().purple(),self.sender["nickname"],self.user_id,array_to_string(self.message.as_array().unwrap()))
    }
}

#[derive(Debug,Serialize, Deserialize)]
pub struct EchoEvent{
    pub status:String,
    pub retcode:i8,
    pub data:Value,
    pub message:String,
    pub wording:String,
    pub echo:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct EchoGetStatus{
    pub online:bool,
    pub good:bool,
    pub stat:Value
}

#[derive(Debug,Serialize, Deserialize)]
pub struct EchoGetVersionInfo{
    pub app_name:String,
    pub protocol_version:String,
    pub app_version:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct EchoLoginInfo{
    pub user_id:i64,
    pub nickname:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupMessageSent{
    pub self_id:i64,
    pub user_id:i64,
    pub time:i64,
    pub message_id:i64,
    pub message_seq:i64,
    pub real_id:i64,
    pub message_type:String,
    pub sender:Value,
    pub raw_message:String,
    pub font:i16,
    pub sub_type:String,
    pub message:Value,
    pub message_format:String,
    pub post_type:String,
    pub message_sent_type:String,
    pub group_id:i64,
    pub target_id:i64
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ChangeGroupCard{
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:String,
    pub card_new:String,
    pub card_old:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct EchoAction{
    pub status:String,
    pub retcode:i8,
    pub data:Value,
    pub message:String,
    pub wording:String,
    pub echo:String
}