extern crate tobu_self_ref;

use std::ptr::NonNull;

use tobu_self_ref::descriptor::{
    FieldDescriptor, FieldType, FileDescriptor, MessageDescriptor, Parent,
};
use tobu_self_ref::reflect::{Field, Message, Value};

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
    _message: Message,
    field1: i32,
    field2: MyNestedMessage,
}

impl MyMessage {
    fn new() -> MyMessage {
        let mut m = MyMessage {
            _message: Message {
                fields: vec![
                    Field {
                        value: Value::Int(NonNull::dangling()),
                        descriptor: &MY_MESSAGE_DESC.fields[0],
                        parent: Parent::Message(&MY_MESSAGE_DESC),
                    },
                    Field {
                        value: Value::Message(NonNull::dangling()),
                        descriptor: &MY_MESSAGE_DESC.fields[1],
                        parent: Parent::Message(&MY_MESSAGE_DESC),
                    },
                ],
                descriptor: &MY_MESSAGE_DESC,
                parent: Parent::File(&MY_FILE_DESC),
            },
            field1: 0,
            field2: MyNestedMessage::new(),
        };
        m._message.fields[0].value = Value::Int(NonNull::from(&m.field1));
        m._message.fields[1].value = Value::Message(NonNull::from(&m.field2._message));
        m
    }

    fn reflect(&mut self) -> &mut Message {
        &mut self._message
    }
}

#[derive(Debug)]
struct MyNestedMessage {
    _message: Message,
    nested_field1: i32,
}

impl MyNestedMessage {
    fn new() -> MyNestedMessage {
        let mut m = MyNestedMessage {
            _message: Message {
                fields: vec![Field {
                    value: Value::Int(NonNull::dangling()),
                    descriptor: &MY_NESTED_MESSAGE_DESC.fields[0],
                    parent: Parent::Message(&MY_NESTED_MESSAGE_DESC),
                }],
                descriptor: &MY_NESTED_MESSAGE_DESC,
                parent: Parent::File(&MY_FILE_DESC),
            },
            nested_field1: 0,
        };
        m._message.fields[0].value = Value::Int(NonNull::from(&m.nested_field1));
        m
    }

    fn reflect(&mut self) -> &mut Message {
        &mut self._message
    }
}

fn main() {
    let mut nested = MyNestedMessage::new();
    let reflect = nested.reflect();

    assert!(matches!(reflect.fields[0].value, Value::Int(..)));

    if let Value::Int(nested_field1) = &mut reflect.fields[0].value {
        let nested_field1 = unsafe { nested_field1.as_mut() };
        *nested_field1 = 1;
    }

    assert_eq!(nested.nested_field1, 1);

    let mut message = MyMessage::new();
    message.field2 = nested;
    let reflect = message.reflect();

    assert!(matches!(reflect.fields[0].value, Value::Int(..)));
    assert!(matches!(reflect.fields[1].value, Value::Message(..)));

    if let Value::Message(field2) = &mut reflect.fields[1].value {
        let field2 = unsafe { field2.as_mut() };
        assert!(matches!(field2.fields[0].value, Value::Int(..)));

        if let Value::Int(nested_field1) = &mut field2.fields[0].value {
            let nested_field1 = unsafe { nested_field1.as_mut() };
            *nested_field1 = 2;
        }
    }

    // Uh-oh! nested moved!!!
    assert_eq!(message.field2.nested_field1, 2);
}
