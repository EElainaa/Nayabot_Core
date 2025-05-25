
use crate::{event::*, message::Message, protocol::Protocol};
use std::{sync::Arc, thread, time::Duration};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use parking_lot::{Condvar, Mutex};
use serde::de::DeserializeOwned;
use serde_json::Value;
use smol::{channel::{Receiver, Sender}, net::{TcpListener, TcpStream}};
use anyhow::{Context, Error, Result};
use async_tungstenite::{async_std::connect_async, tungstenite::{self, protocol::WebSocketConfig}, WebSocketStream};

#[derive(Debug)]
pub struct Bot{
    pub id:i64,
    sender:Sender<String>,
    echo_pool:Arc<DashMap<u16,(Arc<Mutex<Option<Event>>>,Arc<Condvar>)>>,
    event_rx:Receiver<Event>,
    time_limit:Duration
}

impl Clone for Bot {
    /// 克隆一个Bot对象
    fn clone(&self) -> Self {
        Self { id: self.id,
            sender: self.sender.clone(),
            echo_pool: self.echo_pool.clone(),
            event_rx:self.event_rx.clone(),
            time_limit: self.time_limit,
        }
    }
}

/// onebot v11 Bot基础功能
pub trait BaseBot {
    /// 发送私聊消息
    fn send_private_msg(&self,user_id:&i64,message:&Message)->Result<SendMsgEcho,Error>;
    /// 发送群聊消息
    fn send_group_msg(&self,group_id:&i64,message:&Message)->Result<SendMsgEcho,Error>;
    /// 撤回消息
    fn delete_msg<T:ToString>(&self,msg_id:T)->Result<(),Error>;
    // 获取一条消息
    fn get_msg(&self,msg_id:&i64)->Result<EchoGetMsg, Error>;
    /// 获取转发消息
    fn get_forward_msg(&self,id:&i64)->Result<Value, Error>;
    /// 给好友点赞
    fn send_like(&self,user_id:&i64,times:i16)->Result<(), Error>;
    /// 移出群成员
    fn set_group_kick(&self,group_id:&i64,user_id:&i64,reject_add_request:&bool)->Result<(), Error>;
    /// 禁言群成员
    fn set_group_ban(&self,group_id:&i64,user_id:&i64,duration:i64)->Result<(), Error>;
    /// 设置全员禁言
    fn set_group_whole_ban(&self,group_id:&i64,enable:&bool)->Result<(), Error>;
    /// 设置管理员
    fn set_group_admin(&self,group_id:&i64,user_id:&i64,enable:&bool)->Result<(), Error>;
    /// 设置群成员名片
    fn set_group_card(&self,group_id:&i64,user_id:&i64,card:String)->Result<(), Error>;
    /// 设置群名称
    fn set_group_name(&self,group_id:&i64,card:String)->Result<(), Error>;
    /// 退出群聊
    fn set_group_leave(&self,group_id:&i64,is_dismiss:bool)->Result<(), Error>;
    /// 设置特殊群头衔
    fn set_group_special_title(&self,group_id:&i64,user_id:&i64,special_title:String)->Result<(), Error>;
    /// 处理添加好友请求
    fn set_friend_add_request(&self,flag:String,approve:&bool,remark:String)->Result<(), Error>;
    /// 处理入群申请
    fn set_group_add_request(&self,flag:String,sub_type:String,approve:&bool,reason:String)->Result<(), Error>;
    /// 获取陌生人信息
    fn get_stranger_info(&self,user_id:&i64)->Result<Value, Error>;
    /// 获取登录账号信息
    fn get_login_info(&self)->Result<EchoLoginInfo, Error>;
    /// 获取好友列表
    fn get_friend_list(&self)->Result<Value, Error>;
    /// 获取群信息
    fn get_group_info(&self,group_id:&i64)->Result<Value, Error>;
    /// 获取群列表
    fn get_group_list(&self)->Result<Value, Error>;
    /// 获取群成员信息
    fn get_group_member_info(&self,group_id:&i64,user_id:&i64)->Result<Value, Error>;
    /// 获取群成员列表
    fn get_group_member_list(&self,group_id:&i64)->Result<Value, Error>;
    /// 获取群荣耀信息
    fn get_group_honor_info(&self,group_id:&i64,r#type:String)->Result<Value, Error>;
    /// 获取 Cookies
    fn get_cookies(&self,domain:String)->Result<Value, Error>;
    /// 获取 CSRF Token
    fn get_csrf_token(&self)->Result<Value, Error>;
    /// 获取 QQ 相关接口凭证
    fn get_credentials(&self,domain:String)->Result<Value, Error>;
    /// 获取语音
    /// 
    /// 需要配置ffmpeg
    fn get_record(&self,file:String,out_format:String)->Result<Value, Error>;
    /// 获取图片
    fn get_image(&self,file:String)->Result<Value, Error>;
    /// 检查是否可以发送图片
    fn can_send_image(&self)->Result<bool, Error>;
    /// 检查是否可以发送语音
    fn can_send_record(&self)->Result<bool, Error>;
    /// 清理缓存
    fn clean_cache(&self)->Result<(), Error>;
    /// 获取登录状态
    fn get_status(&self) -> Result<EchoGetStatus, Error>;
    /// 获取协议端版本信息
    fn get_version_info(&self)->Result<EchoGetVersionInfo, Error>;
    /// 重启协议端
    fn set_restart(&self,delay:i64)->Result<(), Error>;
}

#[allow(non_camel_case_types)]
/// napcat扩展功能
pub trait napcatBot {
    
}

#[allow(non_camel_case_types)]
/// go_cqhttp扩展功能
pub trait cqhttpBot {
    
}

impl BaseBot for Bot {
    fn send_group_msg(&self,group_id:&i64,message:&Message)->Result<SendMsgEcho,Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"send_group_msg\",\"params\": {{\"group_id\":{group_id},\"message\":{message}}},\"echo\":{random_echo}}}"));
        if event.is_ok() {to_event(event)}else{Err(Error::msg(format!("发送群聊消息失败")))}
    }
    
    fn send_private_msg(&self,user_id:&i64,message:&Message)->Result<SendMsgEcho,Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"send_private_msg\",\"params\": {{\"user_id\":{user_id},\"message\":{message}}},\"echo\":{random_echo}}}"));
        if event.is_ok() {to_event(event)}else{Err(Error::msg(format!("发送群聊消息失败")))}
    }
    
    fn delete_msg<T:ToString>(&self,message_id:T)->Result<(),Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"delete_msg\",\"params\": {{\"message_id\":{}}},\"echo\":{random_echo}}}",message_id.to_string()));
        if event.is_ok() {Ok(())}else{Err(Error::msg(format!("撤回消息[{}]失败",message_id.to_string())))}
    }
    
    fn get_msg(&self,message_id:&i64)->Result<EchoGetMsg, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_msg\",\"params\": {{\"message_id\":{message_id}}},\"echo\":{random_echo}}}"));
        to_event::<EchoGetMsg>(event)
    }
    
    fn get_forward_msg(&self,id:&i64)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_forward_msg\",\"params\": {{\"id\":{id}}},\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn send_like(&self,user_id:&i64,times:i16)->Result<(), Error> {
        self.send(format!("{{\"action\": \"send_like\",\"params\": {{\"user_id\":{user_id},\"times\":{times}}}}}"))
    }
    
    fn set_group_kick(&self,group_id:&i64,user_id:&i64,reject_add_request:&bool)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_group_kick\",\"params\": {{\"group_id\":{group_id}, \"user_id\":{user_id},\"reject_add_request\":{reject_add_request}}}}}"))
    }
    
    fn set_group_ban(&self,group_id:&i64,user_id:&i64,duration:i64)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_group_ban\",\"params\": {{\"group_id\":{group_id},\"user_id\":{user_id},\"duration\":{duration}}}}}"))
    }
    
    fn set_group_whole_ban(&self,group_id:&i64,enable:&bool)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_group_whole_ban\",\"params\": {{\"group_id\":{group_id},\"enable\":{enable}}}}}"))
    }
    
    fn set_group_admin(&self,group_id:&i64,user_id:&i64,enable:&bool)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_group_admin\",\"params\": {{\"group_id\":{group_id},\"user_id\":{user_id},\"enable\":{enable}}}}}"))
    }
    
    fn set_group_card(&self,group_id:&i64,user_id:&i64,card:String)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_group_card\",\"params\": {{\"group_id\":{group_id},\"user_id\":{user_id},\"card\":\"{card}\"}}}}"))
    }
    
    fn set_group_name(&self,group_id:&i64,card:String)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_group_name\",\"params\": {{\"group_id\":{group_id},\"card\":\"{card}\"}}}}"))
    }
    
    fn set_group_leave(&self,group_id:&i64,is_dismiss:bool)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_group_leave\",\"params\": {{\"group_id\":{group_id},\"is_dismiss\":{is_dismiss}}}}}"))
    }
    
    fn set_group_special_title(&self,group_id:&i64,user_id:&i64,special_title:String)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_group_special_title\",\"params\": {{\"group_id\":{group_id},\"user_id\":{user_id},\"special_title\":\"{special_title}\"}}}}"))
    }
    
    fn set_friend_add_request(&self,flag:String,approve:&bool,remark:String)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_friend_add_request\",\"params\": {{\"flag\":\"{flag}\",\"approve\":{approve},\"remark\":\"{remark}\"}}}}"))
    }
    
    fn set_group_add_request(&self,flag:String,sub_type:String,approve:&bool,reason:String)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_group_add_request\",\"params\": {{\"flag\":{flag},\"sub_type\":\"{sub_type}\",\"approve\":{approve},\"reason\":\"{reason}\"}}}}"))
    }
    
    fn get_stranger_info(&self,user_id:&i64)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_stranger_info\",\"params\": {{\"user_id\":{user_id}}},\"echo\":{random_echo}}}"));
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_login_info(&self)->Result<EchoLoginInfo, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_login_info\",\"echo\":{random_echo}}}"));
        to_event::<EchoLoginInfo>(event)
    }
    
    fn get_friend_list(&self)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_friend_list\",\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_group_info(&self,group_id:&i64)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_group_info\",\"params\": {{\"group_id\":{group_id}}},\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_group_list(&self)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_group_list\",\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_group_member_info(&self,group_id:&i64,user_id:&i64)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_group_member_info\",\"params\": {{\"group_id\":{group_id},\"user_id\":{user_id}}},\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_group_member_list(&self,group_id:&i64)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_group_member_list\",\"params\": {{\"group_id\":{group_id}}},\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_group_honor_info(&self,group_id:&i64,r#type:String)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_group_honor_info\",\"params\": {{\"group_id\":{group_id},\"type\":{type}}},\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_cookies(&self,domain:String)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_cookies\",\"params\": {{\"domain\":\"{domain}\"}},\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_csrf_token(&self)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_csrf_token\",\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_credentials(&self,domain:String)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_credentials\",\"params\": {{\"domain\":\"{domain}\"}},\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_record(&self,file:String,out_format:String)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_record\",\"params\": {{\"file\":\"{file}\",\"out_format\":{out_format}}},\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn get_image(&self,file:String)->Result<Value, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_image\",\"params\": {{\"file\":\"{file}\"}},\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn can_send_image(&self)->Result<bool, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"can_send_image\",\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap().get("yes").unwrap().as_bool().unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn can_send_record(&self)->Result<bool, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"can_send_record\",\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                return Ok(event.data.unwrap().get("yes").unwrap().as_bool().unwrap());
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }
    
    fn clean_cache(&self)->Result<(), Error> {
        self.send(format!("{{\"action\": \"clean_cache\"}}"))
    }
    
    fn get_status(&self) -> Result<EchoGetStatus, Error> {
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_status\",\"echo\":{random_echo}}}"));
        
        match event {
            Event::EchoEvent { event } => {
                let r = serde_json::from_value::<EchoGetStatus>(event.data.unwrap())?;
                return Ok(r);
            },
            Event::Error { msg } => return Err(Error::msg(msg)),
            _ => {return Err(Error::msg(format!("错误的事件类型:{:?}",event)))}
        }
    }

    fn get_version_info(&self) -> Result<EchoGetVersionInfo,Error>{
        let random_echo = random_echo(self);
        let event = push_echo(&self,random_echo,format!("{{\"action\": \"get_version_info\",\"echo\":{random_echo}}}"));
        to_event(event)
    }
    
    fn set_restart(&self,delay:i64)->Result<(), Error> {
        self.send(format!("{{\"action\": \"set_restart\",\"params\": {{\"delay\":{delay}}}}}"))
    }
}

