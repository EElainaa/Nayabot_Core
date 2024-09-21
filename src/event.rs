
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
    pub fn get_lifecycle_event(self) -> Result<LifecycleEvent,String>{
        match self {
            Event::LifecycleEvent{event} =>return  Ok(event),
            _=>{return Err("生命周期事件转换失败".to_string());}
        }
    }
    pub fn get_heartbeat_event(self) -> Result<HeartbeatEvent,String>{
        match self {
            Event::HeartbeatEvent{event} =>return  Ok(event),
            _=>{return Err("心跳周期事件转换失败".to_string());}
        }
    }
    pub fn get_group_msg_event(self) -> Result<GroupMsgEvent,String>{
        match self {
            Event::GroupMsgEvent{event} =>return  Ok(event),
            _=>{return Err("群聊消息事件转换失败".to_string());}
        }
    }
    pub fn get_private_msg_event(self) -> Result<PrivateMsgEvent,String>{
        match self {
            Event::PrivateMsgEvent{event} =>return  Ok(event),
            _=>{return Err("私聊消息事件转换失败".to_string());}
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleEvent{//生命周期
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub meta_event_type:String,
    pub sub_type:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatEvent{//心跳
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub meta_event_type:String,
    pub status:Value,
    pub interval:i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMsgEvent{
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
    pub group_id:i64
}

impl fmt::Display for GroupMsgEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} [{}][{}]{}[{: >11}] | {}({}):{} ",time_to_string(self.time),"Inf".green(),self.self_id.to_string().blue(),"群".purple(),self.group_id.to_string().purple(),self.sender["nickname"],self.user_id,array_to_string(self.message.as_array().unwrap()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub sub_type:String,//maybe friend
    pub message:Value,
    pub message_format:String,
    pub post_type:String
}

impl fmt::Display for PrivateMsgEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} [{}][{}]{}[{: >11}] | {}({}):{} ",time_to_string(self.time),"Inf".green(),self.self_id.to_string().blue(),"私".purple(),self.user_id.to_string().purple(),self.sender["nickname"],self.user_id,array_to_string(self.message.as_array().unwrap()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoEvent{
    pub status:String,
    pub retcode:i8,
    pub data:Value,
    pub message:String,
    pub wording:String,
    pub echo:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoGetStatus{
    pub online:bool,
    pub good:bool,
    pub stat:Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoGetVersionInfo{
    pub app_name:String,
    pub protocol_version:String,
    pub app_version:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoLoginInfo{
    pub user_id:i64,
    pub nickname:String
}

