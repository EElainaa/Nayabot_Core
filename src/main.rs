
mod bot;
mod funs;
mod event;
mod modules;

use std::collections::HashMap;

use bot::{Bot, Rstatus};
use modules::*;
use event::*;
use funs::*;

use lazy_static::lazy_static;
use serde_rusqlite::{to_params_named,from_rows};
use ws::{connect, Message};
use serde_json::Value;

static DEFAULT_MODULES_OPTIONS: modules::modules_options=modules_options{
    log:true
};

lazy_static!{
    static ref Modsoptions:HashMap<i64,modules_options> = {
        let data_sql = getdata_sql();
        match data_sql.prepare("SELECT * FROM options"){//读取options表
            Ok(_)=>{}//数据库已初始化
            Err(_)=>{//数据库未初始化
                printwrm("数据库未初始化");
                printinf("进行数据库初始化");
                printinf("添加默认配置");
                match data_sql.execute(r#"CREATE TABLE options (
                                    group_id NUMERIC PRIMARY KEY NOT NULL UNIQUE,
                                    log      BLOB    NOT NULL UNIQUE DEFAULT (true));"#,[]){
                                        Ok(_) =>{
                                            data_sql.execute("INSERT INTO options (group_id, log) VALUES (0, :log)",to_params_named(&DEFAULT_MODULES_OPTIONS).unwrap().to_slice().as_slice()).unwrap();
                                        },
                                        Err(_) =>{printerr("初始化数据库失败");},
                                    };
            }
        };
        //将配置转入HashMap
        let mut tmp = data_sql.prepare("SELECT * FROM options").unwrap();
        let t = from_rows::<modules_options>(tmp.query([]).unwrap());
        let mut m:HashMap<i64,modules_options> = HashMap::new();
        for (group_id,modules_options) in t.enumerate(){
            m.insert(group_id.try_into().unwrap(), modules_options.unwrap());
        }
        m
    };
}

fn main(){
    //正向ws
    // 连接到url并调用闭包
    if let Err(_error) = connect("ws://192.168.1.38:3001", |sender| {
        let bot=Bot::new(sender.clone());
        move |msg:Message| Ok({
            match serde_json::from_str::<Rstatus>(msg.to_string().as_str()) {
                Ok(status)=>{
                    if status.status=="failed" {
                        printerr(status.wording);
                        return Ok(());
                    }else {
                        printinf(status.wording);
                        return Ok(());
                    }
                }
                _=>{}
            }
            
            match serde_json::from_str::<Value>(msg.to_string().as_str()).unwrap()["post_type"].as_str().unwrap() {
                "meta_event"=>{//元事件
                    match serde_json::from_str::<Value>(msg.to_string().as_str()).unwrap()["meta_event_type"].as_str().unwrap() {
                        "lifecycle"=>{
                            let lifecycle_msg:lifecycle_event=serde_json::from_str(msg.to_string().as_str()).unwrap();
                            println!("{}",lifecycle_msg);
                            //执行Bot登录成功事件
                            return Ok(());
                        }
                        "heartbeat"=>{//心跳事件
                            let heartbeat_msg:heartbeat_event=serde_json::from_str(msg.to_string().as_str()).unwrap();
                            if !heartbeat_msg.status["online"].as_bool().unwrap(){
                                printwrm("Bot已断开连接");
                                //执行Bot下线事件
                                return Ok(());
                            }
                        }
                        _=>{println!("{}",msg);return Ok(());}
                    }
                }
                "message"=>{//消息事件
                    match serde_json::from_str::<Value>(msg.to_string().as_str()).unwrap()["message_type"].as_str().unwrap() {
                        "group"=>{//群消息
                            let group_msg:group_msg_event=serde_json::from_str(msg.to_string().as_str()).unwrap();
                            println!("{}",group_msg);
                            //执行群消息处理事件
                            if Modsoptions.get(&0).unwrap().log{
                                log::add_log(&group_msg)
                            }
                            return Ok(());
                        }
                        "private"=>{//私聊消息
                            let private_msg:private_msg_event=serde_json::from_str(msg.to_string().as_str()).unwrap();
                            println!("{}",private_msg);
                            //执行私聊消息处理事件
                            return Ok(());
                        }
                        _=>{println!("{:?}",msg);return Ok(());}
                    }
                }
                _=>{println!("{}",msg);return Ok(());}
            }
        })
    }) {}

}


