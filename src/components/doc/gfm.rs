use leptos::prelude::*;

use crate::api::parser::Dialect;

#[component]
pub fn GitHubExtensionDoc() -> impl IntoView {

    let parsed_doc = Dialect::GitHub.parse_markdown_to_html(GFM_DOC);

    view! {
        <div class="content with-cropped-images">
            <div  
                inner_html=parsed_doc
            /> 
        </div>
    }
}
const GFM_DOC: &str = r#"
## GitHub Flavored Markdown (GFM)

GFM extends CommonMark with syntax that GitHub uses for READMEs, issues, and discussions.  
When you switch the editor to the GitHub dialect, RustyNote enables these extra elements.

### What does GFM add?

| Feature                  | Markdown syntax                                                                                  | HTML output                                                     | Preview                                                                                              |
| ------------------------ | ------------------------------------------------------------------------------------------------ | --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| **Strikethrough**            | `~~text~~`                                                                                       | `<del>text</del>`                                               | ~~text~~                                                                                             |
| **Task Lists**               | `- [ ] Planned` <br> `- [x] Completed`                                                           | `<ul><li><input disabled type="checkbox">...</li></ul>`         | ![Checkbox](resources/images/gfm/checkbox1.png)<br>![Checkbox](resources/images/gfm/checkbox2.png)             |
| **Tables**                   | ![Table](resources/images/gfm/table_syntax1.png)<br>![Table](resources/images/gfm/table_syntax2.png) | `<table><thead>…</thead><tbody>…</tbody></table>`               | ![Table](resources/images/gfm/table_rendered1.png)<br>![Table](resources/images/gfm/table_rendered2.png) |
| **Automatic Link Detection** | `https://example.com`                                                                            | `<a href="https://example.com">https://example.com</a>`         | https://example.com                                                                                  |
| **Code Highlighting**        | ` ```rust `<br>` println!("Hi!"); `<br>` ``` `                                                   | `<pre><code class="language-rust">println!("Hi");</code></pre>` | ![Syntax Highlighting](resources/images/gfm/highlighting.png)                                                        |


### Usage hints
1. Combine task lists with ordered lists if you need numbered steps that can also be checked off.
2. Tables accept alignment markers (`:-`, `-:`) in the separator row to control text alignment.
3. Everything in GFM is backwards compatible with CommonMark, so you can safely mix both and fall back to pure CommonMark when exporting elsewhere.
"#;
