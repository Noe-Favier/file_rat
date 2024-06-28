use crate::structs::rat_file::RatFile;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(dead_code)]
impl<'de, T: Serialize + Deserialize<'de>> RatFile<T> {
    pub(crate) fn remove(&mut self, id: Uuid) -> Result<(), std::io::Error> {
        Ok(())
    }
}
