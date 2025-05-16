
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
