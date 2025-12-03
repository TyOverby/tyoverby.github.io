---
title: "New blog woes"
date: 2025-11-26
---

In the hopes that I'll start regularly blogging soon, I decided to use a real static site 
generator instead of the cobbled-together mess of `Makefile`s and `pandoc` that I used 
before.  

Less than a day into using it, I'm already regretting the decision to go with `hugo`. 
I built a hugo "shortcode" that pulls comment-delimited content from external files into the 
markdown files in order to keep source code in the blog posts up to date.  It works pretty well!

The main downside is that I can't figure out how to dedent the code that is being imported this way,
so if you pull in an OCaml function that is defined inside of a few nested modules, all the code 
keeps the original indentation, causing the alignment to be very ugly.  If anyone knows how to 
fix this, please let me know! 
