Today [bincode](https://github.com/tyoverby/bincode) hits 1.0.0!  Before we truly get started, a brief history

* Sep 15, 2014: First prototype, then named `writer_encoder` was written on a plane because I was too poor to pay for in-flight wifi so I could download the JSON crate.
* Oct 27, 2014: Rename to `bincode`.
* Apr 05, 2015: Initial port to serde written by [erickt](https://github.com/erickt).
  Prior to this, we were using [rustc-serialize](https://github.com/rust-lang-deprecated/rustc-serialize).
* Aug 8, 2015: Servo starts using bincode in [ipc-channel](https://crates.io/crates/ipc-channel)!
* Jan 12, 2016: [Tarpc](https://github.com/google/tarpc) starts using bincode!
* Apr 21, 2017: Serde hits 1.0!  Serde maintainer [dtolnday](https://github.com/dtolnay)
  ports bincode to use the new Serde APIs.

Many thanks to [everyone that has contributed]( https://github.com/TyOverby/bincode/graphs/contributors),
I deeply appreciate the help.

# What is Bincode
At it's root, Bincode is a serializer implementation for Serde.
If you stick a `#[derive(Deserialize, Serialize)]` on your struct, you can use bincode to efficiently
serialize and deserialize those structs to and from bytes.

> If bincode is just another serializer implementation, what sets it apart from all the others?

Bincode is a bit weird in that it's a format that was purpose built for the rust serialization
ecosystem.  It's tight coupling with Serde allows bincode to be very fast and serialize to
very small payloads.

# How does Bincode work?
The way that bincode achieves its speed and size wins is by choosing to not encode structure
into the serialized output.  It relies on the fact that `#[derive(Deserialize, Serialize)]`
implementation of the serde traits deserializes fields in _exactly the same order_ as it
serialized them.  This means that if you implement those traits by hand, you need to uphold this
invariant as well.

Because Bincode leaves out this extraneous structure information, an struct serialized with bincode is
often smaller than it was in memory!

Let's take a look at how bincode serializes some common structures.

### Encoding Numbers

All the rust numbers are encoded directly into the output in little-endian format by default.

```rust
use bincode::{serialize, deserialize};
let bytes: Vec<u8> = serialize(&123456789u32).unwrap();
// a 4-byte u32 gets serialized to 4 bytes.
assert_eq!(bytes.len(), 4);
let number: u32 = deserialize(&bytes).unwrap();
```
<img src="../images/bincode/u32.svg" style="padding: 5px; background:rgb(240, 240, 240)"/>

### Encoding Strings

Strings are serialized by first serializing the length, then the byte content of the string.

```rust
use bincode::{serialize, deserialize};
let bytes: Vec<u8> = serialize(&String::from("hello!")).unwrap();
let string: String = deserialize(&bytes).unwrap();
```

<img src="../images/bincode/string.svg" style="padding: 5px; background:rgb(240, 240, 240)"/>

### Encoding Structs
When serializing a struct, each field is serialized in order of their declaration in the struct.
No additional field information is encoded.

```rust
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    age: u32,
    name: String
}

let person = Person {
    // hey, making these svgs is hard, I'm reusing values for my own sanity.
    age: 123456789,
    name: String::from("hello!"),
};

let bytes: Vec<u8> = serialize(&person).unwrap();
let person_2: Person = deserialize(&bytes).unwrap()A;
```

<img src="../images/bincode/struct.svg" style="padding: 5px; background:rgb(240, 240, 240)"/>

### Encoding Enums
Enums are serialized as a tag (u32) followed by a their fields serialized in declaration order.

```rust
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize)]
struct NumberOrString {
    Number(u32),
    String(String),
}

let num = NumberOrString::Number(123456789);
let string = NumberOrString::String("hello!");

let bytes_num: Vec<u8> = serialize(&num).unwrap();
let bytes_string: Vec<u8> = serialize(&string).unwrap();

let num_out: NumberOrString = deserialize(&bytes_num);
let num_out: NumberOrString = deserialize(&bytes_string);
```

<img src="../images/bincode/enum.svg" style="padding: 5px; background:rgb(240, 240, 240)"/>


# Should I use Bincode?
Bincode is great for very specific serialization tasks, but is less than ideal for others.
To help you decide if you should use bincode or not, I've provided a helpful cost benefit
analysis below.

__Pros__

* Fast
* Small serialized size
* Configurable options for improving speed and size even further!
    * Choose between endianness for integers
    * Set message size limits for Denial Of Service protection
    * Upcoming configurations
        * Variable sized integers for lengths and enum variants
        * Null terminated strings

__Cons__

* Not human readable
* __Serialized data can not be read if structure changes__
  * No re-ordering fields
  * No adding / removing fields

The "no structure changes" drawback can not be understated.  If your program
requires backwards-compatible data representations, take a look at other
formats like [ProtoBuf](https://github.com/google/protobuf) and
[Cap'n Proto](https://capnproto.org/).

However, there are many areas where those limitations aren't actual issues!
The best example of this is [ipc-channel](https://crates.io/crates/ipc-channel) which
uses bincode to send structs across the process boundary.  When both processes are the
same binary, there's no need to worry about the struct definitions being different.

Another popular use case would be video games.  Rarely do networked video games permit
players of different versions of a game to connect to each other.  Similarly to the
ipc-channel example, if every player is running the same build, no need to worry about
back-compat issues.

# The Future of Bincode
Bincode 1.0.0 is at a point where I feel comfortable recommending the project and
am happy overall with the library ergonomics.

Being both a library and an ad-hoc data format, Bincode has some interesting compatibility
requirements.

* The library itself must obey semver (as all crates should).
* Data encoded by one version of Bincode should be readable by future versions of Bincode.

I feel like both of these are fairly easy to achieve while also permitting Bincode to evolve
through the use of configuration options.

