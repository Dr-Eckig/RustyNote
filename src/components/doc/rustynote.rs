use leptos::prelude::*;

use crate::api::parser::Dialect;

#[component]
pub fn RustyNoteDoc() -> impl IntoView {

    let parsed_doc = Dialect::GitHub.parse_markdown_to_html(RUSTYNOTE_DOC);

    view! {
        <div class="content with-cropped-images">
            <div  
                inner_html=parsed_doc
            /> 
        </div>
    }
}

const RUSTYNOTE_DOC: &str = r#"
## RustyNote

RustyNote is a browser-based Markdown notebook powered by WebAssembly and Rust.  
It focuses on fast feedback: every formatting action updates the preview instantly so you always know how your note will look when it is published or shared.

### Highlight features

| Feature                | Description                                                                    | Preview                                                   |
| ---------------------- | ------------------------------------------------------------------------------ | --------------------------------------------------------- |
| **Selectable Dialect** | Choose between CommonMark or GitHub-Flavored Markdown for parsing.             | ![Select Dialect](resources/images/rustynote/dialect.png) |
| **Theme Toggle**       | Work with your preferred color scheme.                                         | ![Theme Toggle](resources/images/rustynote/theme.png)     |
| **Selectable Mode**    | Switch between write mode, read mode, or a combined split view.                | ![Select Mode](resources/images/rustynote/mode.png)       |
| **Format Buttons**     | Use handy formatting buttons if you're new to or unsure about markdown syntax. | ![Format Buttons](resources/images/rustynote/format.png)  |
| **Format Tables**      | Clean up messy GFM tables instantly with the “Format Tables” button.           | ![Format Tables](resources/images/rustynote/tables.png)   |
| **Copy to Clipboard**  | Copy your markdown content with a single click.                                | ![Copy](resources/images/rustynote/copy.png)              |
| **Download**           | Save your work as a markdown file directly to your device.                     | ![Download](resources/images/rustynote/download.png)      |
| **Delete Button**      | Clear all content and start fresh with one click.                              | ![Delete](resources/images/rustynote/delete.png)          |

### Shortcuts
- **Ctrl + B** – **bold**
- **Ctrl + H** – **heading**
- **Ctrl + #** – **code block**
- **Ctrl + M** – **monospace**
"#;
