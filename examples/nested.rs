extern crate self_ref;

use std::ptr::NonNull;

use self_ref::{descriptor::{FieldDescriptor, FieldType, FileDescriptor, MessageDescriptor, Parent}, reflect::{ToFieldRefs, FieldRef}};
use self_ref::reflect::{Message, Reflection, ValueMut, ValueRaw};

const MY_FILE_DESC: FileDescriptor = FileDescriptor {
    id: 1,
    messages: &[&MY_MESSAGE_DESC],
};

const MY_MESSAGE_DESC: MessageDescriptor = MessageDescriptor {
    id: 2,
    fields: &[
        FieldDescriptor {
            id: 21,
            ty: FieldType::Int,
        },
        FieldDescriptor {
            id: 22,
            ty: FieldType::Message,
        },
    ],
    nested: &[&MY_NESTED_MESSAGE_DESC],
};

const MY_NESTED_MESSAGE_DESC: MessageDescriptor = MessageDescriptor {
    id: 3,
    fields: &[FieldDescriptor {
        id: 31,
        ty: FieldType::Int,
    }],
    nested: &[],
};

#[derive(Debug)]
struct MyMessage {
    field1: i32,
    field2: MyNestedMessage,
}

impl MyMessage {
    fn into_reflection(self) -> Reflection<Self> {
        let mut m = Reflection::<Self> {
            info: Message::new(&MY_MESSAGE_DESC, Parent::File(&MY_FILE_DESC)),
            inner: self,
        };
        m.info.fields[0].value = ValueRaw::Int(NonNull::from(&m.inner.field1));
        m.info.fields[1].value = ValueRaw::Message(NonNull::from(&m.info.nested[0]));
        m.info.nested[0].fields[0].value =
            ValueRaw::Int(NonNull::from(&m.inner.field2.nested_field1));
        m
    }
}

impl ToFieldRefs for MyMessage {
    fn to_field_refs(&self) -> Vec<FieldRef> {
        vec![FieldRef::Int(&self.field1), FieldRef::Message(vec![FieldRef::Int(&self.field2.nested_field1)])]
    }
}

#[derive(Debug)]
struct MyNestedMessage {
    nested_field1: i32,
}

impl MyNestedMessage {
    fn into_reflection(self) -> Reflection<Self> {
        let mut m = Reflection::<Self> {
            info: Message::new(&MY_NESTED_MESSAGE_DESC, Parent::File(&MY_FILE_DESC)),
            inner: self,
        };
        m.info.fields[0].value = ValueRaw::Int(NonNull::from(&m.inner.nested_field1));
        m
    }
}

impl ToFieldRefs for MyNestedMessage {
    fn to_field_refs(&self) -> Vec<FieldRef> {
        vec![FieldRef::Int(&self.nested_field1)]
    }
}

fn main() {
    let nested = MyNestedMessage { nested_field1: 0 };
    let mut reflect = nested.into_reflection();
    assert_eq!(reflect.inner.nested_field1, 0);
    assert!(matches!(reflect.info.fields[0].value, ValueRaw::Int(..)));
    if let ValueMut::Int(nested_field1) = reflect.info.fields[0].value.as_mut() {
        assert_eq!(*nested_field1, 0);
        *nested_field1 = 1;
        assert_eq!(reflect.inner.nested_field1, *nested_field1);
        reflect.inner.nested_field1 = 2;
        assert_eq!(reflect.inner.nested_field1, *nested_field1);
    }
    assert!(matches!(reflect.info.parent, Parent::File(..)));
    let nested = reflect.into_inner();
    assert_eq!(nested.nested_field1, 2);

    let message = MyMessage {
        field1: 0,
        field2: nested,
    };
    let mut reflect = message.into_reflection();
    assert!(matches!(
        reflect.info.fields[1].value,
        ValueRaw::Message(..)
    ));
    if let ValueMut::Message(field2) = reflect.info.fields[1].value.as_mut() {
        assert!(matches!(field2.fields[0].value, ValueRaw::Int(..)));
        if let ValueMut::Int(nested_field1) = field2.fields[0].value.as_mut() {
            assert_eq!(*nested_field1, 2);
            *nested_field1 = 3;
            assert_eq!(reflect.inner.field2.nested_field1, *nested_field1);
            reflect.inner.field2.nested_field1 = 4;
            assert_eq!(reflect.inner.field2.nested_field1, *nested_field1);
        }
        assert!(matches!(field2.parent, Parent::Message(..)));
    }
    let message = reflect.into_inner();
    assert_eq!(message.field2.nested_field1, 4);
}
