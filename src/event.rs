
use std::fmt::{self};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::funs::{array_to_string, time_to_string};

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct lifecycle_event{//生命周期
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub meta_event_type:String,
    pub sub_type:String
}

impl fmt::Display for lifecycle_event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} [{}]成功连接Bot{}",time_to_string(self.time),"Inf".green(),self.self_id.to_string().blue())
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct heartbeat_event{//心跳
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub meta_event_type:String,
    pub status:Value,
    pub interval:i32
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct group_msg_event{
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

impl fmt::Display for group_msg_event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} [{}][{}]{}[{: >11}] | {}({}):{} ",time_to_string(self.time),"Inf".green(),self.self_id.to_string().blue(),"群".purple(),self.group_id.to_string().purple(),self.sender["nickname"],self.user_id,array_to_string(self.message.as_array().unwrap()))
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct private_msg_event{
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

impl fmt::Display for private_msg_event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} [{}][{}]{}[{: >11}] | {}({}):{} ",time_to_string(self.time),"Inf".green(),self.self_id.to_string().blue(),"私".purple(),self.user_id.to_string().purple(),self.sender["nickname"],self.user_id,array_to_string(self.message.as_array().unwrap()))
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct echo_event{
    pub status:String,
    pub retcode:i8,
    pub data:Value,
    pub message:String,
    pub wording:String,
    pub echo:String
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct echo_get_status{
    pub online:bool,
    pub good:bool,
    pub stat:Value
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct echo_get_version_info{
    pub app_name:String,
    pub protocol_version:String,
    pub app_version:String
}

