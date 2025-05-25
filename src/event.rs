
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use anyhow::{Context, Error};
use serde_with::skip_serializing_none;

use crate::{bot::{BaseBot, Bot}, message::Message};

#[derive(Debug,Clone)]
pub enum Event{
    LifecycleEvent{event:LifecycleEvent},
    HeartbeatEvent{event:HeartbeatEvent},
    GroupMsgEvent{event:GroupMsgEvent},
    PrivateMsgEvent{event:PrivateMsgEvent},
    GroupRecall{event:GroupRecall},
    EchoEvent{event:EchoEvent},
    GroupCard{event:GroupCard},
    GroupMsgEmojiLikeEvent{event:GroupMsgEmojiLikeEvent},
    PokeEvent{event:PokeEvent},
    TitleEvent{event:TitleEvent},
    GroupBanEvent{event:GroupBanEvent},
    GroupAdminEvent{event:GroupAdminEvent},
    Error{msg:String}
}

impl Event {
    pub fn from(s:&str)-> Result<Event,Error>{
        let e = serde_json::from_str::<BaseEvent>(s).context(format!("获取事件类型失败:{s}")).context(format!("{:?}",serde_json::from_str::<BaseEvent>(s)))?;
        if e.echo.is_some(){
            return Ok(Event::EchoEvent{event:serde_json::from_str::<EchoEvent>(s).context(format!("获取事件类型失败:{s}")).context(format!("{:?}",serde_json::from_str::<EchoEvent>(s)))?});
        }
        match e.post_type.unwrap() {
            PostType::Message =>
                match e.message_type.unwrap() {
                    MessageType::Group => Ok(Event::GroupMsgEvent{event:serde_json::from_str::<GroupMsgEvent>(s).context(format!("{:?}",serde_json::from_str::<GroupMsgEvent>(s)))?}),
                    MessageType::Private => Ok(Event::PrivateMsgEvent{event:serde_json::from_str::<PrivateMsgEvent>(s).context(format!("{:?}",serde_json::from_str::<GroupMsgEvent>(s)))?})
                },
            PostType::MessageSent => {Ok(Event::GroupMsgEvent{event:serde_json::from_str::<GroupMsgEvent>(s).context(format!("{:?}",serde_json::from_str::<GroupMsgEvent>(s)))?})},
            PostType::Notice => 
                match e.notice_type.unwrap() {
                    NoticeType::GroupCard => Ok(Event::GroupCard{event:serde_json::from_str::<GroupCard>(s).context(format!("{:?}",serde_json::from_str::<GroupCard>(s)))?}),
                    NoticeType::GroupMsgEmojiLike => Ok(Event::GroupMsgEmojiLikeEvent{event:serde_json::from_str::<GroupMsgEmojiLikeEvent>(s).context(format!("{:?}",serde_json::from_str::<GroupMsgEmojiLikeEvent>(s)))?}),
                    NoticeType::Notify => 
                            match e.sub_type.unwrap() {
                                SubType::Poke => Ok(Event::PokeEvent{event:serde_json::from_str::<PokeEvent>(s).context(format!("{:?}",serde_json::from_str::<PokeEvent>(s)))?}),
                                SubType::Title => Ok(Event::TitleEvent{event:serde_json::from_str::<TitleEvent>(s).context(format!("{:?}",serde_json::from_str::<TitleEvent>(s)))?}),
                                _ => {Err(Error::msg(format!("获取事件类型失败:{s}")))},
                            }
                    NoticeType::GroupRecall => Ok(Event::GroupRecall{event:serde_json::from_str::<GroupRecall>(s).context(format!("{:?}",serde_json::from_str::<GroupRecall>(s)))?}),
                    NoticeType::GroupBan => Ok(Event::GroupBanEvent{event:serde_json::from_str::<GroupBanEvent>(s).context(format!("{:?}",serde_json::from_str::<GroupBanEvent>(s)))?}),
                    NoticeType::GroupAdmin => Ok(Event::GroupAdminEvent{event:serde_json::from_str::<GroupAdminEvent>(s).context(format!("{:?}",serde_json::from_str::<GroupAdminEvent>(s)))?})
                }
            PostType::MetaEvent => {
                match e.meta_event_type.unwrap() {
                    MetaEventType::Heartbeat => Ok(Event::HeartbeatEvent{event:serde_json::from_str::<HeartbeatEvent>(s).context(format!("{:?}",serde_json::from_str::<HeartbeatEvent>(s)))?}),
                    MetaEventType::Lifecycle => Ok(Event::LifecycleEvent{event:serde_json::from_str::<LifecycleEvent>(s).context(format!("{:?}",serde_json::from_str::<LifecycleEvent>(s)))?})
                }
            },
        }
    }
    pub fn to_string(&self) -> Result<String,Error> {
        match self {
            Event::LifecycleEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::HeartbeatEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::GroupMsgEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::PrivateMsgEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::GroupRecall { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::EchoEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::GroupCard { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::GroupMsgEmojiLikeEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::Error { msg } => Ok(msg.to_string()),
            Event::PokeEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::TitleEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::GroupBanEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
            Event::GroupAdminEvent { event } => Ok(serde_json::to_value(event)?.to_string()),
        }
    }
    pub fn is_ok(&self) -> bool{
        match self {
            Event::EchoEvent { event } => {event.is_ok()}
            // 其他事件无论如何都返回true
            _=>{true}
        }
    }
}
#[derive(Debug,Clone,Serialize,Deserialize)]
#[skip_serializing_none]
struct  BaseEvent {
    post_type:Option<PostType>,
    message_type:Option<MessageType>,
    meta_event_type:Option<MetaEventType>,
    notice_type:Option<NoticeType>,
    sub_type:Option<SubType>,
    echo:Option<u16>,
    data:Option<Value>
}
/// 上报类型
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum PostType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "message_sent")]
    MessageSent,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "meta_event")]
    MetaEvent
}
/// 消息类型
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum MessageType{
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "private")]
    Private
}
/// 元事件类型
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum MetaEventType{
    #[serde(rename = "heartbeat")]
    Heartbeat,
    #[serde(rename = "lifecycle")]
    Lifecycle
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum NoticeType{
    #[serde(rename = "group_card")]
    GroupCard,
    #[serde(rename = "group_msg_emoji_like")]
    GroupMsgEmojiLike,
    #[serde(rename = "notify")]
    Notify,
    #[serde(rename = "group_recall")]
    GroupRecall,
    #[serde(rename = "group_ban")]
    GroupBan,
    #[serde(rename = "group_admin")]
    GroupAdmin
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum SubType{
    #[serde(rename = "poke")]
    Poke,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "friend")]
    Friend,
    #[serde(rename = "ban")]
    Ban,
    #[serde(rename = "lift_ban")]
    LiftBan,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "unset")]
    UnSet
}
/// 生命周期事件
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct LifecycleEvent{
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub meta_event_type:String,
    pub sub_type:String
}
/// 心跳事件
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct HeartbeatEvent{
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub meta_event_type:String,
    pub status:Value,
    pub interval:i32
}

