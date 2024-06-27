use serde::{Deserialize, Serialize};
use uuid::{timestamp, v1::Context, ClockSequence, Timestamp, Uuid};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileItem<T>{
    pub(crate) id: Uuid,

    name: String,
    metadata: T,

    start: u64,
    end: u64,
    size: u64,
}

impl<T> FileItem<T> {
    pub fn new(name: String, metadata: T, size: u64, start: u64, end: u64) -> Self {

        let context = Context::new_random();
    
        let t: Timestamp = Timestamp::now(context);
        let fuuid = Uuid::new_v1(t, &[1, 2, 3, 4, 5, 6]);

        Self {
            id: fuuid,
            name,
            metadata,
            start,
            end,
            size
        }
    }
}