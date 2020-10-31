#[derive(Debug)]
pub struct FileDescriptor {
    pub id: i32,
    pub messages: &'static [&'static MessageDescriptor],
}
#[derive(Debug)]
pub struct MessageDescriptor {
    pub id: i32,
    pub fields: &'static [FieldDescriptor],
    pub nested: &'static [&'static MessageDescriptor],
}

#[derive(Debug)]
pub struct FieldDescriptor {
    pub id: i32,
    pub ty: FieldType,
}

#[derive(Debug)]
pub enum FieldType {
    Int,
    Message,
}

#[derive(Debug)]
pub enum Parent {
    File(&'static FileDescriptor),
    Message(&'static MessageDescriptor),
}
