use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[allow(dead_code)]
pub type RatMetaMap = Map<String, Value>;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct RatMetaBase {
    pub created_at: u64,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RatMeta<TCustom> {
    pub base: RatMetaBase,
    pub custom: TCustom,
}

impl RatMetaBase {
    pub(crate) fn new() -> Self {
        Self {
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

#[allow(dead_code)]
impl<TCustom> RatMeta<TCustom> {
    pub fn new(custom: TCustom) -> Self {
        Self {
            base: RatMetaBase::new(),
            custom,
        }
    }
}

#[allow(dead_code)]
impl RatMeta<RatMetaMap> {
    pub fn new_object() -> Self {
        Self::new(Map::new())
    }

    pub fn insert_custom<V: Into<Value>>(&mut self, key: impl Into<String>, value: V) {
        self.custom.insert(key.into(), value.into());
    }
}
