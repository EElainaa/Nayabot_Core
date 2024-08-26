use chrono::{DateTime, FixedOffset};
use rusqlite::Connection;
use serde_json::Value;
use colored::Colorize;

use crate::bot::{At, Forward, Image, Json, Mface, MsgData, Text};

pub fn array_to_string(array:&Vec<Value>)->String{
    let mut ostr=String::new();
    array.iter().for_each(|s|{
        let m:MsgData = serde_json::from_value(s.clone()).unwrap();//解析消息类型
        match m.r#type.as_str() {
            "text"=>{let data:Text=serde_json::from_value(m.data).unwrap();ostr=ostr.clone()+&data.text}//解析文字消息
            "image"=>{let data:Image=serde_json::from_value(m.data).unwrap();ostr=ostr.clone()+"[图片:"+&data.url+"]"}//解析图片消息
            "mface"=>{let data:Mface=serde_json::from_value(m.data).unwrap();ostr=ostr.clone()+&data.summary}//解析表情消息
            "at"=>{let data:At=serde_json::from_value(m.data).unwrap();ostr=ostr.clone()+"@"+&data.name+"("+&data.qq+") "}
            "reply"=>{}//暂时咕咕咕
            "json"=>{let data:Json=serde_json::from_value(m.data).unwrap();ostr=ostr.clone()+&data.data}
            "forward"=>{let data:Forward=serde_json::from_value(m.data).unwrap();ostr=ostr.clone()+"[转发消息:"+&data.id+"]"}
            _=>{println!("{}",m.r#type);println!("{}",m.data)}
        }
    });
    ostr
}

pub fn time_to_string(time:i64)->String{
    DateTime::from_timestamp(time,0).unwrap().with_timezone(&FixedOffset::east_opt(8*3600).unwrap()).format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn printinf<T: std::fmt::Display>(s:T){
    println!("{} [{}]{}",time_to_string(chrono::Local::now().timestamp()),"Inf".green(),s)
}

pub fn printerr<T: std::fmt::Display>(s:T){
    println!("{} [{}]{}",time_to_string(chrono::Local::now().timestamp()),"Err".red(),s)
}

pub fn printwrm<T: std::fmt::Display>(s:T){
    println!("{} [{}]{}",time_to_string(chrono::Local::now().timestamp()),"Wrm".yellow(),s)
}

pub fn getdata_sql()->Connection{
    Connection::open("data.sql").unwrap()
}
