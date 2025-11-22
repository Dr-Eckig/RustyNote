use leptos::prelude::*;

use crate::{api::{markdown_formatter::format::TextFormattingType, parser::Dialect}, components::{Color, Size, State, button::Button, icons::Icon, tooltip::Tooltip}};

#[component]
pub fn EditTextButtons(markdown: RwSignal<String>, parser: RwSignal<Dialect>) -> impl IntoView {

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
        <div class="is-flex is-justify-content-space-between" style="gap: 0.5rem">
            <Button
                icon=Icon::Heading
                color
                size
                on_click=move || markdown.set(TextFormattingType::Heading.apply_text_formatting())
            />
            <FormatInlineButtons markdown color size disable_github_features />
            <FormatListButtons markdown color size disable_github_features />
            <FormatBlocksButtons markdown color size />
            <FormatUrlButtons markdown color size />
            <FormatStructureButtons markdown color size disable_github_features/> 
        </div>
    }
}

#[component]
fn FormatInlineButtons(
    markdown: RwSignal<String>, 
    color: Color, 
    size: Size, 
    disable_github_features: Signal<State>
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
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
            <Tooltip text=String::from("❌ Please Enable GitHub extension") is_hidden=Signal::derive(move || !matches!(disable_github_features.get(), State::Disabled))>
                <Button
                    icon=Icon::CrossOut
                    color
                    state=disable_github_features
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "~~", suffix: "~~" }.apply_text_formatting())
                />
            </Tooltip>
            <Button
                icon=Icon::Monospace
                color
                size
                on_click=move || markdown.set(TextFormattingType::Inline { prefix: "`", suffix: "`" }.apply_text_formatting())
            />
        </div>
    }
}

#[component]
fn FormatListButtons(
    markdown: RwSignal<String>,
    color: Color, 
    size: Size, 
    disable_github_features: Signal<State>
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
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

            <Tooltip text=String::from("❌ Please Enable GitHub extension") is_hidden=Signal::derive(move || !matches!(disable_github_features.get(), State::Disabled))>
                <Button
                    icon=Icon::Checkbox
                    color
                    state=disable_github_features
                    size
                    on_click=move || markdown.set(TextFormattingType::LinePrefix { prefix: "- [ ] " }.apply_text_formatting())
                />
            </Tooltip>
        </div>
    }
}

#[component]
fn FormatBlocksButtons(
    markdown: RwSignal<String>, 
    color: Color, 
    size: Size, 
) -> impl IntoView {
    
    view! {
        <div class="buttons has-addons m-0">
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
        </div>
    }
}

#[component]
fn FormatUrlButtons(
    markdown: RwSignal<String>, 
    color: Color, 
    size: Size, 
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
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
        </div>
    }
}

#[component]
fn FormatStructureButtons(
    markdown: RwSignal<String>, 
    color: Color, 
    size: Size, 
    disable_github_features: Signal<State>
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
            <Button
                icon=Icon::Line
                color
                size
                on_click=move || markdown.set(TextFormattingType::HorizontalRule.apply_text_formatting())
            />
            <Tooltip text=String::from("❌ Please Enable GitHub extension") is_hidden=Signal::derive(move || !matches!(disable_github_features.get(), State::Disabled))>
                <Button
                    icon=Icon::Table
                    color
                    size
                    state=disable_github_features
                    on_click=move || markdown.set(TextFormattingType::Table.apply_text_formatting())
                />
            </Tooltip>
        </div>
    }
}
