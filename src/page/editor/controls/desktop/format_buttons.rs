use leptos::prelude::*;

use crate::{api::{markdown_formatter::format::TextFormattingType, parser::Dialect}, components::{Color, Size, State, button::{Button, format_tables::FormatTablesButton}, icons::Icon, tooltip::{Tooltip, TooltipDirection}}};

fn tooltip(tip: String, parser: Dialect) -> String {
    if let Dialect::Common = parser {
        "❌ Please Enable GitHub extension".to_string()
    } else {
        tip
    }
}

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
        <div class="is-flex" style="gap: 0.5rem">
            <Tooltip text="# Heading">
                <Button
                    icon=Icon::Heading
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Heading.apply_text_formatting())
                />
            </Tooltip>
            <FormatInlineButtons markdown parser color size disable_github_features />
            <FormatListButtons markdown parser color size disable_github_features />
            <FormatBlocksButtons markdown color size />
            <FormatUrlButtons markdown color size />
            <FormatStructureButtons markdown parser color size disable_github_features/> 
            <FormatTablesButton markdown tooltip_direction=TooltipDirection::Left />
        </div>
    }
}

#[component]
fn FormatInlineButtons(
    markdown: RwSignal<String>, 
    parser: RwSignal<Dialect>, 
    color: Color, 
    size: Size, 
    disable_github_features: Signal<State>
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
            <Tooltip text="**Bold**">
                <Button
                    icon=Icon::Bold
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "**", suffix: "**" }.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text="_Italic_">
                <Button
                    icon=Icon::Italic
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "_", suffix: "_" }.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text=Signal::derive(move || tooltip(String::from("~~CrossOut~~"), parser.get()))>
                <Button
                    icon=Icon::CrossOut
                    color
                    state=disable_github_features
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "~~", suffix: "~~" }.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text="`Inline Code`">
                <Button
                    icon=Icon::Monospace
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "`", suffix: "`" }.apply_text_formatting())
                />
            </Tooltip>
        </div>
    }
}

#[component]
fn FormatListButtons(
    markdown: RwSignal<String>,
    parser: RwSignal<Dialect>,
    color: Color, 
    size: Size, 
    disable_github_features: Signal<State>
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
            <Tooltip text="- Unordered List">
                <Button
                    icon=Icon::UnorderedList
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::LinePrefix { prefix: "- " }.apply_text_formatting())
                />
            </Tooltip>

            <Tooltip text="1. Ordered List">
                <Button
                    icon=Icon::OrderedList
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::OrderedList.apply_text_formatting())
                />
            </Tooltip>

            <Tooltip text=Signal::derive(move || tooltip(String::from("- [ ] Checkbox"), parser.get()))>
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
            <Tooltip text="```Code Block```">
                <Button
                    icon=Icon::Code
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::CodeBlock.apply_text_formatting())
                />
            </Tooltip>
        
            <Tooltip text="> Blockquote">
                <Button
                    icon=Icon::Blockquote
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::LinePrefix { prefix: "> " }.apply_text_formatting())
                />
            </Tooltip>
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
            <Tooltip text="![Image](url)">
                <Button
                    icon=Icon::Image
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "![", suffix: "](url)" }.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text="[Link](url)">
                <Button
                    icon=Icon::Link
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "[", suffix: "](url)" }.apply_text_formatting())
                />
            </Tooltip>
        </div>
    }
}

#[component]
fn FormatStructureButtons(
    markdown: RwSignal<String>, 
    parser: RwSignal<Dialect>,
    color: Color, 
    size: Size, 
    disable_github_features: Signal<State>
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
            <Tooltip text="--- Horizontal Rule">
                <Button
                    icon=Icon::Line
                    color
                    size
                    on_click=move || markdown.set(TextFormattingType::HorizontalRule.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text=Signal::derive(move ||tooltip(String::from("| Table |"), parser.get()))>
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