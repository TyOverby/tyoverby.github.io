---
title: Version control for everything
date: 2026-02-12
draft: true
---

AI assisted agentic coding is everywhere. Skepticism of its value in programming contexts has
largely disappeared, but non-programming use-cases aren't taking off to the same degree, and I
believe that the main reason for this is that software development has version control.

Imagine using claude-code outside of a git repository. Even for small things like refactors, using
AI would be very stressful:

- It would be near-impossible to track the changes that the LLM made. Auditing the LLM generated
  code is useful
    - in the moment to ensure that changes are reasonable before moving on to another task
    - and in the future when you want to understand why some code was written
- The LLM could put the codebase in a bad state and you'd have no way of reverting
- Without extra work, there's not split between the "development branch" and "prod"

Even when I'm paying Claude to work on a small script, I always mint a new git repo just to make my
life easier. But outside of coding, there are scant few services or products that offer similar
workflow and I think that this is seriously hurting AI adoption in these sectors. Let's look at an
example:

## An example

> Managing the software development process is complex. We use issue trackers and pull requests to
> manage work, people use email, instant messaging, and scheduled meetings to discuss them. Keeping
> all of the information in these channels synchronized and up to date is a full time job.

In this scenario, let's say we're concerend with the following systems:

1. github issues (read/write)
2. pull requests (read/write)
3. google calenar (read/write)
4. gmail (read)
5. slack (read)

and you're interested in using an LLM to find places where some information hasn't made its way from
one service to another (e.g. update an issue with new information after an email conversation). This
task is hard in isolation (though I think that todays LLMs could do it) but the biggest issue is
that none of these services have built-in mechanisms that would allow the LLM to propose an action
to be reviewed by a human.

In this case, I think we're left with one option: write a bespoke layer on top of these services
specifically for the agentic workflow. Instead of interacting with the underlying services directly,
an agent would go through your proxy, which would stage the change until someone could review and
dispatch them.

This is challenging for a few reasons:

1. Building one-off systems like this is time consuming, it's tied to the specific workflow and
   complexity grows as you need to integrate more services.
2. "revert" is still out of reach. Underlying services don't always provide methods to undo changes.
3. Detecting and resolving merge conflicts in the underlying services isn't always possible.
4. The layer on top of the underlying services prevents you from seeing what the whole state of the
   world would look like if you were to merge the change. {{< sidenote >}} Imagine having to do code
   review, but could only look at the diff instead of being given the diff _and_ the ability to see
   the entire contents of the codebase before and after the diff is applied.{{< / sidenote >}}

## A weak propsal

As an industry, what could we do to get these systems more interoperable? One option is to move as
much as possible into git as possible. {{< sidenote >}} Jane Street famously
[does code review by embedding code review comments directly in the source code as code comments](https://www.janestreet.com/tech-talks/janestreet-code-review/),
and this workflow decision makes it trivial to involve LLMs in code review because everything in the
process is tracked with version control. {{ < / sidenote > }} Why not put issues alongside the
codebase? Pull request metadata in source control? Move design docs from Google Docs to checked-in
markdown? It would be ideal if all of these were stored in the same repository as the code itself so
that changes to code, issues, and docs could be made in a single atomic update instead of having to
coordinate across services.

In my view, the main obstacle here is that without serious dedication, the user experience for
humans would be massively hurt. This isn't insurmountable, but it would be a lot of work.
