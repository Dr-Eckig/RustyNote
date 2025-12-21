use leptos::prelude::*;

use crate::{api::{markdown_formatter::format::TextFormattingType, parser::Dialect}, components::{Color, Size, State, button::{Button, format_tables::FormatTablesButton}, icons::Icon, tooltip::{Tooltip, TooltipDirection}}};
use crate::Mode; 

#[component]
pub fn EditTextButtons(markdown: RwSignal<String>, parser: RwSignal<Dialect>, mode: RwSignal<Mode>) -> impl IntoView {

    let state = move |is_github_feature: bool| Signal::derive(move || {
        if let Mode::Read = mode.get() {
            State::Disabled
        } else if let Dialect::Common = parser.get() && is_github_feature {
            State::Disabled
        } else {
            State::Normal
        }
    });

    let tooltip = move |tip: String, is_github_feature: bool| Signal::derive(move || {
        if let Mode::Read = mode.get() {
            String::from("❌ The Format Buttons are not available in Read Mode")
        } else if let Dialect::Common = parser.get() && is_github_feature {
            String::from("❌ Please Enable GitHub extension")
        } else {
            Clone::clone(&tip)
        }
    });

    let size = Size::Small;
    let color = Color::White;

    view! {
        <div class="is-flex" style="gap: 0.5rem">
            <Tooltip text=tooltip(String::from("# Heading"), false)>
                <Button
                    aria_label=String::from("Heading")
                    icon=Icon::Heading
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::Heading.apply_text_formatting())
                />
            </Tooltip>
            <FormatInlineButtons markdown color size state tooltip />
            <FormatListButtons markdown color size state tooltip />
            <FormatBlocksButtons markdown color size state tooltip />
            <FormatUrlButtons markdown color size state tooltip />
            <FormatStructureButtons markdown color size state tooltip /> 

            <FormatTablesButton markdown tooltip_direction=TooltipDirection::Left />
        </div>
    }
}

#[component]
fn FormatInlineButtons(
    markdown: RwSignal<String>, 
    color: Color, 
    size: Size, 
    state: impl Send + Fn(bool) -> Signal<State> + Clone + Copy + 'static,
    tooltip: impl Send + Fn(String, bool) -> Signal<String> + Clone + Copy + 'static,
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
            <Tooltip text=tooltip(String::from("**Bold**"), false)>
                <Button
                    aria_label=String::from("Bold")
                    icon=Icon::Bold
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "**", suffix: "**" }.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text=tooltip(String::from("_Italic_"), false)>
                <Button
                    aria_label=String::from("Italic")
                    icon=Icon::Italic
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "_", suffix: "_" }.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text=tooltip(String::from("~~CrossOut~~"), true)>
                <Button
                    aria_label=String::from("Cross Out")
                    icon=Icon::CrossOut
                    color
                    size
                    state=state(true)
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "~~", suffix: "~~" }.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text=tooltip(String::from("`Inline Code`"), false)>
                <Button
                    aria_label=String::from("Inline Code")
                    icon=Icon::Monospace
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "`", suffix: "`" }.apply_text_formatting())
                />
            </Tooltip>
        </div>
    }
}

#[component]
fn FormatListButtons(
    markdown: RwSignal<String>,
    color: Color, 
    size: Size, 
    state: impl Send + Fn(bool) -> Signal<State> + Clone + Copy + 'static,
    tooltip: impl Send + Fn(String, bool) -> Signal<String> + Clone + Copy + 'static,
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
            <Tooltip text=tooltip(String::from("- Unordered List"), false)>
                <Button
                    aria_label=String::from("Unordered List")
                    icon=Icon::UnorderedList
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::LinePrefix { prefix: "- " }.apply_text_formatting())
                />
            </Tooltip>

            <Tooltip text=tooltip(String::from("1. Ordered List"), false)>
                <Button
                    aria_label=String::from("Ordered List")
                    icon=Icon::OrderedList
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::OrderedList.apply_text_formatting())
                />
            </Tooltip>

            <Tooltip text=tooltip(String::from("- [ ] Checkbox"), true)>
                <Button
                    aria_label=String::from("Checkbox")
                    icon=Icon::Checkbox
                    color
                    size
                    state=state(true)
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
    state: impl Send + Fn(bool) -> Signal<State> + Clone + Copy + 'static,
    tooltip: impl Send + Fn(String, bool) -> Signal<String> + Clone + Copy + 'static,
) -> impl IntoView {
    
    view! {
        <div class="buttons has-addons m-0">
            <Tooltip text=tooltip(String::from("```Code Block```"), false)>
                <Button
                    aria_label=String::from("Code Block")
                    icon=Icon::Code
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::CodeBlock.apply_text_formatting())
                />
            </Tooltip>
        
            <Tooltip text=tooltip(String::from("> Blockquote"), false)>
                <Button
                    aria_label=String::from("Blockquote")
                    icon=Icon::Blockquote
                    color
                    size
                    state=state(false)
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
    state: impl Send + Fn(bool) -> Signal<State> + Clone + Copy + 'static,
    tooltip: impl Send + Fn(String, bool) -> Signal<String> + Clone + Copy + 'static,
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
            <Tooltip text=tooltip(String::from("![Image](url)"), false)>
                <Button
                    aria_label=String::from("Image")
                    icon=Icon::Image
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "![", suffix: "](url)" }.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text=tooltip(String::from("[Link](url)"), false)>
                <Button
                    aria_label=String::from("Link")
                    icon=Icon::Link
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::Inline { prefix: "[", suffix: "](url)" }.apply_text_formatting())
                />
            </Tooltip>
        </div>
    }
}

#[component]
fn FormatStructureButtons(
    markdown: RwSignal<String>, 
    color: Color, 
    size: Size, 
    state: impl Send + Fn(bool) -> Signal<State> + Clone + Copy + 'static,
    tooltip: impl Send + Fn(String, bool) -> Signal<String> + Clone + Copy + 'static,
) -> impl IntoView {

    view! {
        <div class="buttons has-addons m-0">
            <Tooltip text=tooltip(String::from("--- Horizontal Rule"), false)>
                <Button
                    aria_label=String::from("Horizontal Rule")
                    icon=Icon::Line
                    color
                    size
                    state=state(false)
                    on_click=move || markdown.set(TextFormattingType::HorizontalRule.apply_text_formatting())
                />
            </Tooltip>
            <Tooltip text=tooltip(String::from("| Table |"), true)>
                <Button
                    aria_label=String::from("Table")
                    icon=Icon::Table
                    color
                    size
                    state=state(true)
                    on_click=move || markdown.set(TextFormattingType::Table.apply_text_formatting())
                />
            </Tooltip>
        </div>
    }
}