/// 群聊消息事件
#[derive(Debug,Clone,Serialize,Deserialize)]
#[skip_serializing_none]
pub struct GroupMsgEvent{
    pub self_id:i64,
    pub user_id:i64,
    pub time:i64,
    pub message_id:i64,
    pub message_seq:i64,
    pub real_id:i64,
    pub real_seq:String,
    pub message_type:MessageType,
    pub sender:MsgSender,
    pub raw_message:String,
    pub font:i16,
    pub sub_type:String,
    #[serde(serialize_with = "serialize_message", deserialize_with = "deserialize_message")]
    pub message:Message,
    pub message_format:String,
    pub post_type:PostType,
    pub message_sent_type:Option<String>,
    pub group_id:i64,
    pub target_id:Option<i64>,
    pub raw:Value
}
impl GroupMsgEvent{
    pub fn reply(&self,bot:&Bot,msg:&Message)->Result<SendMsgEcho,Error>{
        let mut m = Message::new().reply(self.message_id);
        m.push(msg);
        bot.send_group_msg(&self.group_id,&m)
    }
    pub fn delete(&self,bot:&Bot)->Result<(), Error>{
        bot.delete_msg(&self.message_id)
    }
    pub fn get_reply(&self)->Option<crate::message::Reply>{
        self.message.get_reply()
    }
        pub fn get_forward(&self)->Option<crate::message::Forward>{
        self.message.get_forward()
    }
        pub fn get_first_text(&self)->Option<String>{
        self.message.get_first_text()
    }
}
fn serialize_message<S>(message: &Message, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&Value::Array(message.msg.clone()).to_string())
}

