---
title: Umm... actually... that's not a good heuristic for list sortedness
date: 2018-11-26
summary: Um Actually has a broken scoring system, but we can fix it!
draft: true
---

Recently I've been watching a lot of [Um Actually](TODO), a show on
[dropout.tv](dropout.tv) that pits comedians and nerd celebreties against one
another in a trivia gameshow format.  The show is really good, but
occasionally, they do something that _really_ irks me: a common format for the
trivia question will have the contestents attempt to put a list in order.  For
example, "sort these books by release date" or "order these movies by box
office revenue".  The prompts are fun, but the way that the results are scored
is infuritating: They **count all of the items that are in the right slot when
compared to the correctly sorted list**. higher scores are better, and the
contestent with the highest score wins.

As a judge for how sorted a list is, this is basically as bad as you can get.
Sequences that a viewer would intuit as "mostly sorted" will regularly score 0,
and horribly sorted list will occasionally win points by pure chance. Let's
look at an example:

> Sort the Harry Potter novels by release date:
> 
> +------------+----------+----------+
> | True Order | Person A | Person B |
> +------------+----------+----------+
> | HP 1       | HP 7     | HP 7     |
> | HP 2       | HP 1     | HP 6     |
> | HP 3       | HP 2     | HP 5     |
> | HP 4       | HP 3     | HP 4     |
> | HP 5       | HP 4     | HP 3     |
> | HP 6       | HP 5     | HP 2     |
> | HP 7       | HP 6     | HP 1     |
> +------------+----------+----------+

Person A has almost all of the list in the correct order, but accidentally put
the final novel at the front.  Person B has it _all_ wrong though - the list is
fully reversed!  You might think that Person A should win, but since none of
their answers lined up with the correct ordering, they receive 0 points, while
person B will get 1 point because the fourth element happened to end up aligned
with the right answer.

This is an injustice, and I'm going to _get in the comments_ to fix it.

# Defining "mostly sorted"

It's surprisingly hard to come up with a good way to quantify _how sorted_ 
a list is, but I think that a good metric will measure how close a candidate
list is to the properly sorted version, and the best way to do that is measure
how many modifications to the candidate are needed in order to fully sort it.

I think that this approach is intuitive.  If I gave you a hand of playing cards
and asked you to sort them, you'd probably look through it and then start
pulling cards out and putting them in the place where they should go.  If the
hand was already mostly sorted, it would require only a few changes, but if the
hand was randomized, then you'd be doing a lot of re-organizing.

In pseudo-code:

```python
def how_badly_sorted(list):
  steps = 0
  while not is_sorted(list):
    item = remove_an_item(list)
    put_it_back_somewhere_else(list, item)
    steps += 1
  return steps
```

The trouble with this algorithm is that `remove_an_item` and `put_it_back_somewhere_else`
somehow need to know _which_ item to remove and _where to put it back_ in order
to minimize the number of steps that it takes to run.  This is a problem for me 
as the algorithm developer, but it's also a problem for a game show host!
Computing the minimal amount of operations that will transform one list into
another isn't something that you can pause the action to do... so I'm going to
disqualify it, but use it as a baseline to judge the other heuristics against.

For what it's worth, I've implemented a fast-enough version of this using A* pathfinding,
but maybe there's a more clever version - if you come up with one, send me an email!

# (non) option 1: score by exact placement

"Score by exact placement" is the ranking system currently in use by Umm Actually.
For each element in the candidate list, you receive a point if the index of that 
item matches the index of that item in the sorted list.

> The code for this implementation (and all future implementations)
> operates on a list of non-duplicate inegers whose value ranges from `0`
> (inclusive) to `len` (exclusive).  This means that each element in the list is
> a number whose value is the idex that it should be in the sorted list,
> which simplifies the implementations of these algorithms a lot!

{{< code-snippet file="rust/how_sorted/src/lib.rs" region="score_by_exact_placement" lang="rust" >}}