fn to_event<T:DeserializeOwned>(event:Event)->Result<T,Error>{
    match event {
        Event::EchoEvent { event } => {
            let r = serde_json::from_value::<T>(event.data.unwrap()).unwrap();
            return Ok(r);
        },
        Event::Error { msg } => return Err(Error::msg(msg)),
        _ => {return Err(Error::msg(format!("错误的事件类型:{event:?}")))}
    }
}

fn random_echo(bot:&Bot)->u16{
    let mut random_echo = rand::random::<u16>();
    loop{
        if bot.echo_pool.contains_key(&random_echo){
            random_echo = rand::random::<u16>();
            continue;
        }else {
            break;
        }
    }
    random_echo
}

fn push_echo(bot:&Bot,echo:u16,msg:String)->Event{
    let event:Arc<Mutex<Option<Event>>> = Arc::new(Mutex::new(None));
    let cond = Arc::new(Condvar::new());
    let _ = bot.subscribe_echo(echo,event.clone(),cond.clone());
    let _ = bot.send(msg).unwrap();
    let mut event_lock = event.lock();
    let r = cond.wait_for(&mut event_lock,bot.time_limit);
    if r.timed_out(){
        Event::Error{msg:format!("等待响应超时...")}
    }else{
        event_lock.take().unwrap()
    }
    
}

