use std::{fmt::Display, net::TcpStream, str::FromStr, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

use tungstenite::{handshake::{self, machine::HandshakeMachine}, http::Uri, protocol::WebSocketConfig, stream::MaybeTlsStream, Message, WebSocket};

use crate::{bot::*, error::BotError, event::*, funs::*};

type WS=Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>;

///正向WebSocket连接
pub struct BotWebsocket{
    url:String,
    id:i64,
    ws:WS,
    subscribes:Subscribes<Self>
}

impl BotWebsocket{
    ///通过正向ws连接协议端
    pub fn new<T:Display>(url:T,token:T)->Result<BotWebsocket,BotError>{
        let uri = Uri::from_str(url.to_string().as_str())?;
        let builder = tungstenite::ClientRequestBuilder::new(uri).with_header("Authorization", format!("Bearer {}",token));
        let mut ws = tungstenite::connect(builder)?.0;
        let s = ws.read()?;
        let lifecycle_event:LifecycleEvent=serde_json::from_str(s.to_string().as_str())?;
        return Ok(BotWebsocket{
            url:url.to_string(),
            id: lifecycle_event.self_id,
            ws: Arc::new(Mutex::new(ws)),
            subscribes:Subscribes::<Self>::default()
        })
    }
}

impl Bot for BotWebsocket {
    fn send(&mut self,string:&String)->Result<(),BotError> {
        match self.ws.lock().unwrap().send(Message::text(string)){
            Ok(_)=>{
                printinf(format!("Send:{}",&string));
                Ok(())
            }
            Err(err)=>{Err(err.to_string().into())}
        }
    }
    fn send_with_recive(&mut self,string:&String)->Result<String,BotError> {
        match self.send(string){
            Ok(_)=>{
                match self.ws.lock().unwrap().read() {
                    Ok(s) => {
                        Ok(s.to_string())
                    }
                    Err(err) => return Err(err.into())
                }
            }
            Err(err)=>{Err(err.into())}
        }
    }
    async fn send_async(&mut self,string:&String)->Result<(),BotError> {
        match self.ws.lock().unwrap().send(Message::text(string)){
            Ok(_)=>{Ok(())}
            Err(err)=>{Err(err.to_string().into())}
        }
    }
    async fn send_with_recive_async(&mut self,string:&String)->Result<String,BotError> {
        match self.ws.lock().unwrap().send(Message::text(string)){
            Ok(_)=>{
                match self.ws.lock().unwrap().read() {
                    Ok(s) => {
                        Ok(s.to_string())
                    }
                    Err(err) => return Err(err.to_string().into())
                }
            }
            Err(err)=>{Err(err.to_string().into())}
        }
    }
    fn recv_msg(&self)->Result<String,String>{
        Ok(self.ws.lock().unwrap().read().unwrap().to_string())
    }
}

impl BotAPI for BotWebsocket {
    fn run(self) -> JoinHandle<()> {
        let mut bot = self;//别用self
        thread::spawn(move ||{
            loop{
                match bot.recv_msg(){
                Ok(str) => {
                    match Event::from(&str) {
                        Ok(Event::LifecycleEvent{event}) => {
                            if let Some(f) = bot.subscribes.lifecycle_event{
                                f(&mut bot,event)
                            }
                        },
                        Ok(Event::HeartbeatEvent{event}) => {
                            if let Some(f) = bot.subscribes.heartbeat_event{
                                f(&mut bot,event)
                            }
                        },
                        Ok(Event::GroupMsgEvent{event}) => {
                            if let Some(f) = bot.subscribes.group_msg_event{
                                f(&mut bot,event)
                            }
                        },
                        Ok(Event::PrivateMsgEvent{event}) => {
                            if let Some(f) = bot.subscribes.private_msg_event{
                                f(&mut bot,event)
                            }
                        },
                        Err(err) => {
                            printerr(err);
                            printerr(str)
                        },
                    }
                },
                Err(err) => printerr(err),
                }
            }
        })
    }
    fn subscribe(&mut self,factory:EventResolve<Self>)->() {
        match factory {
            EventResolve::LifecycleEvent(f) => self.subscribes.lifecycle_event=Some(f),
            EventResolve::HeartbeatEvent(f) => self.subscribes.heartbeat_event=Some(f),
            EventResolve::GroupMsgEvent(f) => self.subscribes.group_msg_event=Some(f),
            EventResolve::PrivateMsgEvent(f) => self.subscribes.private_msg_event=Some(f),
        }
    }
    fn send_private_msg<T:Display>(&mut self,id:&i64,s:T)->Result<(),BotError>{
        send_private_msg(self, id, s)
    }
    fn get_status(&mut self)->Result<EchoGetStatus, BotError>{
        get_status(self)
    }
    fn get_version_info(&mut self)->Result<EchoGetVersionInfo, BotError>{
        get_version_info(self)
    }
    fn get_login_info(&mut self)->Result<EchoLoginInfo, BotError>{
        get_login_info(self)
    }
    fn send_group_msg<T:Display>(&mut self,group_id:&i64,s:T)->Result<(),BotError> {
        send_group_msg(self, group_id, s)
    }
    
    fn delete_msg(&mut self,msg_id:&i64)->Result<(),BotError> {
        delete_msg(self, msg_id)
    }
}