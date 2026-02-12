---
title: "Background Filter Experiments"
subtitle: "SVG filters and backdrop-filter cross-browser compatibility"
date: 2025-12-06
draft: true
tags: ["css", "svg", "blog"]
---

I tried adding blurred, grainy backgrounds to blog posts using CSS filters and SVG. The goal was
simple: take a background image, blur it heavily, add film grain texture, and make it scroll with
the page content. What I found was a mess of browser compatibility issues.

## Initial Approach: Direct Blur on Background Image

The obvious first attempt was applying `filter: blur(100px)` directly to an element with a
background image:

```css
html::before {
    background-image: url("/images/city.jpg");
    background-size: cover;
    filter: blur(100px);
}
```

This produced terrible results. The blur operates at the image's rendered resolution, and since
`background-size: cover` scales the image up significantly, the blur became jagged and pixelated. A
100px blur at screen resolution becomes much smaller when applied to a zoomed-in source image,
resulting in a choppy, low-quality effect.

## Attempt 2: Pseudo-elements and backdrop-filter

I switched to layered pseudo-elements to apply the blur at viewport resolution:

```css
html::before {
    background-image: url("/images/city.jpg");
    background-size: cover;
    z-index: -2;
}

html::after {
    backdrop-filter: blur(100px) url(#grain);
    z-index: -1;
}
```

The `backdrop-filter` property applies effects to whatever is behind an element. By layering two
pseudo-elements, the bottom one shows the image and the top one blurs it. I added an SVG filter for
grain using `feTurbulence`:

```xml
<filter id="grain">
  <feTurbulence type="fractalNoise" baseFrequency="0.8" numOctaves="4" stitchTiles="stitch" />
  <feBlend mode="overlay" in2="SourceGraphic" />
</filter>
```

This worked in Chrome. The blur applied at viewport resolution, avoiding the jagged edges you get
when blurring at image resolution. The grain overlaid nicely.

It does not work in safari. Safari doesn't support SVG filters in `backdrop-filter`. The
`url(#grain)` part being ignored would be ok, but in practice it just disables the entire filter
rule. I tried moving the grain to a regular `filter` property instead:

```css
html::after {
    backdrop-filter: blur(100px);
    filter: url(#grain);
}
```

This also failed in Safari. The grain would render, but the blur doesn't work.

## Attempt 3: Real DOM Elements

I switched from pseudo-elements to actual divs:

```html
<div class="bg-wrapper">
    <img class="bg-img" src="/images/city.jpg" />
    <div class="bg-blur"></div>
</div>
```

The wrapper applied the grain filter, the img showed the background, and the blur div sat on top
with `backdrop-filter: blur(100px)`. Still broken in Safari.

## Attempt 4: SVG Blur Filter

I replaced CSS `blur()` with an SVG `feGaussianBlur` filter:

```xml
<filter id="blur-opaque">
  <feGaussianBlur stdDeviation="100" in="SourceGraphic" result="blurred" />
  <feComponentTransfer in="blurred" result="opaque">
    <feFuncA type="discrete" tableValues="1 1" />
  </feComponentTransfer>
</filter>
```

The `feComponentTransfer` step forces full opacity because `feGaussianBlur` introduces transparency
at edges. Applied to the image:

```css
.bg-img {
    filter: url(#blur-opaque);
}
```

But this has the same issues as my initial attempt: the blur is being computed on the scale of the
input image instead of the display resolution, resulting in a distinctly unblurred image.

## Failed Adaptive Darkening Experiments

Along the way, I tried to darken only bright pixels to improve text contrast. The first idea was a
threshold-based approach:

```xml
<feColorMatrix type="luminanceToAlpha" in="SourceGraphic" result="luma" />
<feComponentTransfer in="luma" result="thresholdedLuma">
  <feFuncA type="table" tableValues="0 0 0 0 0.2 0.6 1 1" />
</feComponentTransfer>
```

This converts to luminance, then uses a table to create a mask where only bright pixels pass
through. Didn't work - the `type="table"` transfer function produced solid white when used in
`backdrop-filter`.

I tried gamma curves:

```xml
<feComponentTransfer>
  <feFuncR type="gamma" amplitude="1" exponent="3" offset="0" />
  <feFuncG type="gamma" amplitude="1" exponent="3" offset="0" />
  <feFuncB type="gamma" amplitude="1" exponent="3" offset="0" />
</feComponentTransfer>
```

This darkened everything uniformly based on the power curve, but couldn't selectively darken only
bright areas.

The "darken blend with a flat gray" approach was the closest:

```xml
<feFlood flood-color="rgb(89,89,89)" flood-opacity="1" result="gray" />
<feBlend mode="darken" in="SourceGraphic" in2="gray" />
```

This clamps all pixels to a maximum brightness (35% in this case) using `min(source, gray)`. Dark
pixels stay unchanged, bright pixels get clamped. But this required `backdrop-filter`, which brings
us back to Safari compatibility issues.

## What I Learned

1. `backdrop-filter` with SVG `url()` filters doesn't work in Safari
2. CSS `blur()` and svg `<feGaussianBlur>` produces jagged edges when applied to a low resolution
   image, but works smoothly with `backdrop-filter`
3. SVG filter `type="table"` transfer functions break in `backdrop-filter` context
4. Layering effects across multiple elements with different filter types is fragile across browsers
