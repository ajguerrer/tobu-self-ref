use std::fmt;
use std::ptr::NonNull;
use pin_project::pin_project;

use crate::descriptor::{FieldDescriptor, MessageDescriptor, Parent};

#[pin_project]
#[derive(Debug)]
pub struct Reflection<T> {
    pub info: Message,
    #[pin]
    pub inner: T,
}

impl<T: ToFieldRefs> Reflection<T> {
    pub fn new(inner: T, descriptor: &'static MessageDescriptor, parent: Parent) -> Self {
        let mut reflection = Reflection {
            info: Message::new(descriptor, parent),
            inner,
        };
        reflection.wire_self_references();
        reflection
    }

    fn wire_self_references(&mut self) {
        todo!()
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}



#[derive(Debug)]
pub enum FieldRef<'a> {
    Int(&'a i32),
    Message(Vec<FieldRef<'a>>),
}

pub trait ToFieldRefs {
    fn to_field_refs(&self) -> Vec<FieldRef>;
}

pub struct Message {
    pub descriptor: &'static MessageDescriptor,
    pub parent: Parent,
    pub nested: Vec<Message>,
    pub fields: Vec<Field>,
}

impl Message {
    pub fn new(descriptor: &'static MessageDescriptor, parent: Parent) -> Self {
        Message {
            descriptor,
            parent,
            nested: Vec::new(),
            fields: Vec::new(),
        }
    }
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
    pub parent: Parent,
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
