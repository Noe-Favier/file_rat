use std::{
    fmt::{Debug, Display},
};
use uuid::{Uuid};

//TODO: method access
pub struct RFile {
    pub uuid: Uuid,

    pub size: u64,
    pub name: String,
}