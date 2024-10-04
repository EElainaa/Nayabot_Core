//onebot v11
use std::{fmt::Display, thread::JoinHandle};
use crate::event::*;
use anyhow::{Context, Error, Result};

pub enum EventResolve<B:BotAPI> {
    LifecycleEvent(fn(&mut B,LifecycleEvent)),
    HeartbeatEvent(fn(&mut B,HeartbeatEvent)),
    GroupMsgEvent(fn(&mut B,GroupMsgEvent)),
    PrivateMsgEvent(fn(&mut B,PrivateMsgEvent)),
    GroupRecall(fn(&mut B,GroupRecall))
}

pub(crate) struct Subscribes<B:Bot>{
    pub(crate) lifecycle_event:Option<fn(&mut B,LifecycleEvent)>,
    pub(crate) heartbeat_event:Option<fn(&mut B,HeartbeatEvent)>,
    pub(crate) group_msg_event:Option<fn(&mut B,GroupMsgEvent)>,
    pub(crate) private_msg_event:Option<fn(&mut B,PrivateMsgEvent)>,
    pub(crate) group_recall:Option<fn(&mut B,GroupRecall)>
}

impl<B:Bot> Default for Subscribes<B> {
    fn default() -> Self {
        Self { 
            lifecycle_event: Default::default(), 
            heartbeat_event: Default::default(), 
            group_msg_event: Default::default(), 
            private_msg_event: Default::default(), 
            group_recall: Default::default() 
        }
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
    /// group_recall 群聊消息撤回事件
    /// 
    /// private_msg_event 私聊消息事件
    /// 
    fn subscribe(&mut self,factory:EventResolve<Self>)->();
    /// 发送私聊消息
    /// 
    /// 使用JSON格式
    fn send_private_msg<T:Display>(&mut self,id:&i64,s:T)->Result<EchoStatus,Error>;
    /// 发送群聊消息
    /// 
    /// 使用JSON格式
    fn send_group_msg<T:Display>(&mut self,group_id:&i64,s:T)->Result<EchoStatus,Error>;
    /// 撤回消息
    fn delete_msg(&mut self,msg_id:&i64)->Result<EchoStatus,Error>;
    // 获取一条消息
    fn get_msg(&mut self,msg_id:&i64)->Result<EchoEvent, Error>;
    /// 获取转发消息
    fn get_forward_msg(&mut self,id:&i64)->Result<EchoEvent, Error>;
    /// 给好友点赞
    fn send_like(&mut self,user_id:&i64,times:i16)->Result<EchoStatus, Error>;
    /// 移出群成员
    fn set_group_kick(&mut self,group_id:&i64,user_id:&i64,reject_add_request:&bool)->Result<EchoStatus, Error>;
    /// 禁言群成员
    fn set_group_ban(&mut self,group_id:&i64,user_id:&i64,duration:i64)->Result<EchoStatus, Error>;
    /// 设置全员禁言
    fn set_group_whole_ban(&mut self,group_id:&i64,enable:&bool)->Result<EchoStatus, Error>;
    /// 设置管理员
    fn set_group_admin(&mut self,group_id:&i64,user_id:&i64,enable:&bool)->Result<EchoStatus, Error>;
    /// 设置群成员名片
    fn set_group_card<T:Display>(&mut self,group_id:&i64,user_id:&i64,card:&T)->Result<EchoStatus, Error>;
    /// 设置群名称
    fn set_group_name<T:Display>(&mut self,group_id:&i64,user_id:&i64,group_name:&T)->Result<EchoStatus, Error>;
    /// 退出群聊
    fn set_group_leave(&mut self,group_id:&i64)->Result<EchoStatus, Error>;
    /// 设置特殊群头衔
    fn set_group_special_title <T:Display>(&mut self,group_id:&i64,user_id:&i64,special_title:&T)->Result<EchoStatus, Error>;
    /// 处理添加好友请求
    fn set_friend_add_request<T:Display>(&mut self,flag:&T,approve:&bool,remark:&T)->Result<EchoStatus, Error>;
    /// 处理入群申请
    fn set_group_add_request<T:Display>(&mut self,flag:&T,sub_type:&T,approve:&bool,reason:&T)->Result<EchoStatus, Error>;
    /// 获取陌生人信息
    fn get_stranger_info(&mut self,user_id:&i64)->Result<EchoEvent, Error>;
    /// 获取登录账号信息
    fn get_login_info(&mut self)->Result<EchoLoginInfo, Error>;
    /// 获取好友列表
    fn get_friend_list(&mut self)->Result<EchoEvent, Error>;
    /// 获取群信息
    fn get_group_info(&mut self,group_id:&i64)->Result<EchoEvent, Error>;
    /// 获取群列表
    fn get_group_list(&mut self)->Result<EchoEvent, Error>;
    /// 获取群成员信息
    fn get_group_member_info(&mut self,group_id:&i64,user_id:&i64)->Result<EchoEvent, Error>;
    /// 获取群成员列表
    fn get_group_member_list(&mut self,group_id:&i64)->Result<EchoEvent, Error>;
    /// 获取群荣耀信息
    fn get_group_honor_info(&mut self,group_id:&i64,r#type:String)->Result<EchoEvent, Error>;
    /// 获取 Cookies
    fn get_cookies<T:Display>(&mut self,domain:&T)->Result<EchoEvent, Error>;
    /// 获取 CSRF Token
    fn get_csrf_token(&mut self)->Result<EchoEvent, Error>;
    /// 获取 QQ 相关接口凭证
    fn get_credentials<T:Display>(&mut self,domain:&T)->Result<EchoEvent, Error>;
    /// 获取语音
    /// 
    /// 需要配置ffmpeg
    fn get_record<T:Display>(&mut self,file:&T,out_format:&T)->Result<EchoEvent, Error>;
    /// 获取图片
    fn get_image<T:Display>(&mut self,file:&T)->Result<EchoEvent, Error>;
    /// 检查是否可以发送图片
    fn can_send_image(&mut self)->Result<EchoEvent, Error>;
    /// 检查是否可以发送语音
    fn can_send_record(&mut self)->Result<EchoEvent, Error>;
    /// 清理缓存
    fn clean_cache(&mut self)->Result<EchoStatus, Error>;
    /// 获取登录状态
    fn get_status(&mut self) -> Result<EchoGetStatus, Error>;
    /// 获取协议端版本信息
    fn get_version_info(&mut self)->Result<EchoGetVersionInfo, Error>;
    /// 重启协议端
    fn set_restart(&mut self,delay:i64)->Result<EchoStatus, Error>;
}

/// Bot底层功能
#[allow(async_fn_in_trait,unused)]
pub(crate) trait Bot where Self:BotAPI{
    //fn run_async(self) -> AbortHandle;
    /// 向协议端发送消息
    fn send(&mut self,string:&String)->Result<(),Error>;
    /// 向协议端发送消息并接收返回消息
    fn send_with_recive(&mut self,string:&String)->Result<String,Error>;
    /// 向协议端异步发送消息
    async fn send_async(&mut self,string:&String)->Result<(),Error>;
    /// 向协议端异步发送消息并接收返回消息
    async fn send_with_recive_async(&mut self,string:&String)->Result<String,Error>;
    /// 从协议端接收消息
    fn recv_msg(&self)->Result<String,Error>;
}

pub(crate) fn send_private_msg<B:Bot,T:Display>(bot:&mut B,id:&i64,s:T)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"send_private_msg\",\"params\": {{\"user_id\":{},\"message\":{}}}}}",id,s)).with_context(||format!("向 [id = {}] 发送私聊消息失败:发送失败",id))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("向 [id = {}] 发送私聊消息失败:解析失败",id))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("向 [id = {}] 发送私聊消息失败",id)))
}
pub(crate) fn send_group_msg<B:Bot,T:Display>(bot:&mut B,id:&i64,s:T)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"send_group_msg\",\"params\": {{\"group_id\":{},\"message\":{}}}}}",id,s)).with_context(||format!("向 [id = {}] 发送群聊消息失败:发送失败",id))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("向 [id = {}] 发送群聊消息失败:解析失败",id))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("向 [id = {}] 发送群聊消息失败",id)))
}
pub(crate) fn delete_msg<B:Bot>(bot:&mut B,msg_id:&i64)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"delete_msg\",\"params\": {{\"message_id\":{}}}}}",msg_id)).with_context(||format!("撤回消息 [msg_id = {}] 失败:发送失败",msg_id))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("撤回消息 [msg_id = {}] 失败:解析失败",msg_id))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("撤回消息 [msg_id = {}] 失败",msg_id)))
}
pub(crate) fn get_msg<B:Bot>(bot:&mut B,msg_id:&i64)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_msg\",\"params\": {{\"message_id\":{}}}}}",msg_id)).with_context(||format!("获取消息 [msg_id = {}] 失败:发送失败",msg_id))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取消息 [msg_id = {}] 失败:解析失败",msg_id))?)
}
pub(crate) fn get_forward_msg<B:Bot>(bot:&mut B,id:&i64)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_forward_msg\",\"params\": {{\"id\":{}}}}}",id)).with_context(||format!("获取合并转发消息 [id = {}] 失败:发送失败",id))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取合并转发消息 [id = {}] 失败:解析失败",id))?)
}
pub(crate) fn send_like<B:Bot>(bot:&mut B,user_id:&i64,times:i16)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"send_like\",\"params\": {{\"user_id\":{},\"times\":{}}}}}",user_id,times)).with_context(||"点赞失败:发送失败")?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||"点赞失败:解析失败")?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg("点赞失败"))
}
pub(crate) fn set_group_kick<B:Bot>(bot:&mut B,group_id:&i64,user_id:&i64,reject_add_request:&bool)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_group_kick\",\"params\": {{\"group_id\":{}, \"user_id\":{},\"reject_add_request\":{}}}}}",group_id,user_id,reject_add_request)).with_context(||format!("移出群成员 [user_id = {}]失败:发送失败",user_id))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("移出群成员 [user_id = {}]失败:解析失败",user_id))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("移出群成员 [user_id = {}]失败",user_id)))
}
pub(crate) fn set_group_ban<B:Bot>(bot:&mut B,group_id:&i64,user_id:&i64,duration:i64)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_group_ban\",\"params\": {{\"group_id\":{},\"user_id\":{},\"duration\":{}}}}}",group_id,user_id,duration)).with_context(||format!("禁言群成员 [group_id = {},user_id = {}]失败:发送失败",group_id,user_id))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("禁言群成员 [group_id = {},user_id = {}]失败:解析失败",group_id,user_id))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("禁言群成员 [group_id = {},user_id = {}]失败",group_id,user_id)))
}
pub(crate) fn set_group_whole_ban<B:Bot>(bot:&mut B,group_id:&i64,enable:&bool)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_group_whole_ban\",\"params\": {{\"group_id\":{},\"enable\":{}}}}}",group_id,enable)).with_context(||format!("设置全群禁言 [group_id = {},enable = {}]失败:发送失败",group_id,enable))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("设置全群禁言 [group_id = {},enable = {}]失败:解析失败",group_id,enable))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("设置全群禁言 [group_id = {},enable = {}]失败",group_id,enable)))
}
pub(crate) fn set_group_admin<B:Bot>(bot:&mut B,group_id:&i64,user_id:&i64,enable:&bool)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_group_admin\",\"params\": {{\"group_id\":{},\"user_id\":{},\"enable\":{}}}}}",group_id,user_id,enable)).with_context(||format!("设置管理员 [group_id = {},user_id = {},enable = {}]失败:发送失败",group_id,user_id,enable))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("设置管理员 [group_id = {},user_id = {},enable = {}]失败:解析失败",group_id,user_id,enable))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("设置管理员 [group_id = {},user_id = {},enable = {}]失败",group_id,user_id,enable)))
}
pub(crate) fn set_group_card<B:Bot,T:Display>(bot:&mut B,group_id:&i64,user_id:&i64,card:&T)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_group_card\",\"params\": {{\"group_id\":{},\"user_id\":{},\"card\":\"{}\"}}}}",group_id,user_id,card)).with_context(||format!("设置群成员名片 [group_id = {},user_id = {},card = {}]失败:发送失败",group_id,user_id,card))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("设置群成员名片 [group_id = {},user_id = {},card = {}]失败:解析失败",group_id,user_id,card))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("设置群成员名片 [group_id = {},user_id = {},card = {}]失败",group_id,user_id,card)))
}
pub(crate) fn set_group_name<B:Bot,T:Display>(bot:&mut B,group_id:&i64,user_id:&i64,group_name:&T)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_group_name\",\"params\": {{\"group_id\":{},\"card\":\"{}\"}}}}",group_id,group_name)).with_context(||format!("设置群名称 [group_id = {},user_id = {},card = {}]失败:发送失败",group_id,user_id,group_name))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("设置群名称 [group_id = {},user_id = {},card = {}]失败:解析失败",group_id,user_id,group_name))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("设置群名称 [group_id = {},user_id = {},card = {}]失败",group_id,user_id,group_name)))
}
pub(crate) fn set_group_leave<B:Bot>(bot:&mut B,group_id:&i64)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_group_leave\",\"params\": {{\"message_id\":{}}}}}",group_id)).with_context(||format!("退出群 [group_id = {}]失败:发送失败",group_id))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("退出群 [group_id = {}]失败:解析失败",group_id))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("退出群 [group_id = {}]失败",group_id)))
}
pub(crate) fn set_group_special_title <B:Bot,T:Display>(bot:&mut B,group_id:&i64,user_id:&i64,special_title:&T)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_group_special_title\",\"params\": {{\"group_id\":{},\"user_id\":{},\"special_title\":\"{}\"}}}}",group_id,user_id,special_title)).with_context(||format!("设置群成员头衔 [group_id = {},user_id = {},special_title = {}]失败:发送失败",group_id,user_id,special_title))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("设置群成员头衔 [group_id = {},user_id = {},special_title = {}]失败:解析失败",group_id,user_id,special_title))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("设置群成员头衔 [group_id = {},user_id = {},special_title = {}]失败",group_id,user_id,special_title)))
}
pub(crate) fn set_friend_add_request<B:Bot,T:Display>(bot:&mut B,flag:&T,approve:&bool,remark:&T)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_friend_add_request\",\"params\": {{\"flag\":\"{}\",\"approve\":{},\"remark\":\"{}\"}}}}",flag,approve,remark)).with_context(||format!("处理好友请求 [flag = {},approve = {}]失败:发送失败",flag,approve))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("处理好友请求 [flag = {},approve = {}]失败:解析失败",flag,approve))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("处理好友请求 [flag = {},approve = {}]失败",flag,approve)))
}
pub(crate) fn set_group_add_request<B:Bot,T:Display>(bot:&mut B,flag:&T,sub_type:&T,approve:&bool,reason:&T)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_group_add_request\",\"params\": {{\"flag\":{},\"sub_type\":\"{}\",\"approve\":{},\"reason\":\"{}\"}}}}",flag,sub_type,approve,reason)).with_context(||format!("处理入群申请 [flag = {},approve = {}]失败:发送失败",flag,approve))?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||format!("处理入群申请 [flag = {},approve = {}]失败:解析失败",flag,approve))?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg(format!("处理入群申请 [flag = {},approve = {}]失败",flag,approve)))
}
pub(crate) fn get_login_info<B:Bot>(bot:&mut B)->Result<EchoLoginInfo, Error>{
    let res = bot.send_with_recive(&format!("{{\"action\": \"get_login_info\"}}")).with_context(||"获取登录信息失败:发送失败")?;
    let echo_event = serde_json::from_str::<EchoEvent>(res.as_str()).with_context(||"获取登录信息失败:解析失败")?;
    let login_info = serde_json::from_value::<EchoLoginInfo>(echo_event.data).with_context(||"获取登录信息失败:解析失败")?;
    Ok(login_info)
}
pub(crate) fn get_stranger_info<B:Bot>(bot:&mut B,user_id:&i64)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_stranger_info\",\"params\": {{\"user_id\":{}}}}}",user_id)).with_context(||format!("获取陌生人信息 [user_id = {}] 失败:发送失败",user_id))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取陌生人信息 [user_id = {}] 失败:解析失败",user_id))?)
}
pub(crate) fn get_friend_list<B:Bot>(bot:&mut B)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_friend_list\"}}")).with_context(||"获取好友列表失败:发送失败")?;
    Ok(serde_json::from_str(&s).with_context(||"获取好友列表失败:解析失败")?)
}
pub(crate) fn get_group_info<B:Bot>(bot:&mut B,group_id:&i64)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_group_info\",\"params\": {{\"group_id\":{}}}}}",group_id)).with_context(||format!("获取群信息 [group_id = {}] 失败:发送失败",group_id))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取群信息 [group_id = {}] 失败:解析失败",group_id))?)
}
pub(crate) fn get_group_list<B:Bot>(bot:&mut B)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_group_list\"}}")).with_context(||"获取群列表失败:发送失败")?;
    Ok(serde_json::from_str(&s).with_context(||"获取群列表失败:解析失败")?)
}
pub(crate) fn get_group_member_info<B:Bot>(bot:&mut B,group_id:&i64,user_id:&i64)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_group_member_info\",\"params\": {{\"group_id\":{},\"user_id\":{}}}}}",group_id,user_id)).with_context(||format!("获取群成员信息 [group_id = {},user_id = {}] 失败:发送失败",group_id,user_id))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取群成员信息 [group_id = {},user_id = {}] 失败:解析失败",group_id,user_id))?)
}
pub(crate) fn get_group_member_list<B:Bot>(bot:&mut B,group_id:&i64)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_group_member_list\",\"params\": {{\"group_id\":{}}}}}",group_id)).with_context(||format!("获取群成员列表 [group_id = {}] 失败:发送失败",group_id))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取群成员列表 [group_id = {}] 失败:解析失败",group_id))?)
}
pub(crate) fn get_group_honor_info<B:Bot>(bot:&mut B,group_id:&i64,r#type:String)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_group_honor_info\",\"params\": {{\"group_id\":{},\"type\":{}}}}}",group_id,r#type)).with_context(||format!("获取群荣耀信息 [group_id = {},type = {}] 失败:发送失败",group_id,r#type))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取群荣耀信息 [group_id = {},type = {}] 失败:解析失败",group_id,r#type))?)
}
pub(crate) fn get_cookies<B:Bot,T:Display>(bot:&mut B,domain:&T)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_cookies\",\"params\": {{\"domain\":\"{}\"}}}}",domain)).with_context(||format!("获取Cookies [domain = {}] 失败:发送失败",domain))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取Cookies [domain = {}] 失败:解析失败",domain))?)
}
pub(crate) fn get_csrf_token<B:Bot>(bot:&mut B)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_csrf_token\"}}")).with_context(||format!("获取CSRF Token失败:发送失败"))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取CSRF Token失败:解析失败"))?)
}
pub(crate) fn get_credentials<B:Bot,T:Display>(bot:&mut B,domain:&T)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_credentials\",\"params\": {{\"domain\":\"{}\"}}}}",domain)).with_context(||format!("获取 QQ 相关接口凭证 [domain = {}] 失败:发送失败",domain))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取 QQ 相关接口凭证 [domain = {}] 失败:解析失败",domain))?)
}
pub(crate) fn get_record<B:Bot,T:Display>(bot:&mut B,file:&T,out_format:&T)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_record\",\"params\": {{\"file\":\"{}\",\"out_format\":{}}}}}",file,out_format)).with_context(||format!("获取语音 [file = {}] 失败:发送失败",file))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取语音 [file = {}] 失败:解析失败",file))?)
}
pub(crate) fn get_image<B:Bot,T:Display>(bot:&mut B,file:&T)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"get_image\",\"params\": {{\"file\":\"{}\"}}}}",file)).with_context(||format!("获取图片 [file = {}] 失败:发送失败",file))?;
    Ok(serde_json::from_str(&s).with_context(||format!("获取图片 [file = {}] 失败:解析失败",file))?)
}
pub(crate) fn can_send_image<B:Bot>(bot:&mut B)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"can_send_image\"}}")).with_context(||"检查是否可以发送图片失败:发送失败")?;
    Ok(serde_json::from_str(&s).with_context(||"检查是否可以发送图片失败:解析失败")?)
}
pub(crate) fn can_send_record<B:Bot>(bot:&mut B)->Result<EchoEvent, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"can_send_record\"}}")).with_context(||"检查是否可以发送语音失败:发送失败")?;
    Ok(serde_json::from_str(&s).with_context(||"检查是否可以发送语音失败:发送失败")?)
}
pub(crate) fn get_status<B:Bot>(bot:&mut B)->Result<EchoGetStatus, Error>{
    let res = bot.send_with_recive(&format!("{{\"action\": \"get_status\"}}")).with_context(||"获取登录状态:发送失败")?;
    Ok(serde_json::from_value::<EchoGetStatus>(serde_json::from_str::<EchoEvent>(res.as_str())?.data).with_context(||"获取登录状态:解析失败")?)
}
pub(crate) fn get_version_info<B:Bot>(bot:&mut B)->Result<EchoGetVersionInfo, Error>{
    let res = bot.send_with_recive(&format!("{{\"action\": \"get_version_info\"}}")).with_context(||"获取协议端版本信息:发送失败")?;
    Ok(serde_json::from_value::<EchoGetVersionInfo>(serde_json::from_str::<EchoEvent>(res.as_str())?.data).with_context(||"获取协议端版本信息:解析失败")?)
}
pub(crate) fn clean_cache<B:Bot>(bot:&mut B)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"clean_cache\"}}")).with_context(||"清理缓存失败:发送失败")?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||"清理缓存失败:解析失败")?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg("清理缓存失败"))
}
pub(crate) fn set_restart<B:Bot>(bot:&mut B,delay:i64)->Result<EchoStatus, Error>{
    let s = bot.send_with_recive(&format!("{{\"action\": \"set_restart\",\"params\": {{\"delay\":{}}}}}",delay)).with_context(||"重启协议端失败:解析失败")?;
    let status:EchoStatus = serde_json::from_str(&s).with_context(||"重启协议端失败:解析失败")?;
    if status.status=="ok"{
        return Ok(status)
    }
    Err(Error::msg("重启协议端失败"))
}
