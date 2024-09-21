//onebot v11
use std::{collections::HashMap, fmt::Display, net::TcpStream, sync::{Arc, Mutex}, thread::{self, JoinHandle, Thread}};
use crate::{event::*, funs::{printerr, printinf, printwrm}};
use serde_json::Value;
use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};

type WS=Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>;

//TODO
//pub struct BotHttp{}
//pub struct BotRHttp{}
//pub struct BotRWebsocket{}

pub trait EventResolve {
    fn run(self,event:Event);
}

pub struct BotWebsocket{
    url:String,
    id:i64,
    ws:WS,
    subscribes:HashMap<String,fn(Event)>
}

impl BotWebsocket{
    ///通过正向ws连接协议端
    pub fn new<T:Display>(url:T)->Result<BotWebsocket,String>{
        match tungstenite::client::connect(url.to_string()) {
            Ok((mut ws,_resp)) =>{
                match ws.read() {
                    Ok(s)=>{
                        let lifecycle_event:LifecycleEvent=serde_json::from_str(s.to_string().as_str()).unwrap();
                        return Ok(BotWebsocket{
                            url:url.to_string(),
                            id: lifecycle_event.self_id,
                            ws: Arc::new(Mutex::new(ws)),
                            subscribes:HashMap::new()
                        })
                    }
                    Err(err)=>{
                        return Err(err.to_string())
                    }
                }
            }
            Err(err) => return Err(err.to_string()),
        }
    }
    pub fn get_ws(&self) -> WS {
        self.ws.clone()
    }
    pub fn get_url(&self) -> String{
        self.url.clone()
    }
    pub fn get_id(&self) -> i64{
        self.id
    }
}

#[allow(async_fn_in_trait)]
pub trait Bot{
    /// 启动一个新线程运行bot
    fn run(self) -> JoinHandle<()>;
    //fn run_async(self) -> AbortHandle;
    /// 发送消息
    fn send(&mut self,string:&String)->Result<(),String>;
    /// 发送消息并接收返回消息
    fn send_with_recive(&mut self,string:&String)->Result<String,String>;
    /// 异步发送消息
    async fn send_async(&mut self,string:&String)->Result<(),String>;
    /// 异步发送消息并接收返回消息
    async fn send_with_recive_async(&mut self,string:&String)->Result<String,String>;
    /// 接收消息
    fn recv_msg(&self)->Result<String,String>;
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
    fn subscribe<T:ToString>(&mut self,t:T,factory:fn(Event))->();
}

impl Bot for BotWebsocket {

    fn run(self) -> JoinHandle<()> {
        let bot = self;
        thread::spawn(move ||{
            loop{
                match bot.recv_msg(){
                    Ok(str) => {
                        let d = serde_json::from_str::<Value>(&str.as_str()).unwrap();
                        match d.get("post_type").unwrap().to_string().as_str() {
                            "\"meta_event\""=>{
                                if let Some(f) = bot.subscribes.get("meta_event") {
                                    //f()
                                }else {
                                    match d.get("meta_event_type").unwrap().to_string().as_str(){
                                        "\"heartbeat\""=>{
                                            if let Some(f) = bot.subscribes.get("heartbeat_event") {
                                                let event:HeartbeatEvent = serde_json::from_str(&str).unwrap();
                                                f(Event::HeartbeatEvent { event })
                                            }
                                        }
                                        _=>{printwrm(format!("未知meta_event事件 {}",str))}
                                    }
                                }
                            }
                            "\"message\""=>{
                                if let Some(f) = bot.subscribes.get("message") {
                                    //f()
                                }else {
                                    match d.get("message_type").unwrap().to_string().as_str(){
                                        "\"group\""=>{
                                            if let Some(f) = bot.subscribes.get("group_msg_event") {
                                                let event:GroupMsgEvent = serde_json::from_str(&str).unwrap();
                                                f(Event::GroupMsgEvent { event })
                                            }
                                        }
                                        "\"private\""=>{
                                            if let Some(f) = bot.subscribes.get("private_msg_event") {
                                                let event:GroupMsgEvent = serde_json::from_str(&str).unwrap();
                                                f(Event::GroupMsgEvent { event })
                                            }
                                        }
                                        _=>{printwrm(format!("未知meta_event事件 {}",str))}
                                    }
                                }
                            }
                            _=>{printwrm(format!("未知事件 {}",str))}
                        }
                    },
                    Err(err) => printerr(err),
                }
            }
        })
    }

