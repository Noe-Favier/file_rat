use uuid::{Uuid, v1::Context};

pub struct FileItem<T>{
    id: Uuid,

    name: String,
    metadata: T,

    start: u64,
    end: u64,
    size: u64,
}

impl<T> FileItem<T> {
    pub fn new(name: String, metadata: T, size: u64, start: u64, end: u64) -> Self {

        let context = Context::new(0);
        let my_uuid = Uuid::new_v1(&context, 0).unwrap();

        Self {
            id: uuid,
            name,
            metadata,
            start,
            end,
            size
        }
    }
}