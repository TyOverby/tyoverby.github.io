---
title: "Pattern Matching Everywhere"
date: 2025-01-15
draft: true
---

Been using Rust for a while now, and I still get excited every time I discover a new place where pattern matching works.

Today's discovery: you can use patterns in closure arguments!

```rust
let pairs = vec![(1, 2), (3, 4), (5, 6)];
let sums: Vec<_> = pairs.iter()
    .map(|(a, b)| a + b)
    .collect();
```

I love how the `|(a, b)|` just works. No need for `|pair| pair.0 + pair.1`.
