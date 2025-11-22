use leptos::prelude::*;

use crate::{api::{markdown_formatter::format::TextFormattingType, parser::Dialect}, components::{Color, Size, State, button::Button, icons::Icon}};

#[component]
pub fn MobileSidebar(markdown: RwSignal<String>, parser: RwSignal<Dialect>, sidebar_open: RwSignal<bool>) -> impl IntoView {
    
    let disable_github_features = Signal::derive(move || {
        if let Dialect::Common = parser.get() {
            State::Disabled
        } else {
            State::Normal
        }
    });
    let size = Size::Small;
    let color = Color::White;

    view! { 
        <div class="column is-narrow is-hidden-tablet px-0">
            <div class="sidebar is-flex is-flex-direction-column is-justify-content-space-evenly pr-2" class:is-active=move || sidebar_open.get()>
                <Button
                    icon=Icon::Heading
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Heading.apply_text_formatting())
                />
                <Button
                    icon=Icon::Bold
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "**", suffix: "**" }.apply_text_formatting())
                />
                <Button
                    icon=Icon::Italic
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "_", suffix: "_" }.apply_text_formatting())
                />
                <Button
                    icon=Icon::CrossOut
                    color
                    state=disable_github_features
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "~~", suffix: "~~" }.apply_text_formatting())
                />
                <Button
                    icon=Icon::Monospace
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "`", suffix: "`" }.apply_text_formatting())
                />
                <Button
                    icon=Icon::UnorderedList
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::LinePrefix { prefix: "- " }.apply_text_formatting())
                />
                <Button
                    icon=Icon::OrderedList
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::OrderedList.apply_text_formatting())
                />
                <Button
                    icon=Icon::Checkbox
                    color
                    state=disable_github_features
                    size
                    on_click=move || markdown.set(TextFormattingType::LinePrefix { prefix: "- [ ] " }.apply_text_formatting())
                />
                <Button
                    icon=Icon::Code
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::CodeBlock.apply_text_formatting())
                />
                <Button
                    icon=Icon::Blockquote
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::LinePrefix { prefix: "> " }.apply_text_formatting())
                />
                <Button
                    icon=Icon::Image
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "![", suffix: "](url)" }.apply_text_formatting())
                />
                <Button
                    icon=Icon::Link
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "[", suffix: "](url)" }.apply_text_formatting())
                />
                <Button
                    icon=Icon::Line
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::HorizontalRule.apply_text_formatting())
                />
                <Button
                    icon=Icon::Table
                    color
                    size
                    state=disable_github_features
                    on_click=move || markdown.set(TextFormattingType::Table.apply_text_formatting())
                />
            </div>
        </div>
    }
}