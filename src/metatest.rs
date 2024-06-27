use std::fs::File;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct MetadataTest {
    text: String,
    hour: String,
}

impl MetadataTest {
    pub fn new() -> Self {
        Self {
            text: "Hello, world!".to_string(),
            hour: "12:00".to_string(),
        }
    }
    
}