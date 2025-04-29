use std::{net::TcpStream, sync::Arc};
use anyhow::{Error, Ok};
use tokio::sync::Mutex;
use tungstenite::{stream::MaybeTlsStream, WebSocket};

type WS=WebSocket<MaybeTlsStream<TcpStream>>;

/// 协议端通信网络协议
#[derive(Debug,Clone)]
pub(crate) struct NetProtocol{
    /// 一个[NetControl]类型的实例
    pub(crate) instance:NetControl
}

impl NetProtocol {
    pub(crate) async fn send(&self,msg:String)->Result<(),Error>{
        self.instance.send(msg).await
    }
    pub(crate) async fn recv(&self)->Result<String,Error>{
        self.instance.recv().await
    }
}

/// 协议端通信网络控制
//#[async_trait]
/*
    /// 向协议端发送一个[String]类型的数据
    async fn send(&self,msg:String)->Result<(),Error>;
    /// 从协议端接收一个[String]类型的数据
    async fn recv(&self)->Result<String,Error>;
    async fn send_with_recv(&self,msg:String)->Result<String,Error>;
*/

#[derive(Debug)]
pub(crate) enum NetControl{
    WSProtocol(Arc<Mutex<WS>>),
    ReserverWSProtocol,
    HttpProtocol,
    ReserverHttpProtocol
}

impl Clone for NetControl {
    fn clone(&self) -> Self {
        match self {
            Self::WSProtocol(arg0) => Self::WSProtocol(arg0.clone()),
            Self::ReserverWSProtocol => Self::ReserverWSProtocol,
            Self::HttpProtocol => Self::HttpProtocol,
            Self::ReserverHttpProtocol => Self::ReserverHttpProtocol,
        }
    }
}

impl NetControl{
    /// 向协议端发送一个[String]类型的数据
    async fn send(&self,msg:String)->Result<(),Error>{
        match self {
            NetControl::WSProtocol(ws) => {
                ws.lock().await.send(tungstenite::Message::text(msg))?;
                Ok(())
            },
            NetControl::ReserverWSProtocol => todo!(),
            NetControl::HttpProtocol => todo!(),
            NetControl::ReserverHttpProtocol => todo!(),
        }
    }
    async fn recv(&self)->Result<String,Error>{
        match self {
            NetControl::WSProtocol(ws) => {
                let mut con = ws.lock().await;
                if con.can_read(){
                    Ok(con.read()?.to_string())
                }else{
                    Err(Error::msg("连接关闭"))
                }
            },
            NetControl::ReserverWSProtocol => todo!(),
            NetControl::HttpProtocol => todo!(),
            NetControl::ReserverHttpProtocol => todo!(),
        }
    }
}

pub enum ProtocolType{
    /// 参数(url,token)
    WebSocket(String,String),
    /// 参数(port,token)
    ReserverWebSocket(i32,String),
    /// 参数(url,token)
    Http(String,String),
    /// 参数(port,token)
    ReserverHttp(i32,String)
}
