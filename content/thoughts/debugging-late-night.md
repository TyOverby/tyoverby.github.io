---
title: "Late Night Debugging"
date: 2025-01-10
draft: true
---

Spent two hours debugging a concurrency issue only to realize I was looking at cached log output.

The fix was working after the first 10 minutes. I was just running `tail -f` on the wrong log file.

Note to self: always check which terminal tab you're looking at. Maybe get more sleep.
