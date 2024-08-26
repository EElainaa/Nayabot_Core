use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use ws::Sender;

#[allow(dead_code)]

pub struct Bot{
    sender:Sender
}

#[allow(dead_code)]
impl Bot {
    pub fn new(sender:Sender)->Bot{Bot{sender:sender}}
    pub fn send_private_msg<T:Display>(&self,id:&i64,s:T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"send_private_msg\",\"params\": {{\"user_id\":{},\"message\":{}}}}}",id,s))}
    pub fn send_group_msg<T:Display>(&self,id:&i64,s:T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"send_group_msg\",\"params\": {{\"group_id\":{},\"message\":{}}}}}",id,s))}
    pub fn delete_msg(&self,msg_id:&i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"delete_msg\",\"params\": {{\"message_id\":{}}}}}",msg_id))}
    pub fn get_msg(&self,msg_id:&i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_msg\",\"params\": {{\"message_id\":{}}}}}",msg_id))}
    pub fn get_forward_msg(&self,id:&i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_forward_msg\",\"params\": {{\"id\":{}}}}}",id))}
    pub fn send_like(&self,user_id:&i64,times:i16)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"send_like\",\"params\": {{\"user_id\":{},\"times\":{}}}}}",user_id,times))}
    pub fn set_group_kick(&self,group_id:&i64,user_id:&i64,reject_add_request:&bool)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_group_kick\",\"params\": {{\"group_id\":{}, \"user_id\":{},\"reject_add_request\":{}}}}}",group_id,user_id,reject_add_request))}
    pub fn set_group_ban(&self,group_id:&i64,user_id:&i64,duration:i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_group_ban\",\"params\": {{\"group_id\":{},\"user_id\":{},\"duration\":{}}}}}",group_id,user_id,duration))}
    //pub fn set_group_anonymous_ban(&self){}
    pub fn set_group_whole_ban(&self,group_id:&i64,enable:&bool)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_group_whole_ban\",\"params\": {{\"group_id\":{},\"enable\":{}}}}}",group_id,enable))}
    pub fn set_group_admin(&self,group_id:&i64,user_id:&i64,enable:&bool)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_group_admin\",\"params\": {{\"group_id\":{},\"user_id\":{},\"enable\":{}}}}}",group_id,user_id,enable))}
    //pub fn set_group_anonymous(&self){}
    pub fn set_group_card<T:Display>(&self,group_id:&i64,user_id:&i64,card:&T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_group_card\",\"params\": {{\"group_id\":{},\"user_id\":{},\"card\":\"{}\"}}}}",group_id,user_id,card))}
    pub fn set_group_name<T:Display>(&self,group_id:&i64,group_name:&T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_group_name\",\"params\": {{\"group_id\":{},\"card\":\"{}\"}}}}",group_id,group_name))}
    pub fn set_group_leave(&self,group_id:&i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_group_leave\",\"params\": {{\"message_id\":{}}}}}",group_id))}
    //pub fn set_group_special_title(&self){}
    pub fn set_friend_add_request<T:Display>(&self,flag:&T,approve:&bool,remark:&T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_friend_add_request\",\"params\": {{\"flag\":\"{}\",\"approve\":{},\"remark\":\"{}\"}}}}",flag,approve,remark))}
    pub fn set_group_add_request<T:Display>(&self,flag:&T,sub_type:&T,approve:&bool,reason:&T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_group_add_request\",\"params\": {{\"flag\":{},\"sub_type\":\"{}\",\"approve\":{},\"reason\":\"{}\"}}}}",flag,sub_type,approve,reason))}
    pub fn get_login_info(&self)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_login_info\"}}"))}
    pub fn get_stranger_info(&self,user_id:&i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_stranger_info\",\"params\": {{\"user_id\":{}}}}}",user_id))}
    pub fn get_friend_list(&self)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_friend_list\"}}"))}
    pub fn get_group_info(&self,group_id:&i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_group_info\",\"params\": {{\"group_id\":{}}}}}",group_id))}
    pub fn get_group_list(&self)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_group_list\"}}"))}
    pub fn get_group_member_info(&self,group_id:&i64,user_id:&i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_group_member_info\",\"params\": {{\"group_id\":{},\"user_id\":{}}}}}",group_id,user_id))}
    pub fn get_group_member_list(&self,group_id:&i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_group_member_list\",\"params\": {{\"group_id\":{}}}}}",group_id))}
    pub fn get_group_honor_info<T:Display>(&self,group_id:&i64,r#type:&T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_group_honor_info\",\"params\": {{\"group_id\":{},\"type\":{}}}}}",group_id,r#type))}
    pub fn get_cookies<T:Display>(&self,domain:&T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_cookies\",\"params\": {{\"domain\":\"{}\"}}}}",domain))}
    //pub fn get_csrf_token(&self){}
    //pub fn get_credentials(&self){}
    pub fn get_record<T:Display>(&self,file:&T,out_format:&T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_record\",\"params\": {{\"file\":\"{}\",\"out_format\":{}}}}}",file,out_format))}
    pub fn get_image<T:Display>(&self,file:&T)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_image\",\"params\": {{\"file\":\"{}\"}}}}",file))}
    pub fn can_send_image(&self)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"can_send_image\"}}"))}
    pub fn can_send_record(&self)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"can_send_record\"}}"))}
    pub fn get_status(&self)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_status\"}}"))}
    pub fn get_version_info(&self)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"get_version_info\"}}"))}
    pub fn set_restart(&self,delay:&i64)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"set_restart\",\"params\": {{\"delay\":{}}}}}",delay))}
    pub fn clean_cache(&self)->Result<(), ws::Error>{self.sender.send(format!("{{\"action\": \"clean_cache\"}}"))}
}

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
    subType:Number,
    pub url:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mface{//表情包
    emoji_id:String,
    emoji_package_id:String,
    key:String,
    pub summary:String,
    url:String
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
pub struct Rstatus{
    pub status:String,
    retcode:i32,
    data:Value,
    message:Value,
    pub wording:String,
    echo:Value
}



pub struct MessageBuilder{
    msg:Value
}

#[allow(dead_code)]
impl MessageBuilder {
    pub fn new()->MessageBuilder{MessageBuilder{msg:Value::Array(vec![])}}
    pub fn build(self)->Value{self.msg}
    pub fn text<T:ToString>(mut self,s:T)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "text","data": {"text": s.to_string()}}));self}
    pub fn face<T:ToString>(mut self,s:T)->MessageBuilder{self.msg.as_array_mut().unwrap().push(serde_json::json!({"type": "face","data": {"id": s.to_string()}}));self}
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

