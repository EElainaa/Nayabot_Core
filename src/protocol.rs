
pub enum Protocol{
    /// 参数(url,token)
    WebSocket(String,String),
    /// 参数(port,token)
    ReserverWebSocket(String,String),
    /// 参数(url,token)
    Http(String,String),
    /// 参数(port,token)
    ReserverHttp(String,String)
}
