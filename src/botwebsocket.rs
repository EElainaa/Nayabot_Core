use std::{io::{Read, Write}, net::TcpStream, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;
use tungstenite::{stream::MaybeTlsStream, WebSocket};
use anyhow::Error;
use crate::bot::*;

/*
impl BotWebsocket{
    ///通过正向ws连接协议端
    pub fn new<T:Display>(url:T,token:&str)->Result<BotWebsocket,Error>{
        let uri = Uri::from_str(url.to_string().as_str()).with_context(||"Url格式错误")?;
        let builder = if token.is_empty(){tungstenite::ClientRequestBuilder::new(uri)}else{tungstenite::ClientRequestBuilder::new(uri).with_header("Authorization",format!("Bearer {}",token))};
        let mut ws = tungstenite::connect(builder).with_context(||format!("连接{}失败",url))?.0;
        let s = ws.read()?;
        let lifecycle_event:LifecycleEvent=serde_json::from_str(s.to_string().as_str())?;
        Ok(BotWebsocket{
            url:url.to_string(),
            id: lifecycle_event.self_id,
            ws: Mutex::new(ws),
            subscribes:Subscribes::<Self>::default()
        })
    }
    pub fn get_url(&self) -> &str {
        &self.url
    }
    pub fn get_id(&self) -> i64 {
        self.id
    }
}

impl Bot for BotWebsocket {
    async fn recv_msg<F>(&self,check: F,) -> Result<String,Error>where F: Fn(&str) -> bool + Send + 'static,{
        loop{
            let msg = self.ws.lock().await.read().unwrap().into_text().unwrap().to_string();
            if check(msg.as_str()){return Ok(msg)}
        }
    }
    async fn recv_msg_with_echo(&self,echo:String) -> Result<String,Error>{
        self.recv_msg(move |v|{
            match serde_json::from_str::<Echo>(v) {
                Ok(msgecho) => {
                    if echo.to_string()==msgecho.echo{
                        return true
                    }else{false}
                },
                Err(_) => false,
            }
        }).await
    } 
    async fn send(&mut self,string:&String)->anyhow::Result<(),Error> {
        match self.ws.lock().await.send(tungstenite::Message::Text(string.into())){
            std::result::Result::Ok(_) => Ok(()),
            Err(_) => Err(Error::msg(format!("向协议端发送消息:{}失败",string))),
        }
    }
    
    async fn send_with_recive(&mut self,string:&String)->anyhow::Result<String,Error> {
        let echo:i64 = random();
        match self.send(string).await {
            Ok(_) => {
                let result = self.recv_msg_with_echo(echo.to_string()).await.unwrap();
                Ok(result)
            },
            Err(err) => {Err(err)},
        }
    }
    
}

impl BotAPI for BotWebsocket {
    fn run(self) -> JoinHandle<()> {
        let bot = Arc::new(Mutex::new(self));
        tokio::spawn(async move {
            loop{
                let msg = {
                    let locked_bot = bot.lock().await;
                    locked_bot.recv_msg(|_| true).await
                };
                match msg{
                Ok(str) => {
                    let bot_clone = Arc::clone(&bot);
                    tokio::spawn(async move {
                        let mut locked_bot = bot_clone.lock().await;
                        match Event::from(&str) {
                            Ok(Event::LifecycleEvent{event}) => {
                                if let Some(f) = &mut locked_bot.subscribes.lifecycle_event{
                                    f(&mut locked_bot,event)
                                }
                            }
                            Ok(Event::HeartbeatEvent{event}) => {
                                if let Some(f) = &mut locked_bot.subscribes.heartbeat_event{
                                f(&mut locked_bot,event)
                            }
                            }
                            Ok(Event::GroupMsgEvent{event}) => {
                                if let Some(f) = &mut locked_bot.subscribes.group_msg_event{
                                    f(&mut locked_bot,event)
                                }
                            }
                            Ok(Event::PrivateMsgEvent{event}) => {
                                if let Some(f) = &mut locked_bot.subscribes.private_msg_event{
                                    f(&mut locked_bot,event)
                                }
                            }
                            Ok(Event::GroupRecall { event }) => {
                                if let Some(f) = &mut locked_bot.subscribes.group_recall{
                                    f(&mut locked_bot,event)
                                }
                            }
                            Err(err) => {
                                printerr(err);
                                printerr(str)
                            }
                };
                    });
                },
                Err(err) => printerr(err)
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
            EventResolve::GroupRecall(f) => self.subscribes.group_recall=Some(f)
        }
    }
    
    fn get_version_info(self)->anyhow::Result<EchoGetVersionInfo, Error> {
        todo!()
    }
    /*
    fn send_private_msg<T:Display>(&mut self,id:&i64,s:T)->Result<EchoStatus,Error>{send_private_msg(self, id, s)}
    fn get_status(&mut self)->Result<EchoGetStatus, Error>{get_status(self)}
    fn get_version_info(&mut self)->Result<EchoGetVersionInfo, Error>{
        let rt = Runtime::new().unwrap();
        let bot = Arc::new(Mutex::new(self));
        rt.block_on(async {
            get_version_info(bot).await
        })
    }
    fn get_login_info(&mut self)->Result<EchoLoginInfo, Error>{get_login_info(self)}
    fn send_group_msg<T:Display>(&mut self,group_id:&i64,s:T)->Result<EchoStatus,Error> {send_group_msg(self, group_id, s)}
    fn delete_msg(&mut self,msg_id:&i64)->Result<EchoStatus,Error> {delete_msg(self, msg_id)}
    fn clean_cache(&mut self)->Result<EchoStatus, Error> {clean_cache(self)}
    fn get_msg(&mut self,msg_id:&i64)->Result<EchoEvent, Error> {get_msg(self, msg_id)}
    fn get_forward_msg(&mut self,id:&i64)->Result<EchoEvent, Error> {get_forward_msg(self, id)}
    fn send_like(&mut self,user_id:&i64,times:i16)->Result<EchoStatus, Error> {send_like(self, user_id, times)}
    fn set_group_kick(&mut self,group_id:&i64,user_id:&i64,reject_add_request:&bool)->Result<EchoStatus, Error> {set_group_kick(self, group_id, user_id, reject_add_request)}
    fn set_group_ban(&mut self,group_id:&i64,user_id:&i64,duration:i64)->Result<EchoStatus, Error> {set_group_ban(self, group_id, user_id, duration)}
    fn set_group_whole_ban(&mut self,group_id:&i64,enable:&bool)->Result<EchoStatus, Error> {set_group_whole_ban(self, group_id, enable)}
    fn set_group_admin(&mut self,group_id:&i64,user_id:&i64,enable:&bool)->Result<EchoStatus, Error> {set_group_admin(self, group_id, user_id, enable)}
    fn set_group_card<T:Display>(&mut self,group_id:&i64,user_id:&i64,card:&T)->Result<EchoStatus, Error> {set_group_card(self, group_id, user_id, card)}
    fn set_group_name<T:Display>(&mut self,group_id:&i64,user_id:&i64,group_name:&T)->Result<EchoStatus, Error> {set_group_name(self, group_id, user_id, group_name)}
    fn set_group_leave(&mut self,group_id:&i64)->Result<EchoStatus, Error> {set_group_leave(self, group_id)}
    fn set_group_special_title <T:Display>(&mut self,group_id:&i64,user_id:&i64,special_title:&T)->Result<EchoStatus, Error> {set_group_special_title(self, group_id, user_id, special_title)}
    fn set_friend_add_request<T:Display>(&mut self,flag:&T,approve:&bool,remark:&T)->Result<EchoStatus, Error> {set_friend_add_request(self, flag, approve, remark)}
    fn set_group_add_request<T:Display>(&mut self,flag:&T,sub_type:&T,approve:&bool,reason:&T)->Result<EchoStatus, Error> {set_group_add_request(self, flag, sub_type, approve, reason)}
    fn get_stranger_info(&mut self,user_id:&i64)->Result<EchoEvent, Error> {get_stranger_info(self, user_id)}
    fn get_friend_list(&mut self)->Result<EchoEvent, Error> {get_friend_list(self)}
    fn get_group_info(&mut self,group_id:&i64)->Result<EchoEvent, Error> {get_group_info(self, group_id)}
    fn get_group_list(&mut self)->Result<EchoEvent, Error> {get_group_list(self)}
    fn get_group_member_info(&mut self,group_id:&i64,user_id:&i64)->Result<EchoEvent, Error> {get_group_member_info(self, group_id, user_id)}
    fn get_group_member_list(&mut self,group_id:&i64)->Result<EchoEvent, Error> {get_group_member_list(self, group_id)}
    fn get_group_honor_info(&mut self,group_id:&i64,r#type:String)->Result<EchoEvent, Error> {get_group_honor_info(self, group_id, r#type)}
    fn get_cookies<T:Display>(&mut self,domain:&T)->Result<EchoEvent, Error> {get_cookies(self, domain)}
    fn get_csrf_token(&mut self)->Result<EchoEvent, Error> {get_csrf_token(self)}
    fn get_credentials<T:Display>(&mut self,domain:&T)->Result<EchoEvent, Error> {get_credentials(self, domain)}
    fn get_record<T:Display>(&mut self,file:&T,out_format:&T)->Result<EchoEvent, Error> {get_record(self, file, out_format)}
    fn get_image<T:Display>(&mut self,file:&T)->Result<EchoEvent, Error> {get_image(self, file)}
    fn can_send_image(&mut self)->Result<EchoEvent, Error> {can_send_image(self)}
    fn can_send_record(&mut self)->Result<EchoEvent, Error> {can_send_record(self)}
    fn set_restart(&mut self,delay:i64)->Result<EchoStatus, Error> {set_restart(self, delay)}*/
}
*/