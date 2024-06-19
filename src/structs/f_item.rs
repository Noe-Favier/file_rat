use uuid::Uuid;

pub struct FileItem<T>{
    id: Uuid,

    name: String,
    metadata: T,

    start: u64,
    end: u64,
}