fn deserialize_message<'de, D>(deserializer: D) -> Result<Message, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Array(msg) => Ok(Message{msg}),
        _=>{Ok(Message::new())}
    }
}

/// 群聊消息发送者
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct MsgSender{
    pub user_id:i64,
    pub nickname:String,
    pub sex:Option<String>,
    pub age:Option<i32>,
    pub card:Option<String>,
    pub area:Option<String>,
    pub level:Option<String>,
    pub role:Option<GroupRole>,
    pub title:Option<String>
}
/// 群权限
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum GroupRole{
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member
}
/// 群消息子类型
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum GroupSubType {
    Normal,
    Anonymous,
    Notice
}
/// 私聊消息
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PrivateMsgEvent{
    pub self_id:i64,
    pub user_id:i64,
    pub time:i64,
    pub message_id:i64,
    pub message_seq:i64,
    pub real_id:i64,
    pub message_type:String,
    pub sender:MsgSender,
    pub raw_message:String,
    pub font:i16,
    pub sub_type:String,
    #[serde(serialize_with = "serialize_message", deserialize_with = "deserialize_message")]
    pub message:Message,
    pub message_format:String,
    pub post_type:PostType
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct NoticeEvent{
    pub time:i64,
    pub self_id:i64,
    pub post_type:PostType,
    pub notice_type:String,
    pub sub_type:String,
    pub target_id:i64,
    pub user_id:i64,
    pub group_id:i64,
    pub raw_info:Value
}

/// 回应事件
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EchoEvent{
    pub status:String,
    pub retcode:i8,
    pub data:Option<Value>,
    pub message:String,
    pub wording:String,
    pub echo:u16
}
impl EchoEvent {
    pub fn is_ok(&self)->bool{
        if self.status=="ok"{true}else{false}
    }
}
/// 执行结果回应
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SendMsgEcho{
    pub message_id:i64,
}
/// 获取状态回应
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EchoGetStatus{
    pub online:bool,
    pub good:bool,
    pub stat:Value
}
/// 获取协议端版本信息回应
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EchoGetVersionInfo{
    pub app_name:String,
    pub protocol_version:String,
    pub app_version:String,
}
/// 获取登录信息回应
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EchoLoginInfo{
    pub user_id:i64,
    pub nickname:String
}
/// 群名片改变事件
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ChangeGroupCard{
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:String,
    pub card_new:String,
    pub card_old:String
}
/// 群聊消息撤回事件
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GroupRecall{
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:String,
    pub operator_id:i64,
    pub message_id:i64
}
/// 设置群管理员事件
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ChangeGroupAdmin{
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:String,
    pub sub_type:String
}
/// 获取消息回应
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EchoGetMsg{
    pub time:i64,
    pub message_id:i64,
    pub message_type:MessageType,
    pub real_id:i64,
    pub sender:MsgSender,
    pub message:Value
}
/// 群成员名片变化事件
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GroupCard{
    pub time:i64,
    pub self_id:i64,
    pub post_type:PostType,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:NoticeType,
    pub card_new:String,
    pub card_old:String
}
/// 群消息表情回应事件
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GroupMsgEmojiLikeEvent{
    pub time:i64,
    pub self_id:i64,
    pub post_type:PostType,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:NoticeType,
    pub likes:Vec<EmojiLike>
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EmojiLike{
    pub emoji_id:String,
    pub count:i32
}
/// 戳一戳事件
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PokeEvent{
    pub time:i64,
    pub self_id:i64,
    pub post_type:PostType,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:NoticeType,
    pub target_id:i64,
    pub raw_info:Vec<Value>,
    pub sub_type:SubType
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct TitleEvent{
    pub time:i64,
    pub self_id:i64,
    pub post_type:PostType,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:NoticeType,
    pub sub_type:SubType,
    pub title:String
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GroupBanEvent{
    pub time:i64,
    pub self_id:i64,
    pub post_type:PostType,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:NoticeType,
    pub operator_id:i64,
    pub duration:i64,
    pub sub_type:SubType
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GroupAdminEvent{
    pub time:i64,
    pub self_id:i64,
    pub post_type:PostType,
    pub group_id:i64,
    pub user_id:i64,
    pub notice_type:NoticeType,
    pub sub_type:SubType
}