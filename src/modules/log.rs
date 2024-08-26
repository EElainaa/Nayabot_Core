use serde::{Deserialize, Serialize};
use crate::{getdata_sql, printerr};

pub trait Log {
    fn to_log(&self)->MsgLog;
}

#[derive(Serialize, Deserialize)]
pub struct MsgLog{//一条log的格式
    pub id:i64,
    pub time:String,
    pub sender_id:i64,
    pub msg:String
}

pub fn create_group_table(id:i64){//创建一个表
    let data_sql=getdata_sql();
    match data_sql.execute(format!("CREATE TABLE '{}' (time TEXT PRIMARY NOT NULL, sender_id NUMERIC NOT NULL, message TEXT NOT NULL);",id).as_str(),[]) {
        Ok(_)=>{},
        Err(err)=>{printerr(format!("写入日志失败：{}",err))}
    };
}

pub fn add_log<T:Log>(msg:&T){//添加一条日志
    let data_sql=getdata_sql();
    let msglog=msg.to_log();
    match data_sql.prepare(format!("SELECT * FROM '{}'",msglog.id).as_str()) {
        Err(_)=>{create_group_table(msglog.id)}
        _=>{}
    };
    match data_sql.execute(format!("INSERT INTO '{}' (time, sender_id, message) VALUES ('{}', '{}', '{}')",msglog.id,msglog.time,msglog.sender_id,msglog.msg).as_str(),[]) {
        Err(err)=>{printerr(format!("写入日志失败:{}",err))}
        _=>{}
    }
}
