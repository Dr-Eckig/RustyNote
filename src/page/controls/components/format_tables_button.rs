use leptos::prelude::*;
use markdown_table_formatter::format_tables;

use crate::components::{Color, Size, State, button::Button, icons::Icon, tooltip::{Tooltip, TooltipDirection}};

#[component]
pub fn FormatTablesButton(
    markdown: RwSignal<String>, 
    #[prop(into)] tooltip_direction: Signal<TooltipDirection>
) -> impl IntoView {

    let state = Signal::derive(move || {
        if contains_markdown_table(&markdown.get()) { 
            State::Normal 
        } else { State::Disabled }
    });

    let tooltip = Signal::derive(move || {
        if contains_markdown_table(&markdown.get()) {
            "Format Tables"
        } else {
            "❌ This option will be enabled when a markdown table is inserted"
        }
    });

    view! {
        <Tooltip text=tooltip direction=tooltip_direction>
            <div class="pl-2 is-relative">
                <Button 
                    text="Format Tables"
                    icon=Icon::AlignJusitify
                    color=Color::Primary
                    size=Size::Small
                    state
                    on_click=move || {
                        markdown.update(|md| {
                            *md = format_tables(md.clone())
                        })
                    }
                />
            </div>
        </Tooltip>
    }
}

fn contains_markdown_table(md: &str) -> bool {
    let mut in_code_block = false;
    let lines: Vec<&str> = md.lines().collect();

    for i in 0..lines.len() {
        let line = lines[i];

        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block { continue; }

        if is_table_header(line) && i + 1 < lines.len() && is_table_divider(lines[i + 1]) {
            return true;
        }
    }
    false
}

fn is_table_header(line: &str) -> bool {
    let l = line.trim();
    let pipe_count = l.matches('|').count();
    pipe_count >= 1 && l.chars().any(|c| c != '|' && !c.is_whitespace())
}

fn is_table_divider(line: &str) -> bool {
    let l = line.trim();
    if !l.contains('-') || !l.contains('|') { return false; }
    l.chars().all(|c| matches!(c, '-' | ':' | '|' | ' ' | '\t'))
}
