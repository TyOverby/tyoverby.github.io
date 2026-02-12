---
title: Version control for everything
date: 2026-02-12
draft: true
toc: false
---

AI assisted agentic coding is everywhere. Skepticism of its value in programming contexts has
considerably faded, but non-programming use cases that aren't aren't taking off to the same degree,
and I believe that the main reason for this is that they're lacking version control.

Imagine using claude-code outside of a git repository. Even for small things like refactors, using
AI would be very stressful:

- It would be near-impossible to track the changes that the LLM made. {{< sidenote >}} Auditing the
  LLM generated code is useful in the moment to ensure that changes are reasonable before moving on
  to another task, and in the future when you want to understand why some code was written
  {{< /sidenote >}}
- The LLM could put the codebase in a bad state and you'd have no way of reverting
- Without extra work, there's no split between the "development branch" and "prod"
- You can't parallelize work by having multiple LLMs work on different branches

Even when I'm paying Claude to work on a small script, I always create a new git repo just to make
my life easier. But outside of coding, there are few services or products that offer similar
functionality and I think that this is seriously hurting AI adoption in these sectors. Let's look at
an example:

## Case study: software development outer loop

Managing the software development process is complex. We use issue trackers and pull requests to
manage work, people use email, instant messaging, and scheduled meetings. Keeping all of the
information in these channels synchronized and up to date is a full time job. In this scenario,
let's say we're concerned with the following systems:

1. github issues (read/write)
2. pull requests (read/write)
3. google calendar (read/write)
4. google docs (read/write)
5. gmail (read)
6. slack (read)

and you're interested in using an LLM to find places where some information hasn't made its way from
one service to another (e.g. update an issue with new information after an email conversation). This
task is hard in isolation (though I think that today's LLMs could do it) but _the biggest issue is
that none of these services have built-in mechanisms that would allow the LLM to propose an action
to be reviewed by a human._

### Option 1: a proxy layer

Leaving the underlying services as-is, we could write a proxy that adds a "pull requests" layer that
would allow review before publishing the changes directly. Instead of interacting with the
underlying services directly, an agent would go through this service, which would stage the change
until someone could review, approve, and publish them.

This is challenging for a few reasons:

1. Building one-off systems like this is time consuming, it's tied to the specific workflow and
   complexity grows as you need to integrate more services.
2. "Revert" is still out of reach. Underlying services don't always provide methods to undo changes.
3. Detecting and resolving merge conflicts in the underlying services isn't always possible.
4. The layer on top of the underlying services prevents you from seeing what the whole state of the
   world would look like if you were to merge the change. {{< sidenote >}} Imagine having to do code
   review, but could only look at the diff instead of being given the diff _and_ the ability to see
   the entire contents of the codebase before and after the diff is applied.{{< /sidenote >}}
5. There's no atomicity. If someone clicks the "publish" button and one change fails for some
   reason, all the previous changes to other services are already out in the world, and some of them
   can't be reverted.

### Option 2: put everything else in git

If you're ok with leaving the services entirely, you could move as much as possible into git.
{{< sidenote >}} Jane Street famously
[does code review by embedding code review comments directly in the source code as code comments](https://www.janestreet.com/tech-talks/janestreet-code-review/),
and this workflow decision makes it trivial to involve LLMs in code review because everything in the
process is tracked with version control. {{< /sidenote >}} Why not put issues alongside the
codebase? Pull request and code-review metadata in source control? Move design docs from Google Docs
to checked-in markdown? It would be ideal if all of these were stored in the same repository as the
code itself so that changes to code, issues, and docs could be made in a single atomic update
instead of having to coordinate across services.

In my view, the main obstacle here is that without serious dedication, the user experience for
humans would be massively hurt. This isn't insurmountable, but it would be a lot of work.

## A better world for LLMs is a better world for me

Although I've framed this blog post as "things that would make LLMs more useful outside of
programming", you could just as easily replace "LLM" with "Junior Developer" (or "Senior developer")
and all of the points would hold. It's not just agent-style LLMs that would benefit from this
integration, _I_ would be more productive if all of my tools had branches, version history, and
atomic changes.
