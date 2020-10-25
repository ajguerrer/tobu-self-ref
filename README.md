# Self Ref

`self-ref` is a prototype for creating a reflective type system like the one in use by
[protobuf-go][protobuf-go].

This project is named self-ref because it relies on structs with self referential fields to tie
reflective information to the concrete values.

## Goals

Investigate whether self-ref fields are a viable solution for protobuf refection in rust. To be
viable, several sub-goals need to be met:

1. Type must be able to convert into its reflection and back without losing information.
1. Any changes to the values of a reflection are captured when the reflection is converted back to
   it's concrete type.
1. Reflection interface must be similar to the one defined in [protobuf-go][protobuf-go], but 
   feels "[rusty](https://rust-lang.github.io/api-guidelines/)", including utilizing rust's type 
   system.
1. Must be possible to generate reflection implementations with the protobuf compiler through a 
   plugin.
1. Reflection must support proto2 and proto3 spec including nested messages.
1. Dynamic parents - messages can be constructed in isolation or nested in another message.
   -  If isolated, the parent descriptor must be the descriptor of the file where the message is
      declared.
   -  If nested, the parent descriptor must be the descriptor of the parent message where the 
      message is nested.

[protobuf-go]: https://pkg.go.dev/google.golang.org/protobuf/reflect/protoreflect