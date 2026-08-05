---
title: Text editor architecture from ed to jupyterlab
date: 2026-08-05
toc: false
draft: true
---

In the early days, computers were machines that could take up an entire wall,
and users interacted with them via
[teletypes](https://en.wikipedia.org/wiki/Teleprinter).  This electronic
typewriter could be in the same room as the computer, in a building next door,
or in an operator's residence where they would connect it [to their home
telephone](https://en.wikipedia.org/wiki/Acoustic_coupler) and dial in.  [Line
editors](https://en.wikipedia.org/wiki/Line_editor) like
[Ed](https://en.wikipedia.org/wiki/Ed_\(text_editor\)){{< sidenote >}}the
standard Unix text editor{{< /sidenote >}} were a good fit for the technical limitations of the day: the "command
/ response" model of editing works well when you can see your commands printed
onto the teletype paper immediately and the response might involve a long
round-trip to the nearby university.

With the advent of the personal computer, we started to see interactive text
editors like [vi](https://en.wikipedia.org/wiki/Vi_\(text_editor\)) and
[emacs](https://en.wikipedia.org/wiki/Emacs).  With a digital screen and the
computer physically attached to the monitor, interactive editing was practical
and useful!  These editors and their descendants have dominated developer
usage up until today, and their architecture assumes in many places that the
editor UI is running on the same machine that is storing the files, hosting
language tooling, has your built executables, etc.

But nowadays I rarely edit files on the physical machine that I'm typing into!
Lots of my work recently is happening on machines in "the cloud" because that's
where the beefy GPUs are.  I'm running vim over ssh{{< sidenote >}}because one of my vim plugins runs
executables that need to be on the machine with the GPU{{< /sidenote >}}, and the
experience sucks - with round trip latency that can reach into the hundreds of
milliseconds, every keypress takes ages to show its effect on my screen.

In the machine learning community{{< sidenote >}}where programming on powerful
computers far away from your laptop is common{{< /sidenote >}}, many practitioners use notebooks like
[JupyterLab](https://jupyter.org).  Much like `Ed`, with a notebook, you type
out a full command locally and then send it to the remote machine to be
evaluated.  Now I don't think that notebook-programming was developed solely to
save programmers from typing latency,{{< sidenote >}}similarly, I think that
Ed's design was mostly constrained by typewriter hardware and not
round-trip-timing for keypresses{{< /sidenote >}} but I do believe that development with a notebook is what made the
practice of "writing code where it runs" ergonomic in this era of slow links to
expensive hardware.
