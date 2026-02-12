---
title: "In search of a new text editor: Part 1 - server/client split"
date: 2025-12-13
draft: true
toc: false
---

"Folks on IRC say that cool people use vim. I want to be cool! I'll use vim". Not the best logic,
but I was 14, and it was only the second worst decision I made that day (Arch user btw). Vim (and
later Neovim) have been constant companions throughout my journey as a programmer, and I'm very glad
that I paid down the cost of learning a complex editor at a time where wrestling with my development
environment was mainly just preventing me from making bad flash games.

# So why am I looking for a new editor?

In a word: "responsiveness". In six words: "responsiveness over high latency remote connections".
Neovim is very fast when you're running it locally, but most of my development actually happens on
remote machines, so I'm sshing to my development box and running `nvim` over there. This works fine
when latency is low, but as soon as the round-trip time for a TCP packet gets up into the
milliseconds, keypresses start to feel sluggish as they're sent to the remote machine to be
interpreted.

Fortunately, there's a pattern among new text editors that can help a lot in situations like this.

# Splitting the editor in two

The basic architecture that these new editors follow is that of the "client / server" split. The
server half of the editor lives on the development machine and handles things like:

- Saving files to disk
- Fetching completions
- Building & running artifacts
- Collecting diagnostics

while the client half runs on the computer that your keyboard is plugged into, and manages:

- Buffer state
- Keyboard shortcut interpretation
- Window management & layout
- UI and presentation

This separation of responsibilities is commonplace in other disciplines; web pages work like this,
but it's also found in multiplayer videogames so that user-input is handled locally. Imagine how bad
it would be if you needed a `client->server->client` round trip in order to turn the camera!

# The candidates

## VS Code (and forks)

Visual Studio Code was the first editor that I became aware of having a client / server split. I
suspect that this was a byproduct of their decision to use the Electron UI framework. Electron
forces developers to split their code between two processes: a Main processes with full access to
the node.js runtime, and a Render process that runs inside Chromium. These processes communicate via
IPC, and I imagine that VS Code's [remote development] functionality is primarily a proxy for IPC
endpoints.

In addition to their official [ssh extension] you can also run [a vscode webserver] on your
development machine, and then access it through the browser from another computer.

## Zed

<!-- references -->

[remote development]: https://code.visualstudio.com/docs/remote/ssh
[ssh extension]: https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-ssh
[a vscode webserver]: https://code.visualstudio.com/docs/remote/vscode-server
