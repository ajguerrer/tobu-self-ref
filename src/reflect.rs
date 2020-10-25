use std::fmt;
use std::ptr::NonNull;

use crate::descriptor::{FieldDescriptor, MessageDescriptor, Parent};

#[derive(Debug)]
pub struct Reflection<T> {
    pub info: Message,
    pub inner: T,
}

impl<T> Reflection<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

pub struct Message {
    pub descriptor: &'static MessageDescriptor,
    pub parent: fn() -> Parent,
    pub nested: Vec<Message>,
    pub fields: Vec<Field>,
}

// Custom impl to deal with parent/child cycles
impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Message")
            .field("descriptor", &self.descriptor)
            .field("parent", &format_args!("{:p}", &self.parent))
            .field("nested", &self.nested)
            .field("fields", &self.fields)
            .finish()
    }
}

pub struct Field {
    pub descriptor: &'static FieldDescriptor,
    pub parent: fn() -> Parent,
    pub value: ValueRaw,
}

// Custom impl to deal with parent/child cycles
impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Message")
            .field("descriptor", &self.descriptor)
            .field("parent", &format_args!("{:p}", &self.parent))
            .field("value", &self.value.as_ref())
            .finish()
    }
}

#[derive(Debug)]
pub enum ValueRaw {
    Int(NonNull<i32>),
    Message(NonNull<Message>),
}

impl ValueRaw {
    pub fn as_ref(&self) -> Value {
        match self {
            ValueRaw::Int(i) => Value::Int(unsafe { i.as_ref() }),
            ValueRaw::Message(m) => Value::Message(unsafe { m.as_ref() }),
        }
    }

    pub fn as_mut(&mut self) -> ValueMut {
        match self {
            ValueRaw::Int(i) => ValueMut::Int(unsafe { i.as_mut() }),
            ValueRaw::Message(m) => ValueMut::Message(unsafe { m.as_mut() }),
        }
    }
}

#[derive(Debug)]
pub enum Value<'a> {
    Int(&'a i32),
    Message(&'a Message),
}

#[derive(Debug)]
pub enum ValueMut<'a> {
    Int(&'a mut i32),
    Message(&'a mut Message),
}
