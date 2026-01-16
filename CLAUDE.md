# tyoverby.com hugo blog project

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
│   ├── pages/                  # Timeless pages (resources, documentation)
│   └── reading/                # Reading list (links to external articles)
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
│   ├── reading/
│   │   ├── single.html       # Redirect to external URL
│   │   └── list.html         # Reading list index template
│   └── index.html            # Homepage template
├── static/                    # Static assets (served at site root)
│   ├── css/                   # Stylesheets
│   ├── images/                # Images referenced in posts
│   └── ...                    # Other static files (PDFs, HTML demos, etc.)
└── public/                    # Generated site (gitignored, auto-built by CI)
```

## Content Types

The site has four types of content, each with its own purpose and style:

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

### 4. Reading List (`/reading/`)

Links to external articles and blog posts. Items link directly to external URLs (no individual pages on this site). Has its own RSS feed separate from the main feed.

**Front matter:**
```yaml
---
title: "Article Title"
date: 2025-01-16
link: "https://example.com/article"
description: "Brief summary of the article"
---
```

**URL structure:** `/reading/filename/` (redirects to external link)

**Features:**
- Titles link directly to external URLs
- External link icon displayed after title
- "added" prefix on dates to clarify it's when the link was saved
- Separate RSS feed at `/reading/index.xml`
- Excluded from main RSS feed

**When the user asks to add a link to the reading list:**

1. Fetch the URL and read the page content
2. Generate a good title from the article (or from URL if page can't be loaded)
3. Write a short summary (3 sentences max) that:
   - Does NOT editorialize or add opinions
   - Does NOT repeat claims made in the post
   - Optimizes for searchability and finding references later
   - If the page can't be loaded, omit the summary entirely
4. Use the provided "date added" if given, otherwise use current date
5. Create the file in `content/reading/` with a slug derived from the title
6. Unless specifically requested, these new files should NOT be marked as a "draft"

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

### Code Snippets from Files

You can inline code snippets from external source files using the `code-snippet` shortcode. This keeps code DRY and ensures examples stay in sync with actual source files.

**Location:** `layouts/shortcodes/code-snippet.html`

**Usage:**

1. In your source file, add marker comments around the code you want to extract:

```rust
// static/examples/demo.rs
fn example() {
    // SNIPPET_START: region_name
    let x = 42;
    println!("Value: {}", x);
    // SNIPPET_END: region_name
}
```

2. In your markdown file, use the shortcode:

```markdown
{{< code-snippet file="static/examples/demo.rs" region="region_name" lang="rust" >}}
```

**Parameters:**
- `file` - Path to source file (relative to project root)
- `region` - Name of the region to extract (must match marker comments)
- `lang` - Syntax highlighting language (rust, go, python, javascript, etc.)

**Marker format:**
- Default: `// SNIPPET_START: name` and `// SNIPPET_END: name`
- Adjust comment style for your language (e.g., `#` for Python, `/* */` for C)

**Benefits:**
- Single source of truth for code examples
- Code can be compiled/tested separately
- Automatic updates when source files change
- Clean separation of code and documentation

**Example:** See `content/posts/hugo-snippet-demo.md` and `static/examples/parser.rs`

## Common Commands

### Testing Your Changes

After making a change to hugo templates or blog content, please preview your changes and inspect the generated HTML:

**1. Build the site locally:**
```bash
hugo --buildDrafts
```

**2. Inspect generated HTML:**

The built site is in the `public/` directory. You can inspect the generated HTML:

```bash
# View a specific post's HTML
cat public/posts/my-post/index.html
```

**3. Common checks:**
- Verify code snippets extracted correctly from source files
- Ensure syntax highlighting is applied (look for `<div class="highlight">`)
- Check that images load with correct paths
- Validate math rendering (if using KaTeX)
- Confirm links work and point to correct URLs

**4. Clean rebuild:**

If you encounter issues, try a clean rebuild:
```bash
# Remove old build artifacts
rm -rf public/

# Rebuild from scratch
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

Styles are defined in the css files in `static/css`

## Custom Domain

The `CNAME` file contains `tyoverby.com` which GitHub Pages uses for custom domain configuration.
