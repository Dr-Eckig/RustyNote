use leptos::prelude::*;

use crate::api::parser::Dialect;

#[component]
pub fn CommonMarkDoc() -> impl IntoView {

    let parsed_doc = Dialect::GitHub.parse_markdown_to_html(COMMON_MARK_DOC);

    view! {
        <div class="content with-cropped-images">
            <div  
                inner_html=parsed_doc
            /> 
        </div>
    }
}

const COMMON_MARK_DOC: &str = r#"
## CommonMark

CommonMark is the formal Markdown specification.  
It defines a predictable, plain-text syntax so the same note renders identically on every compliant parser.

### Core syntax reference
| Feature           | Markdown syntax                                | HTML output                                                     | Preview                                              |
| ----------------- | ---------------------------------------------- | --------------------------------------------------------------- | ---------------------------------------------------- |
| **Heading**           | `#, ##, ###, ...`                              | `<h1>…</h1>`, `<h2>…</h2>`                                      | ![Headline](resources/images/commonmark/heading.png)      |
| **Bold**              | `**text**`                                     | `<strong>text</strong>`                                         | **text**                                             |
| **Italic**            | `*text*` or `_text_`                           | `<em>text</em>`                                                 | *text*                                               |
| **Unordered list**    | `- item` or `* item`                           | `<ul><li>…</li></ul>`                                           | - item                                               |
| **Ordered list**      | `1. item`                                      | `<ol><li>…</li></ol>`                                           | 1. item                                              |
| **Link**              | `[Label](https://example.com)`                 | `<a href="https://example.com">Label</a>`                       | [Label](https://example.com)                         |
| **Image**             | `![alt](https://picsum.photos/200)`            | `<img alt="alt" src="https://picsum.photos/200" />`             | ![alt](https://picsum.photos/200)                    |
| **Inline code**       | `` `Code` ``                                   | `<code>Code</code>`                                             | `Code`                                               |
| **Fenced code block** | ` ```rust `<br>` println!("Hi!"); `<br>` ``` ` | `<pre><code class="language-rust">println!("Hi");</code></pre>` | ![Codeblock](resources/images/commonmark/codeblock.png)   |
| **Blockquote**        | `> quoted text`                                | `<blockquote>…</blockquote>`                                    | ![Blockquote](resources/images/commonmark/blockquote.png) |
| **Horizontal rule**   | `---` or `***`                                 | `<hr />`                                                        | ------------                                         |

**Tip:** CommonMark ignores leading/trailing spaces within blocks, so focus on the markers (`#`, `>`, ``` ````, etc.) to control structure.
"#;
