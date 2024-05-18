# Super Simple Structs

A language and compiler for defining language agnostic message protocols using packed-structs.

The compiler takes protocol specifications as `.sss` files and emits source code for de/encoding protocol in one of a few supported programming languages.
The compiler is written in Rust and has a plugin model to allow implementing source generation logic for other programming languages.


```
protocol my.protocol [version = 1.0] {

    message EventRequest [1] {
        u8  event_id;
        bool   require_all;
        u64 quantity_requested;
    }
    
    enum Outcome : u8 { SUCCESS = 1, FAIL = 2, UNKNOWN = 3 }
            
    message OventOutcome [2] {
        u64 timestamp;
        u8  event_id;
        u64 quantity_granted;
    }
    
    
    message LogMessage [3] {
        u64 timestamp;
        12; 
        char[30] message;
    }
}
```


## Features (one day)

Unlike more complex messaging protocols all fields in `sss` protocols are guaranteed to have a fixed offset within a message.
This allows really nice features like parser-free access to serialized data directly to/from binary buffers.

## Types

Scalar types are
 - unsigned integers `u8`,`u16`, `u32`, `u64`
 - signed integers `i8`, `i16`, `i32`, `i64`
 - floating point `f32`, `f64`

 - `byte`, `char` (`ascii`)

Note `byte` and `char` are aliases for `u8` to aid in code generation & indicating to protocol users the form of the data.

Fixed length arrays can also be used with the `t[X]` type where `t` is a scalar type and `X` is the length of the buffer.

E.g `ascii[30]` means a 30 byte sequence of ascii bytes - code generators can use this information to provide nice APIs 
around fields of this type, such as exposing a  `sss::ascii_view<30>` in the c++ implementation.


## Messages

A message is the base unit in a `sss` protocol, a message comprises an identifier and a set of fields.
Within a protocol each message must have a unique identifier.

```
message EventRequest [1] {
    u8   event_id;
    bool require_all;
    u64  quantity_requested;
}
```

## Structs

Structs are compositions of fields which can be reused in multiple messages. 



```
struct UserInfo {
    ascii:10  username;
    ascii:100 session_token;
}


message LoginResponse [5] {
    UserInfo user;
    ascii:50 @message;
}

message OtherResponse [6] {
    UserInfo user;
    // ...
}
```

_note: Structs do not have an identifier as they are not intended to live on the wire in isolation._

## Reserved bytes

If certain bytes of a message need to be reserved for later then an anonymous field can be declared with a size only.
The fields following this reservation will be moved back by this size.

```
struct foo {
    u8 f1;
    20;
    u8 f2; <- this byte is at offset 21 within the message 
} 
```

### Enums

Enums allow assigning a name to a value.

```
enum Outcome : u8 { SUCCESS = 1, FAIL = 2, UNKNOWN = 3 }
```

### Bitflags

bitflags are like enums which will auto assign different bit values to the variants allowing representation of multiple 
states in a single scalar value.

```
bitflags FailFlags : u8 { FLAG_1, FLAG_2, FLAG_3, FLAG_4 }
```