---
title: "Demo: Inline Code Snippets in Hugo"
date: 2025-01-26
draft: true
---

This post demonstrates how to inline code snippets from external source files using Hugo shortcodes.

## Overview

Instead of copying code into markdown files, we can maintain source code separately and pull specific regions into our posts. This keeps code DRY and ensures examples stay in sync with actual source files.

## Example: A Simple Parser

Let's look at a toy parser implementation. First, we define our token types:

{{< code-snippet file="static/examples/parser.rs" region="token_definition" lang="rust" >}}

The tokenizer needs to handle numbers specially, since they can span multiple characters:

{{< code-snippet file="static/examples/parser.rs" region="tokenize_number" lang="rust" >}}

And here's the main tokenization loop that ties it all together:

{{< code-snippet file="static/examples/parser.rs" region="main_tokenize" lang="rust" >}}

## How It Works

In the source file (`static/examples/parser.rs`), regions are marked with comments:

```rust
// SNIPPET_START: region_name
... code to extract ...
// SNIPPET_END: region_name
```

Then in markdown, use the shortcode:

```
{{</* code-snippet file="static/examples/parser.rs" region="region_name" lang="rust" */>}}
```

## Benefits

- **Single source of truth**: Code lives in actual source files
- **Type checking**: Your examples can be compiled/tested
- **Easy updates**: Change the source file, and all posts update automatically
- **Clean separation**: Keep code and prose separate

The full source file is available at `/examples/parser.rs`.
