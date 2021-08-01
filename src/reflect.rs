use std::fmt;
use std::ptr::NonNull;

use crate::descriptor::{FieldDescriptor, MessageDescriptor, Parent};

pub struct Message {
    pub descriptor: &'static MessageDescriptor,
    pub parent: Parent,
    pub fields: Vec<Field>,
}

// Custom impl to deal with parent/child cycles
impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Message")
            .field("descriptor", &self.descriptor)
            .field("parent", &format_args!("{:p}", &self.parent))
            .field("fields", &self.fields)
            .finish()
    }
}

pub struct Field {
    pub descriptor: &'static FieldDescriptor,
    pub parent: Parent,
    pub value: Value,
}

// Custom impl to deal with parent/child cycles
impl<'a> fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Field")
            .field("descriptor", &self.descriptor)
            .field("parent", &format_args!("{:p}", &self.parent))
            .field(
                "value",
                match &self.value {
                    Value::Int(i) => unsafe { i.as_ref() },
                    Value::Message(m) => unsafe { m.as_ref() },
                },
            )
            .finish()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Int(NonNull<i32>),
    Message(NonNull<Message>),
}