impl Bot {
    /// 向协议端发送消息
    pub fn send(&self,msg:String)->Result<(),Error>{
        Ok(self.sender.try_send(msg)?)
    }

    pub fn set_time_limit(&mut self,time_limit:Duration){
        self.time_limit=time_limit
    }

    pub fn iter(&self)->BotIter{
        BotIter { event_rx: self.event_rx.clone() }
    }

    fn subscribe_echo(&self,echo:u16,event:Arc<Mutex<Option<Event>>>,cond:Arc<Condvar>)->Result<(),Error>{
        self.echo_pool.insert(echo, (event,cond));
        Ok(())
    }

    /// 创建Bot
    /// 
    /// 传入[crate::protocol::Protocol]来控制协议类型
    /// ```rust
    /// Bot::new(Protocol::WebSocket("127.0.0.1:3001".to_string(),"114514".to_string())
    /// ```
    pub fn new(protocol_type:Protocol)->Result<Bot>{
        match protocol_type {
            Protocol::WebSocket(addr,token) => {
                let addr = if addr.trim().starts_with("ws://"){addr}else{format!("ws://{}",addr.trim())};// 补全地址
                let b = tungstenite::ClientRequestBuilder::new(addr.parse()?);
                let builder = if !token.trim().eq(""){b.with_header("Authorization", format!("Bearer {token}"))}else{b};//添加鉴权请求头
                let (ws,_) = smol::block_on(async{
                    connect_async(builder).await
                })?;
                
                let (mut ws_writer,mut ws_reader) = ws.split();
                let echo_pool:Arc<DashMap<u16,(Arc<Mutex<Option<Event>>>,Arc<Condvar>)>> = Arc::new(DashMap::new());
                let echo_pool_clone = echo_pool.clone();

                //判断是否连接成功
                let msg = smol::block_on(async{ws_reader.next().await}).context("连接失败")?.context("连接失败")?;
                let status:Value = serde_json::from_str(msg.to_text()?)?;
                if status.get("status")==Some(&Value::from("failed")){
                    return Err(Error::msg(status.get("message").unwrap().to_string()))
                };
                let id = status.get("self_id").unwrap().as_i64().unwrap();

                // 网络消息发送/接收管道
                let (msg_sender,msg_recver) = smol::channel::unbounded::<String>();

                // 事件发送/接收管道
                let (event_sender,event_recver) = smol::channel::unbounded::<Event>();

                // 网络消息接收
                let _recv = thread::spawn(move||smol::block_on(async move{
                    while let Some(Ok(s)) = ws_reader.next().await {
                        match Event::from(&s.to_string()){
                            Ok(e)=>{
                                match &e {
                                    Event::EchoEvent { event } => {
                                        if echo_pool_clone.contains_key(&event.echo){
                                            if let Some((_,(event,cond))) = echo_pool_clone.remove(&event.echo){
                                                *event.lock() = Some(e);
                                                cond.notify_one();
                                                continue;
                                            };
                                        }
                                    },
                                    _=>{
                                        // 将事件推送至事件队列
                                        let _ = event_sender.send(e).await;
                                    }
                                }
                            }
                            Err(e)=>{
                                let _ = event_sender.send(Event::Error {msg: e.to_string()}).await;
                            }
                        }
                    }
                }));

                // 网络消息发送
                let _send = thread::spawn(move||smol::block_on(async move{
                    loop {
                        if let Ok(msg) = msg_recver.recv().await{
                            let _ = ws_writer.send(tungstenite::Message::Text(msg.into())).await;
                            let _ = ws_writer.flush().await;
                        };
                    }
                }));

                //包装Bot对象
                return Ok(Bot{
                    id,
                    sender:msg_sender,
                    echo_pool,
                    event_rx:event_recver,
                    time_limit:Duration::from_secs(30)
                })
            }
            Protocol::ReserverWebSocket(addr,token) => {
                let listener = smol::block_on(async{
                    TcpListener::bind(addr).await
                })?;
                let (stream,_) = smol::block_on(async{
                    listener.accept().await
                })?;
                let ws_stream = smol::block_on(async{
                    async_tungstenite::accept_async(stream).await
                })?;
                let (mut ws_writer,mut ws_reader) = ws_stream.split();

                todo!()
            }
            Protocol::Http(addr,token) => {
                let addr = if addr.trim().starts_with("http://"){addr}else{format!("http://{}",addr.trim())};
                todo!()
            }
            Protocol::ReserverHttp(addr,token) => {todo!()}
        }
    }
}

pub struct BotIter{
    event_rx:Receiver<Event>
}

impl Iterator for BotIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.event_rx.recv_blocking().unwrap())
    }
}
