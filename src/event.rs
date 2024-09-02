
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
    post_type:String,
    meta_event_type:String,
    sub_type:String
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
    post_type:String,
    meta_event_type:String,
    pub status:Value,
    interval:i32
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
    self_id:i64,
    user_id:i64,
    time:i64,
    message_id:i64,
    message_seq:i64,
    real_id:i64,
    message_type:String,
    sender:Value,
    raw_message:String,
    font:i16,
    sub_type:String,//maybe friend
    message:Value,
    message_format:String,
    post_type:String
}

impl fmt::Display for private_msg_event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} [{}][{}]{}[{: >11}] | {}({}):{} ",time_to_string(self.time),"Inf".green(),self.self_id.to_string().blue(),"私".purple(),self.user_id.to_string().purple(),self.sender["nickname"],self.user_id,array_to_string(self.message.as_array().unwrap()))
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct notice_event{

}
