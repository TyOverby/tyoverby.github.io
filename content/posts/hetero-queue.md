---
title: Heterogeneous Queue in Rust
date: 2016-10-05
draft: true
---

In object-oriented languages, it is common to have collections containing
objects that implement a certain interface.

```cs
interface Animal { ... }
class Cat: Animal { ... }
class Dog: Animal { ... }

List<Animal> listOfAnimals = ...;
```

And while this is certainly possible in rust by using trait-objects,

```rust
trait Animal { ... }

let list_of_animals: Vec<&Animal> = ...;
// or
let list_of_animals: Vec<Box<Animal>> = ...;
```

it is frowned upon due for stylistic as well as performance reasons:
each method call from here-on-out will be invoked via interface dispatch.

I recently encountered a problem that *could* be solved by using a vec of
trait objects, but due to performance concerns, I couldn't afford the
performance penalty.

Fortunately, the types of the objects in my list are known at compile-time,
and the number of the items in the list stays constant, so I thought I'd try
to work my way around this by way of a heterogeneous queue that constructs
a specialized type for each instance of the list.

## The bad approach

For comparisons sake, here's a rough outline of what I had *before* using
trait objects.


```rust
trait Shape { .. }

// `process(..)` makes a *lot* of method calls on `shape`, so having
// this trait object as a parameter kills perf.
fn process(shape: &Shape) { .. }

fn main() {
    let coll: Vec<&Shape> = vec![ .., .., .., .. ];

    for obj in coll {
        process(obj);
    }
}

```

## The good approach

```rust
trait Shape { .. }

// Generic function, no longer goes through interface dispatch.
fn process<S: Shape>(shape: &S) { .. }

trait ShapeList<T: Shape> {
    fn apply(&self);
}

struct ShapeNil<T: Shape> { }

struct ShapeCons<T: Shape, N: ShapeList<T>> {
    shape: T,
    next: N
}

impl <T> ShapeList<T> for ShapeNil<T> {
    fn apply(&self) {
        // do nothing, we've reached the end
    }
}

impl <T, N> ShapeList<T> for ShapeCons<T, N> {
    fn apply(&self) {
        // Process yourself
        process(&self.shape);
        // Then tell the next link to process itself
        next.apply();
    }
}
```
