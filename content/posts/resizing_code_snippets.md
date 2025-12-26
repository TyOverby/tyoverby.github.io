---
title: "CSS: resizing code snippets to fit your device"
date: 2025-12-26
math: true
draft: true
---

I read a lot of programming blogs on my phone, and code blocks that don't fit width-wise on the
screen are a major source of irritation. Unlike prose, code blocks won't be dynamically
formatted{{< sidenote >}} Ok, but maybe it could? Imagine compiling `rustfmt` to wasm, and embedding
it in the page so that device would have code formatted perfectly?{{< / sidenote >}} based on the
available space on the page, so you're stuck with code that is usually 80 characters wide, causing
horizontal overflow that forces me to scroll left and right to follow the code.

I'd usually be fine with shrinking the text so that this doesn't happen, but it would be unfortunate
to force small text on desktop users.

With my own blog I wanted to play around with dynamically resizing the font size for code based on
the width of the screen, here's what it looks like:

<video style="width:100%;" src="/videos/resize_text_to_fit.mp4" muted autoplay loop playsinline></video>

# Table stakes

First thing I did for my blog is set up my code formatters to prefer shorter lines:

{{< filename >}}rustfmt.toml{{< /filename >}}

```toml
max_width  = 60
tab_spaces = 2
```

{{< filename >}}.ocamlformat{{< /filename >}}

```toml
profile = janestreet
version = 0.28.1
margin  = 60
```

{{< filename >}}.prettier.config.js{{< /filename >}}

```js
module.exports = { printWidth: 60 };
```

# The math

In order to exactly fit 60 characters on the screen, we want
$character\ width =
\frac{screen\ width}{60}$ so you might assume that
`font-size: calc(100vw / 60);`{{< sidenote >}} `100vw` is a dimensional value that measures "100% of
the viewport width" {{< / sidenote >}} would do the trick, but `font-size` sets the _height_ of the
font, not the width, so we'll have to scale it by the $\frac{character\ width}{character\ height}$
ratio{{< sidenote >}} This ratio is font-dependent, so you'll need to find the values for your font
{{< / sidenote >}}, like so:

<!-- prettier-ignore-start -->
```css
.autosize {
  --code-chars: 60;
  --code-char-ratio: 0.6;

  font-size: calc(
    100vw / (var(--code-chars) * var(--code-char-ratio)));
}
```
<!-- prettier-ignore-end -->

But we don't want the text to get too small or too big, so let's clamp{{< sidenote >}} Notice that
because the expression is inside of `clamp` the `calc` function is no longer needed
{{< / sidenote >}} it:

<!-- prettier-ignore-start -->
```css
.autosize {
  --code-chars: 60;
  --code-char-ratio: 0.6;

  font-size:
    clamp(
      8px,
      100vw / (var(--code-chars) * var(--code-char-ratio)),
      0.9em);
}
```
<!-- prettier-ignore-end -->

# Dynamic `--code-chars`?

If the goal is to only shrink the content when necessary, why not dynamically set `--code-chars` to
the length of the longest line in a code block? You can absolutely do this:

<!-- prettier-ignore-start -->
```js
for (let block of document.querySelectorAll(".autosize")) {
  let max_line_length = 0

  for (let line of block.innerText.split("\n")) {
    max_line_length = Math.max(max_line_length, line.length)
  }

  max_line_length = Math.min(max_line_length, 60)

  if (max_line_length > 0) {
    block.style.setProperty("--code-chars", max_line_length)
  }
}
```
<!-- prettier-ignore-end -->

but if you have a lot of code blocks on a page, you'll end up with _some_ of them being scaled by
different amounts than others, which looks really weird visually. A middleground that I'm fond of is
setting the scaling line length based on the longest line that can be found in the whole document,
like so:

<!-- prettier-ignore-start -->
```js
let max_line_length = 0
const codeblocks = document.querySelectorAll(".autosize")

for (let block of codeblocks) {
  for (let line of block.innerText.split("\n")) {
    max_line_length = Math.max(max_line_length, line.length)
  }
}

max_line_length = Math.min(max_line_length, 60)

if (max_line_length > 0) {
  for (let block of codeblocks) {
    block.style.setProperty("--code-chars", max_line_length)
  }
}
```
<!-- prettier-ignore-end -->
