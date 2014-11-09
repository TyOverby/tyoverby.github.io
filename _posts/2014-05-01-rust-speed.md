---
layout: post
title: A Rusty Microbenchmark
subtitle: Rust eating C food
---

As a totally contrived example, I will be computing the expression

<p>
    $ \sum_{i=0}^{1,000,000,000} \begin{cases} i/2 + 2 & \text{if } i \% 2 = 0 \\\\ 0 & \text{otherwise} \end{cases}$
</p>

in both C and Rust to see how close in performance Rust can get to C while still being written in
an idiomatic pure-functional style.

# Rust
```rust
use std::iter::range;

fn main() {
    let mut iterator =
      range(0i32, 1_000_000_000i32)
        .filter(|x| *x % 2 == 0)
        .map(|x| x / 2)
        .map(|x| x + 2);
    let ans = iterator.fold(0, |a, b| a + b);
    println!("{}", ans);
}
```

# C

```c
#include <stdio.h>

void main(void) {
    int counter = 0;
    for (int i = 0; i < 1000000000; i++) {
        if (i % 2 == 0) {
            counter += (i / 2) + 2;
        }
    }
    printf("%d\n", counter);
}
```

# Benchmark / Analysis

Both of these implementations are flawed in various ways:
1. Using 32-bit integers means that the computation overflows very quickly
2. Finding the even numbers doesn't require a filter or if statement when you
   could only look at every other number to begin with.
3. The rust implementation should do both map steps at once.

These issues were kept despite their flaws:
1. To avoid slower number operations;
2. To test how well the compilers handle branching at the CPU level; and
3. To see if Rust is able to optimize away the additional map.

I was incredibly surprised to see that on my machine, Rust was able to
outperform C reliably.

## Rust
```
$ rustc iter.rs  --opt-level 3
$ time ./iter
-1583310976

real    0m0.884s
user    0m0.872s
sys     0m0.004s
```
## C
```
$ gcc test.c -std=c99 -O3
$ time ./a.out
-1583310976

real    0m1.398s
user    0m1.388s
sys     0m0.004s
```

I'm sure that there are ways to get the C version faster, but the only important takeaway from this
5 second experiment is that the Rust compiler does a damn good job of providing abstractions
that hit really close to the metal.

