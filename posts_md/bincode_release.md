---
title: Bincode 1.0.0
author: Ty Overby
image: ../images/snowy-mountains.jpeg
abstract: |
    Today [Bincode](https://github.com/tyoverby/bincode) hits 1.0.0!  Before we truly get started, a brief history:
    <table>
    <tr>
    <td style="min-width: 7em;">Sep 15 2014</td>
    <td>First prototype - then named <code>writer_encoder</code> - was written while I was on a plane with no wifi.
    </tr>
    <tr>
    <td>Oct 27 2014</td>
    <td>Rename to <code>bincode</code></td>
    </tr>
    <tr>
    <td>Apr 05 2015</td>
    <td>Initial port to serde written by <a href="https://github.com/erickt">erickt</a> Prior to this, we were using <a href="https://github.com/rust-lang-deprecated/rustc-serialize">rustc-serialize</a></td>
    </tr>
    <tr>
    <td>Aug 08 2015</td>
    <td><div class="line-block">Servo starts using bincode in <a href="https://crates.io/crates/ipc-channel">ipc-channel</a></div></td>
    </tr>
    <tr>
    <td>Jan 12 2016</td>
    <td><a href="https://github.com/google/tarpc">Tarpc</a> starts using bincode</td>
    </tr>
    <tr>
    <td>Apr 21 2017</td>
    <td>Serde hits 1.0! Serde maintainer <a href="https://github.com/dtolnay">dtolnay</a> ports bincode to use the new Serde APIs</td>
    </tr>
    </table>

    Many thanks to [everyone who has contributed]( https://github.com/TyOverby/bincode/graphs/contributors).
    I deeply appreciate the help.
---

# What is Bincode?
At it's root, Bincode is a serializer implementation for Serde.
If you stick a `#[derive(Deserialize, Serialize)]` on your struct, Bincode can efficiently
serialize and deserialize those structs to and from bytes.

> If Bincode is just another serializer implementation, what sets it apart from all the others?

Bincode is unique in that it's a format that was built specifically for the Rust serialization
ecosystem.  Tight coupling with Serde allows Bincode to be very fast and serialize to
very small payloads.

# How does Bincode work?
Bincode achieves its speed and size wins is by not encoding structure information
into the serialized output.  It relies on the fact that `#[derive(Deserialize, Serialize)]`
implementations of the Serde traits deserializes fields in _exactly the same order_ as it
serialized them.  (If you implement those traits by hand, you need to uphold this
invariant as well.)

Because Bincode leaves out this extraneous structure information, a struct serialized with Bincode is
often smaller than it was in memory! Let's take a look at how Bincode serializes some common structures.

### Encoding Numbers

All the Rust numbers are encoded directly into the output in little-endian format by default.

```rust
use bincode::{serialize, deserialize};
let bytes: Vec<u8> = serialize(&123456789u32)?;
// a 4-byte u32 gets serialized to 4 bytes.
assert_eq!(bytes.len(), 4);
let number: u32 = deserialize(&bytes)?;
```
<img src="../images/bincode/u32.svg" />

### Encoding Strings

Strings are serialized by first serializing the length and then serializing the byte content of the string.

```rust
use bincode::{serialize, deserialize};
let bytes: Vec<u8> = serialize(&String::from("hello!"))?;
let string: String = deserialize(&bytes)?;
```

<img src="../images/bincode/string.svg" />

### Encoding Structs
When serializing a struct, each field is serialized in order of its declaration in the struct.
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

let bytes: Vec<u8> = serialize(&person)?;
let person_2: Person = deserialize(&bytes)?;
```

<img src="../images/bincode/struct.svg" />

### Encoding Enums
Enums are serialized as a tag (u32) followed by their fields serialized in declaration order.

```rust
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize)]
struct NumberOrString {
    Number(u32),
    String(String),
}

let num = NumberOrString::Number(123456789);
let string = NumberOrString::String("hello!");

let bytes_num: Vec<u8> = serialize(&num)?;
let bytes_string: Vec<u8> = serialize(&string)?;

let num_out: NumberOrString = deserialize(&bytes_num)?;
let num_out: NumberOrString = deserialize(&bytes_string)?;
```

<img src="../images/bincode/enum.svg" />


# Should I use Bincode?
Bincode is great for very specific serialization tasks, but is less than ideal for others.
To help you decide if you should use it or not, I've provided a helpful cost benefit
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
* No cross-language support
* __Serialized data can not be read if structure changes__
  * No reordering fields
  * No adding / removing fields

The "no structure changes" drawback can not be understated.  If your program
requires backwards-compatible data representations, take a look at other
formats such as [ProtoBuf](https://github.com/google/protobuf) or
[Cap'n Proto](https://capnproto.org/).

However, there are many areas where those limitations aren't actual issues!
The best example of this is [ipc-channel](https://crates.io/crates/ipc-channel) which
uses bincode to send structs across the process boundary.  When both processes are the
same binary, there's no need to worry about the struct definitions being different.

Another popular use case would be video games.  Networked video games rarely permit
players of different versions of a game to connect to each other.  Similarly to the
ipc-channel example, if every player is running the same build, there is no need to
worry about back-compat issues.

# The Future of Bincode
Bincode 1.0.0 is at a point where I feel comfortable recommending the project and
am happy overall with the library ergonomics.

Being both a library and an ad-hoc data format, Bincode has some interesting compatibility
requirements.

* The library itself must obey semver (as all crates should).
* Data encoded by one version of Bincode should be readable by future versions of Bincode.

Both of these are fairly easy to achieve while also permitting Bincode to evolve
through the use of configuration options.  As new language features come online (I'm looking
at you impl-trait), bincode will be re-released with major version changes.
