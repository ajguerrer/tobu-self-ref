#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileDescriptor {
    pub id: i32,
    pub messages: &'static [&'static MessageDescriptor],
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageDescriptor {
    pub id: i32,
    pub fields: &'static [FieldDescriptor],
    pub nested: &'static [&'static MessageDescriptor],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldDescriptor {
    pub id: i32,
    pub ty: FieldType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FieldType {
    Int,
    Message,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Parent {
    File(&'static FileDescriptor),
    Message(&'static MessageDescriptor),
}
