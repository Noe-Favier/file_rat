use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum CompressionType {
    Fast = 1,
    Best = 2,
    #[default]
    Default = 3,
}

#[allow(dead_code)]
impl CompressionType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Fast),
            2 => Some(Self::Best),
            3 => Some(Self::Default),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        self.clone() as u8
    }

    pub fn is_valid(value: &str) -> bool {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "fast" | "best" | "default"
        )
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "fast" => Some(Self::Fast),
            "best" => Some(Self::Best),
            "default" => Some(Self::Default),
            _ => None,
        }
    }
}

