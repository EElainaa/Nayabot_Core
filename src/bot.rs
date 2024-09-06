//onebot v11
use std::{fmt::Display, net::TcpStream, sync::{Arc, Mutex}};
use crate::{event::*, funs::printinf};
use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};

type WS=Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>;

pub struct BotHttp{}
pub struct BotRHrrp{}

#[derive(Debug)]
pub struct BotWebsocket{
    url:String,
    id:i64,
    ws:WS
}

impl BotWebsocket {
    pub fn new<T:Display>(url:T)->Result<BotWebsocket,String>{
        match tungstenite::client::connect(url.to_string()) {
            Ok((mut ws,_resp)) =>{
                match ws.read() {
                    Ok(s)=>{
                        let lifecycle_event:lifecycle_event=serde_json::from_str(s.to_string().as_str()).unwrap();
                        return Ok(BotWebsocket{
                            url:url.to_string(),
                            id: lifecycle_event.self_id,
                            ws: Arc::new(Mutex::new(ws))
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
}

trait BotSend {
    fn send(&mut self,string:String)->Result<(),String>;
    fn send_with_recive(&mut self,string:String)->Result<String,String>;
    async fn send_async(&mut self,string:String)->Result<(),String>;
    async fn send_with_recive_async(&mut self,string:String)->Result<String,String>;
}

impl BotSend for BotWebsocket {
    fn send(&mut self,string:String)->Result<(),String> {
        match self.get_ws().lock().unwrap().send(Message::text(string)){
            Ok(_)=>{printinf("success");
                Ok(())
            }
            Err(err)=>{Err(err.to_string())}
        }
    }
    
    fn send_with_recive(&mut self,string:String)->Result<String,String> {
        match self.send(string){
            Ok(_)=>{
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
    
    async fn send_async(&mut self,string:String)->Result<(),String> {
        match self.ws.lock().unwrap().send(Message::text(string)){
            Ok(_)=>{Ok(())}
            Err(err)=>{Err(err.to_string())}
        }
    }
    
    async fn send_with_recive_async(&mut self,string:String)->Result<String,String> {
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
}

impl BotWebsocket {
    pub fn send_private_msg<T:Display>(&mut self,id:&i64,s:T)->Result<(),String>{
        send_private_msg(self, id, s)
    }
    pub fn get_status(&mut self)->Result<echo_get_status, String>{
        get_status(self)
    }
    pub fn get_version_info(&mut self)->Result<echo_get_version_info, String>{
        get_version_info(self)
    }
}

pub struct BotRWebsocket{}

/*
#[allow(dead_code)]
impl Bot {
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
}*/

fn send_private_msg<B:BotSend,T:Display>(bot:&mut B,id:&i64,s:T)->Result<(), String>{
    return bot.send(format!("{{\"action\": \"send_private_msg\",\"params\": {{\"user_id\":{},\"message\":{}}}}}",id,s))
}

//获取bot状态
fn get_status<T:BotSend>(bot:&mut T)->Result<echo_get_status, String>{
    match bot.send_with_recive(format!("{{\"action\": \"get_status\"}}")){
        Ok(res)=>{
            printinf("send successly");
            Ok(serde_json::from_value::<echo_get_status>(serde_json::from_str::<echo_event>(res.as_str()).unwrap().data).unwrap())
        }
        Err(err)=>{Err(err)}
    }
}

fn get_version_info<T:BotSend>(bot:&mut T)->Result<echo_get_version_info, String>{
    match bot.send_with_recive(format!("{{\"action\": \"get_version_info\"}}")){
        Ok(res)=>{
            Ok(serde_json::from_value::<echo_get_version_info>(serde_json::from_str::<echo_event>(res.as_str()).unwrap().data).unwrap())
        }
        Err(err)=>{Err(err)}
    }
}