    fn send(&mut self,string:&String)->Result<(),String> {
        match self.get_ws().lock().unwrap().send(Message::text(string)){
            Ok(_)=>{
                printinf(format!("Send:{}",&string));
                Ok(())
            }
            Err(err)=>{Err(err.to_string())}
        }
    }
    
    fn send_with_recive(&mut self,string:&String)->Result<String,String> {
        match self.send(string){
            Ok(_)=>{
                printinf(format!("Send:{}",&string));
                match self.ws.lock().unwrap().read() {
                    Ok(s) => {
                        Ok(s.to_string())
                    }
                    Err(err) => return Err(err.to_string())
                }
            }
            Err(err)=>{Err(err.to_string())}
        }
    }
    
    async fn send_async(&mut self,string:&String)->Result<(),String> {
        match self.ws.lock().unwrap().send(Message::text(string)){
            Ok(_)=>{Ok(())}
            Err(err)=>{Err(err.to_string())}
        }
    }
    
    async fn send_with_recive_async(&mut self,string:&String)->Result<String,String> {
        match self.ws.lock().unwrap().send(Message::text(string)){
            Ok(_)=>{
                match self.ws.lock().unwrap().read() {
                    Ok(s) => {
                        printinf(&s);
                        Ok(s.to_string())
                    }
                    Err(err) => return Err(err.to_string())
                }
            }
            Err(err)=>{Err(err.to_string())}
        }
    }

    fn recv_msg(&self)->Result<String,String>{
        Ok(self.get_ws().lock().unwrap().read().unwrap().to_string())
    }
    fn subscribe<T:ToString>(&mut self,t:T,factory:fn(Event))->() {
        self.subscribes.insert(t.to_string(), factory);
    }
}
impl BotWebsocket {
    ///发送私聊消息
    pub fn send_private_msg<T:Display>(&mut self,id:&i64,s:T)->Result<(),String>{
        send_private_msg(self, id, s)
    }
    ///获取登录状态
    pub fn get_status(&mut self)->Result<EchoGetStatus, String>{
        get_status(self)
    }
    ///获取协议端版本信息
    pub fn get_version_info(&mut self)->Result<EchoGetVersionInfo, String>{
        get_version_info(self)
    }
    ///获取登录账号信息
    pub fn get_login_info(&mut self)->Result<EchoLoginInfo, String>{
        get_login_info(self)
    }
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
fn send_private_msg<B:Bot,T:Display>(bot:&mut B,id:&i64,s:T)->Result<(), String>{
    bot.send(&format!("{{\"action\": \"send_private_msg\",\"params\": {{\"user_id\":{},\"message\":{}}}}}",id,s))
}

fn send_group_msg<B:Bot,T:Display>(bot:&mut B,id:&i64,s:T)->Result<(), String>{
    bot.send(&format!("{{\"action\": \"send_group_msg\",\"params\": {{\"group_id\":{},\"message\":{}}}}}",id,s))
}

fn delete_msg<B:Bot>(bot:&mut B,msg_id:&i64)->Result<(), String>{
    bot.send(&format!("{{\"action\": \"delete_msg\",\"params\": {{\"message_id\":{}}}}}",msg_id))
}

fn get_msg<B:Bot>(bot:&mut B,msg_id:&i64)->Result<(), String>{
    bot.send(&format!("{{\"action\": \"get_msg\",\"params\": {{\"message_id\":{}}}}}",msg_id))
}

fn get_status<T:Bot>(bot:&mut T)->Result<EchoGetStatus, String>{
    match bot.send_with_recive(&format!("{{\"action\": \"get_status\"}}")){
        Ok(res)=>{
            Ok(serde_json::from_value::<EchoGetStatus>(serde_json::from_str::<EchoEvent>(res.as_str()).unwrap().data).unwrap())
        }
        Err(err)=>{Err(err)}
    }
}

fn get_version_info<T:Bot>(bot:&mut T)->Result<EchoGetVersionInfo, String>{
    match bot.send_with_recive(&format!("{{\"action\": \"get_version_info\"}}")){
        Ok(res)=>{
            Ok(serde_json::from_value::<EchoGetVersionInfo>(serde_json::from_str::<EchoEvent>(res.as_str()).unwrap().data).unwrap())
        }
        Err(err)=>{Err(err)}
    }
}

fn get_login_info<T:Bot>(bot:&mut T)->Result<EchoLoginInfo, String>{
    match bot.send_with_recive(&format!("{{\"action\": \"get_login_info\"}}")){
        Ok(res)=>{
            Ok(serde_json::from_value::<EchoLoginInfo>(serde_json::from_str::<EchoEvent>(res.as_str()).unwrap().data).unwrap())
        }
        Err(err)=>{Err(err)}
    }
}

