pub(crate) mod log;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case,non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize,PartialEq)]
pub struct modules_options{
    pub log:bool
}
