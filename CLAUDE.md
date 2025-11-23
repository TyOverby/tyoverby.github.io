# Hugo Blog Documentation

This is a personal blog built with Hugo and deployed to GitHub Pages via GitHub Actions.

## Project Structure

```
tyoverby.github.io/
├── .github/workflows/hugo.yml  # GitHub Actions deployment workflow
├── CNAME                       # Custom domain: tyoverby.com
├── hugo.toml                   # Hugo configuration
├── content/                    # All blog content
│   ├── _index.md               # Homepage content
│   ├── posts/                  # Blog posts (markdown files)
│   ├── thoughts/               # Daily thoughts (shorter, casual posts)
│   └── pages/                  # Timeless pages (resources, documentation)
├── layouts/                    # Hugo templates
│   ├── _default/
│   │   ├── baseof.html       # Base template (includes KaTeX for math)
│   │   ├── single.html       # Individual post template
│   │   └── list.html         # List page template
│   ├── thoughts/
│   │   ├── single.html       # Individual thought template
│   │   └── list.html         # Thoughts index template
│   ├── pages/
│   │   ├── single.html       # Individual page template
│   │   └── list.html         # Pages index template
│   └── index.html            # Homepage template
├── static/                    # Static assets (served at site root)
│   ├── css/                   # Stylesheets
│   ├── images/                # Images referenced in posts
│   └── ...                    # Other static files (PDFs, HTML demos, etc.)
└── public/                    # Generated site (gitignored, auto-built by CI)
```

## Content Types

The site has three types of content, each with its own purpose and style:

### 1. Blog Posts (`/posts/`)

Formal, long-form articles with publication dates and full metadata.

**Creating a new post:**
```bash
hugo new content/posts/my-new-post.md
```

**Front matter:**
```yaml
---
title: "My Post Title"
date: 2025-01-15
draft: false
math: true        # Enable KaTeX math rendering (optional)
subtitle: "..."   # Post subtitle (optional)
author: "..."     # Author name (optional)
abstract: "..."   # HTML abstract shown before content (optional)
toc: false        # Disable table of contents (optional, default: true)
---

Your content here...
```

**URL structure:** `/posts/filename/`

### 2. Daily Thoughts (`/thoughts/`)

Shorter, more casual observations and quick notes. These appear with full content on the thoughts index page.

**Creating a new thought:**
```bash
hugo new content/thoughts/my-thought.md
```

**Front matter:**
```yaml
---
title: "Thought Title"
date: 2025-01-15
---

Quick thought or observation...
```

**URL structure:** `/thoughts/filename/`

**Features:**
- Simpler layout without table of contents
- Full content displayed on index page with proper markdown rendering
- Date displayed in shorter format (e.g., "Jan 15, 2025")

### 3. Pages (`/pages/`)

Timeless resources and documentation without publication dates. Sorted alphabetically.

**Creating a new page:**
```bash
hugo new content/pages/my-page.md
```

**Front matter:**
```yaml
---
title: "Page Title"
description: "Brief description shown in listings"  # optional
toc: false        # Disable table of contents (optional, default: true)
---

Your content here...
```

**URL structure:** `/filename/` (served at site root)

**Features:**
- No dates displayed
- Optional description field
- Table of contents support for longer pages
- Sorted by title (not date)

## Content Formatting

### Images

Reference images from the `static/` directory using absolute paths:
```markdown
![Description](/images/my-image.png)
<img src="/images/poly_ops/diagram.svg" />
```

### Math Rendering

Posts with `math: true` in front matter support LaTeX math:
- Inline math: `$x^2 + y^2 = r^2$`
- Display math: `$$ \sum_{i=0}^{n} i $$`

### Code Blocks

All content types support fenced code blocks with syntax highlighting:
````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
````

## Common Commands

### Local Development

```bash
# Start development server (includes drafts)
hugo server -D

# Start development server (published posts only)
hugo server

# Build site locally
hugo

# Build including drafts
hugo --buildDrafts
```

### Deployment

Deployment is automatic via GitHub Actions:
1. Commit changes to the master branch
2. Push to GitHub: `git push origin master`
3. GitHub Actions builds and deploys automatically
4. Site updates at https://tyoverby.com

## Configuration

### hugo.toml

Key settings:
- `baseURL` - Site URL (https://tyoverby.com/)
- `title` - Site title
- `[permalinks]` - URL structure for each content type:
  - `posts = '/posts/:filename/'`
  - `thoughts = '/thoughts/:filename/'`
  - `pages = '/:filename/'` (at site root)
- `[markup.goldmark.renderer]` - `unsafe = true` allows HTML in markdown

### GitHub Actions Workflow

Location: `.github/workflows/hugo.yml`

Triggers:
- Push to `master` or `main` branch
- Manual trigger via Actions tab

The workflow:
1. Installs Hugo v0.131.0
2. Builds site with `--gc --minify`
3. Deploys to GitHub Pages

## Styling

The site uses minimal, clean default styling defined in `layouts/_default/baseof.html`. Customize styling by:
1. Editing the `<style>` block in `baseof.html`, or
2. Adding CSS files to `static/css/` and linking them in the template

## Custom Domain

The `CNAME` file contains `tyoverby.com` which GitHub Pages uses for custom domain configuration.
