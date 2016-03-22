Web-Assembly is a binary execution environment sheparded by major browser
vendors in the WC3.  Once implemented, web-assembly will be an alternative
to Javascript when it comes to programming dynamic clientside applications
on the web.

The "assembly" piece of web-assembly is the Virtual Instruction Set
Architecture (Virtual-ISA) that webassembly targets.  Unlike other
instruction set arcitectures (e.g. ARM, x86, Mips), web-assembly's ISA
doesn't actually map to a specific hardware type

Ok, so web-assembly is an ISA that runs compiled code in your web browser.
If you're still interested, here's a table of contents so you can decide
how much of this rambling article you want to skip.

## Table Of Contents

* [Table Of Contents](#table-of-contents)
* [Native Code in the Browser - A History](#native-code-in-the-browser-a-history)
    * [ActiveX](#activex)
    * [NaCl and PNaCl](#nacl-and-pnacl)
    * [asm.js](#asm-js)
* [What is Web Assembly](#what-is-web-assembly)

## Native Code in the Browser - A History

Perhaps unsurprisingly, web-assembly is not the first time that someone
thought "it would be cool to run native programs in a sandbox inside the
browser".  In fact, there are two major "implementations" that are widely
available right now!

### ActiveX

### NaCl and PNaCl

In 2011, Google demoed Native Client (NaCl) inside of Chrome.  NaCl is a
sandboxing architecture that allowed specially-compiled x86, x86-64, and
arm binaries to be executed by the browser inside of a web page.

When a binary was downloaded, first Chrome would verify various properties
about the instructions contained within (no system calls, strict jump
instruction alignment).  Then the program could be ran with minimal
interferance from the NaCl sandbox.  As a result, developers are able to
write chrome apps that took advantage of the performance benefits of running
natively.

Unfortunately, developers still had to produce a binary specific to each
platform that they wanted to target - x86, x86-64, and arm.  This annoyance
was the inspiration for PNaCl: Portable Native Client.  With PNaCl, a developer
compiled their application down to a platform agnostic representation which was
ahead-of-time compiled down to the native format on the target machine.

In the end, neither NaCl or PNaCl took off, mostly because other browser vendors
didn't want to implement them.

### asm.js

Javascript interpreters have been getting faster and faster ever since Google
declared a holy war on jank in the late 2000s.  Someone (TODO: who?) decided
that Javascript interpreters with their fancy jitting engines would be able to
interpret native code that had been compiled down to a low-level javascript
subset that would be immediately obvious to jit-engines to optimize.  This
person created emscripten (a fork of LLVM) which would end up being able to
compile entire game engines written in C and C++ to javascript.

Along the way, Mozilla developer (TODO: who?) figured that the output generated
by emscripten could be ahead-of-time compiled by the browser for even better
optimization opporitunities.  This was the birth of asm.js, a formalized
javascript subset that would attempt to ahead-of-time jit modules with
`"use asm";` as the first statement.

Unlike NaCl and PNaCl, asm.js saw lots of adoption!  Because it was a subset of
javascript, it worked in every browser; though it was faster in Fire Fox because
of the explicit support for ahead-of-time compilation.  Many game engines support
publishing their games to the web via emscripten and asm.js.

## What is Web-Assembly?
