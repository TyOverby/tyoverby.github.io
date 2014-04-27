---

title: Inscribe
subtitle: A static blog generator without features
date: 2014-04-26 PST

---

I've wanted to write a blog for a while.  I'll regularly have ideas that I want
to share with others on a permanent basis, and having a nice tech-themed blog
seems to be the way to do that.  However, I'm not a fan of paying for hosting
and I'm a fan of Github Pages, so I'll be in need of  a static site generator
to make the viewable content for github to host.

Github Pages comes with its own static site generator
[Jekyll](http://jekyllrb.com/) which I did play with for a few hours of
frustratingly reading the documentation.  The truth was that I just wasn't
willing to learn a new templating language, a project structure, and a suite
of configuration settings.  While exploring other static site generators, I
came across [Wintersmith](https://github.com/jnordberg/wintersmith) which although
it probably has everything you might want in a static site generator,
doesn't really meet my definition of _simple_.

It was through looking at these existing generators that I was able to narrow my
focus on what I think my ideal blog generator would look like.  What I've made
(and will be talking about for the rest of this post) is that static-content blog
generator which I've named Inscribe.

## Hello Inscribe

For any content-focused site (blogs are an excelent subset), there are typically
3 forms of data that are most important to producing the final content.
*  Sitewide settings
*  Post specific settings
*  Post content

__Sitewide settings__ are for conveniantly storing settings like the name of your blog, or
the author.  This could also include project settings for the static-site generator to work with.
__Post specific settings__ might include the date that a post was made, the title of the post,
the author (maybe you have guest authors?) or any other metadata that you want included on a
post-by-post basis.  The __post content__ is the most obvious of the three; it's the actual words that you want
to get displayed on a page somewhere.

So how does Inscribe store and organize these three forms of data?  Lets look at a sample Inscribe directory
to find out.

```
$ tree
.
├── blog.json
└── posts
    ├── helloworld.md
    └── inscribe.md

```

As you might be able to guess, the __sitewide settings__ are stored in `blog.json` while both __post specific settings__ and
__post content__ are stored in the markdown files in the `posts` directory.

## ./blog.json
Let's look at a really simple `blog.json` file.  A basic `blog.json` file can be created from the command
line with `inscribe init`.

```json
{
    "name": "Ty's Blog",
    "subName": "musings of a madman",
    "author": "Ty Overby",
    "_outDir_": "./out/",
    "_postsDir_": "./posts/"
}
```

The keys with underscores before and after them are required for Inscribe to work.  Everything else is optional.
`_outDir_` is the directory that your rendered html will eventuall go.  `_postsDir_` is the directory in which Inscribe will look for posts.

## ./posts/some_post.md
So somehow we need to fit both the __post specific settings__ and __post content__ into a single markup file.
This is a problem already solved by [Wintersmith](https://github.com/jnordberg/wintersmith), and I liked their
solution so I stole it.  What we do is denote a section at the top of the file that consists of key-value pairs
of properties for the post.
This is what it looks like:
```markdown
---
title: Some Post Title
subtitle: A really bad subtitle
date: 2014-04-26 PST
---

# Markdown Header
The rest of your blog goes here.
```

## Static content generation (or lack thereof)
So by this point you're probably wondering how those html files that you want so bad get generated.
Well, sorry to break the news to you, but they don't.
Inscribe only goes half way.  It gathers and organizes information about the blog, and about the posts,
but it dumps that info into a single .json file and walks away.

Before you close this tab and head back to hackernews hear me out:  I don't pretend to know your needs
better than you do.  All I've done is written a pretty tiny script that gathers these data and hands
it off to you in a nice little package.  No templating language to learn, no _programming_ language to
learn; if you can read a JSON file, you can easily access the data that matters in your blog.

Ok, assuming you're still with me, when you run `inscribe generate`, it reads your `blog.json` and all the
files in `_outDir` and spews out a `bundle.json` in the working directory.  This json file contains
all the information about your blog and the posts.  Here's what it looks like when given the previous `blog.json`
and the markdown file:

```json
{
    "name": "Ty's Blog",
    "subName": "musings of a madman",
    "author": "Ty Overby",
    "_outDir_": "./out/",
    "_postsDir_": "./posts/",
    "_posts_": [{
        "title": "Some Post Title",
        "subtitle": "A really bad subtitle",
        "date": "2014-04-26 PST",
        "_file_": "some_post.md",
        "_contents_": "# Markdown Header\nThe rest of your blog goes here."
    }]
}
```
The first thing that should jump out at you is that the upper level of the structure is exactly the same
as our `blog.json` file.  The only field that has been added is `_posts_`, which is an array containing
maps for all of the blog posts.

The post object itself is also quite familiar.  Those key-value pairs defined in the top of the post are
present as key-value pairs on the post object.  The internal properties are `_file_` and `_contents_` which
contain the file name the blog was generated from and the __post content__ from that post.

There are two things to note about the value of the  `_contents_` field.

1. The header information that is used to fill out the metadata is gone.
2. The content is still in markdown format.

The first of these should be fairly unsurprising.  There's no reason we'd want that section of the file there after
the info is already extracted out nice and cleanly for you.

The second one might be a bit more surprising.  This might have been the stage where the markdown
tohtml stage occurred, but it didn't.  The truth is that Inscribe doesn't care what you write the __post content__
in.

This has a few implications.  The most important of which is that Inscribe can't apply a whitelist when searching
for certain files in the `_postsDir_` directory.  The result of this is that every file inside `_postsDir_` must
be a valid post file.  No images or misc files.


## After Inscribe

So `$ inscribe generate` produces a `bundle.json` file, but what happens after that?  I'd like to say
"this is left as an exercise to the reader", but the truth is that I've already written a proper jade-markdown
that expects __post content__ to be markdown, and requires a `post.jade` file for templating the rest of the
page.

I'm hoping that other people will create their own scripts that take `bundle.json` and do magical things with it.
But if they don't and I'm the only one that uses it, that's file with me too. `:)`
