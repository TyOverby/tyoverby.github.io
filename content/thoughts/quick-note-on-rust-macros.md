---
title: "Quick Note on Rust Macros"
date: 2025-01-20
draft: true
---

Was debugging some macro expansion today and realized I keep forgetting about `cargo expand`.

It's such a useful tool for understanding what your macros actually generate. Just run:

```bash
cargo install cargo-expand
cargo expand
```

Game changer when you're dealing with complex proc macros.
