# typeset
Quick and dirty static site generator
## How to use
1. Create a folder where you will store your markdown and configuration files. One good option is to use whatever folder you're gonna store your output posts in and create a subdirectory.
2. In that folder, create the following files:
- `typeset.toml`
- `index.html`
- `template.html`
3. In `typeset.toml`, enter the following keys.

| Key            | Value                                                                                                         |
|----------------|---------------------------------------------------------------------------------------------------------------|
| schema_version | For Typeset 0.1.0, this should be 1.                                                                          |
| name           | The name of the blog. Currently unused.                                                                       |
| index          | The filename of the index page. Should be `index.html`.                                                       |
| template       | The filename of the template. Should be `template.html`.                                                      |
| page_title     | The pattern to use for page titles. Currently unused.                                                         |
| input          | Regex to select pages to input as markdown. I recommend `.md#`.                                               |
| output         | The output directory for the index, template, and posts. If in a subdirectory 1 level deep, use "..".         |
| ref_from_index | Currently unused, but must be set.                                                                            |
| time_format    | Format string for the time. See [this page](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html). |

4. In `index.html`, create HTML for the page which will show a list of links to all posts. Create only 1 link to a post, and instead of an `<a>` element to link to the post, type the following:
```html
<meta typeset="index-entry" content="n"/>
```
where the n in the value of the `content` field is equal to how many levels up is the unit that should be repeated per post.

5. In `template.html`, create HTML for an individal post. In lieu of the actual name and contents of the post, use the following tags instead:

| Key                                          | Value                                                                                                    |
|----------------------------------------------|----------------------------------------------------------------------------------------------------------|
| `<meta typeset="page-title" content="..."/>` | Use in place of a `<title>` element. In the content attribute, a $ will be replaced with the page title. |
| `<meta typeset="title"/>`                    | The page title, in text form.                                                                            |
| `<meta typeset="date"/>`                     | The post's publication date.                                                                             |
| `<meta typeset="body"/>`                     | The text component of the page, in text form. **Do not use a `<body>` tag!!**                            |

6. Create a markdown file. Before you begin writing, insert the following lines (with the ... changed based on your needs, but with the quotes still present):

| Key     | Value                                                                                        |
|-------------------|------------------------------------------------------------------------------------|
| title = "..."     | The title of the page.                                                             |
| published = "..." | The publication date of the page. MUST be in the format specified in typeset.toml! |

7. Insert line breaks, and begin writing.
8. Once you're done writing all your posts, simply enter the directory containing your posts, template, index, and typeset.toml files, and run `typeset`.
9. Your index in HTML form as well as all your posts should come out nicely!
10. If they don't, it's probably a bug. Open an issue, if you would.
