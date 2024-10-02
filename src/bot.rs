//onebot v11
use std::{fmt::Display, thread::JoinHandle};
use crate::{error::BotError, event::*};



//TODO
//pub struct BotHttp{}
//pub struct BotRHttp{}
//pub struct BotRWebsocket{}

pub enum EventResolve<B:BotAPI> {
    LifecycleEvent(fn(&mut B,LifecycleEvent)),
    HeartbeatEvent(fn(&mut B,HeartbeatEvent)),
    GroupMsgEvent(fn(&mut B,GroupMsgEvent)),
    PrivateMsgEvent(fn(&mut B,PrivateMsgEvent))
}

pub(crate) struct Subscribes<B:Bot>{
    pub(crate) lifecycle_event:Option<fn(&mut B,LifecycleEvent)>,
    pub(crate) heartbeat_event:Option<fn(&mut B,HeartbeatEvent)>,
    pub(crate) group_msg_event:Option<fn(&mut B,GroupMsgEvent)>,
    pub(crate) private_msg_event:Option<fn(&mut B,PrivateMsgEvent)>
}

impl<B:Bot> Default for Subscribes<B> {
    fn default() -> Self {
        Self { lifecycle_event: Default::default(), heartbeat_event: Default::default(), group_msg_event: Default::default(), private_msg_event: Default::default() }
    }
}

/// 公开API
pub trait BotAPI where Self:Sized{
    /// 启动一个新线程运行bot
    fn run(self) -> JoinHandle<()>;
    /// 订阅一个事件
    /// 
    /// lifecycle_event 生命周期事件
    /// 
    /// heartbeat_event 心跳事件
    /// 
    /// group_msg_event 群聊消息事件
    /// 
    /// private_msg_event 私聊消息事件
    /// 
    fn subscribe(&mut self,factory:EventResolve<Self>)->();
    /// 发送群聊消息
    /// 
    /// 使用JSON格式
    fn send_group_msg<T:Display>(&mut self,group_id:&i64,s:T)->Result<(),BotError>;
    /// 发送私聊消息
    /// 
    /// 使用JSON格式
    fn send_private_msg<T:Display>(&mut self,id:&i64,s:T)->Result<(),BotError>;
    /// 获取登录状态
    fn get_status(&mut self) -> Result<EchoGetStatus, BotError>;
    /// 获取协议端版本信息
    fn get_version_info(&mut self)->Result<EchoGetVersionInfo, BotError>;
    /// 获取登录账号信息
    fn get_login_info(&mut self)->Result<EchoLoginInfo, BotError>;
    /// 撤回消息
    fn delete_msg(&mut self,msg_id:&i64)->Result<(),BotError>;
}

/// Bot底层功能
#[allow(async_fn_in_trait,unused)]
pub(crate) trait Bot where Self:BotAPI{
    //fn run_async(self) -> AbortHandle;
    /// 向协议端发送消息
    fn send(&mut self,string:&String)->Result<(),BotError>;
    /// 向协议端发送消息并接收返回消息
    fn send_with_recive(&mut self,string:&String)->Result<String,BotError>;
    /// 向协议端异步发送消息
    async fn send_async(&mut self,string:&String)->Result<(),BotError>;
    /// 向协议端异步发送消息并接收返回消息
    async fn send_with_recive_async(&mut self,string:&String)->Result<String,BotError>;
    /// 从协议端接收消息
    fn recv_msg(&self)->Result<String,String>;
}



/*
#[allow(dead_code)]
impl Bot {
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
}*/
pub(crate) fn send_private_msg<B:Bot,T:Display>(bot:&mut B,id:&i64,s:T)->Result<(), BotError>{
    bot.send_with_recive(&format!("{{\"action\": \"send_private_msg\",\"params\": {{\"user_id\":{},\"message\":{}}}}}",id,s))?;
    Ok(())
}

pub(crate) fn send_group_msg<B:Bot,T:Display>(bot:&mut B,id:&i64,s:T)->Result<(), BotError>{
    bot.send_with_recive(&format!("{{\"action\": \"send_group_msg\",\"params\": {{\"group_id\":{},\"message\":{}}}}}",id,s))?;
    Ok(())
}

pub(crate) fn delete_msg<B:Bot>(bot:&mut B,msg_id:&i64)->Result<(), BotError>{
    bot.send_with_recive(&format!("{{\"action\": \"delete_msg\",\"params\": {{\"message_id\":{}}}}}",msg_id))?;
    Ok(())
}

pub(crate) fn get_msg<B:Bot>(bot:&mut B,msg_id:&i64)->Result<(), BotError>{
    bot.send_with_recive(&format!("{{\"action\": \"get_msg\",\"params\": {{\"message_id\":{}}}}}",msg_id))?;
    Ok(())
}

pub(crate) fn get_status<T:Bot>(bot:&mut T)->Result<EchoGetStatus, BotError>{
    let res = bot.send_with_recive(&format!("{{\"action\": \"get_status\"}}"))?;
    Ok(serde_json::from_value::<EchoGetStatus>(serde_json::from_str::<EchoEvent>(res.as_str())?.data)?)
}

pub(crate) fn get_version_info<T:Bot>(bot:&mut T)->Result<EchoGetVersionInfo, BotError>{
    let res = bot.send_with_recive(&format!("{{\"action\": \"get_version_info\"}}"))?;
    Ok(serde_json::from_value::<EchoGetVersionInfo>(serde_json::from_str::<EchoEvent>(res.as_str())?.data)?)
}

pub(crate) fn get_login_info<T:Bot>(bot:&mut T)->Result<EchoLoginInfo, BotError>{
    let res = bot.send_with_recive(&format!("{{\"action\": \"get_login_info\"}}"))?;
    let echo_event = serde_json::from_str::<EchoEvent>(res.as_str())?;
    let login_info = serde_json::from_value::<EchoLoginInfo>(echo_event.data)?;
    Ok(login_info)
}
