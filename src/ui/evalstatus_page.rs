// Copyright (c) 2024-2025 Zededa, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    model::model::Model,
    traits::{IEventHandler, IPresenter, IWindow},
};

/// Format nanoseconds into a human-readable time string
fn format_duration_ns(nanos: u64) -> String {
    let secs = nanos / 1_000_000_000;
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{}d {}", days, hours % 24)
    } else if hours > 0 {
        format!("{}h {}", hours, mins % 60)
    } else if mins > 0 {
        format!("{}m {}", mins, secs % 60)
    } else {
        format!("{}s", secs)
    }
}

/// Format a countdown in seconds into a human-readable time string
fn format_countdown_secs(secs: u64) -> String {
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{}d {}h", days, hours % 24)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins % 60)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs % 60)
    } else {
        format!("{}s", secs)
    }
}

#[derive(Default)]
pub struct EvalStatusPage {}

impl EvalStatusPage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IWindow for EvalStatusPage {
    fn status_bar_tips(&self) -> Option<String> {
        Some("Evaluation Status Information - Device is in evaluation mode".to_string())
    }
}

impl IEventHandler for EvalStatusPage {
    fn handle_event(&mut self, _event: crate::events::Event) -> Option<crate::ui::action::Action> {
        None
    }
}

impl IPresenter for EvalStatusPage {
    fn render(&mut self, area: &Rect, frame: &mut Frame<'_>, model: &Rc<Model>, _focused: bool) {
        let block = Block::default()
            .title("Evaluation Status")
            .borders(Borders::ALL);

        let model_ref = model.borrow();
        
        let text = if let Some(eval_status) = &model_ref.eval_status {
            let mut lines = vec![];

            // Header info
            lines.push(Line::from(""));
            if eval_status.is_evaluation_platform {
                lines.push(Line::from(vec![
                    Span::styled(
                        "EVALUATION MODE ACTIVE",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::raw(
                        "Do not change settings or reboot your device during evaluation"
                    ),
                ]));
                lines.push(Line::from(""));
            }

            // Current Phase Info
            lines.push(Line::from(vec![
                Span::raw("Current Phase:            "),
                Span::styled(
                    eval_status.phase.as_str(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            // Slot info
            lines.push(Line::from(vec![
                Span::raw("Current Slot:             "),
                Span::styled(
                    eval_status.current_slot.as_str(),
                    Style::default().fg(Color::Cyan),
                ),
            ]));

            // Onboarding status
            if eval_status.allow_onboard {
                lines.push(Line::from(vec![
                    Span::raw("Onboarding Status:        "),
                    Span::styled(
                        "Allowed",
                        Style::default().fg(Color::Green),
                    ),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::raw("Onboarding Status:        "),
                    Span::styled(
                        "Blocked (Evaluation Running)",
                        Style::default().fg(Color::Yellow),
                    ),
                ]));
            }

            // Note section
            if !eval_status.note.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::raw("Note:                     "),
                    Span::styled(
                        eval_status.note.as_str(),
                        Style::default().fg(Color::Yellow),
                    ),
                ]));
            }

            // Test Information
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled(
                    "Test Information:",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            lines.push(Line::from(vec![
                Span::raw("Started:                  "),
                Span::styled(
                    eval_status.test_start_time.as_str(),
                    Style::default().fg(Color::Green),
                ),
            ]));

            // Test Duration with human-readable format
            let duration_str = format_duration_ns(eval_status.test_duration);
            lines.push(Line::from(vec![
                Span::raw("Test Duration:            "),
                Span::styled(
                    duration_str,
                    Style::default().fg(Color::Cyan),
                ),
            ]));

            // Reboot Countdown - with urgency coloring
            let reboot_secs = eval_status.reboot_countdown;
            let reboot_str = format_countdown_secs(reboot_secs);
            let reboot_color = if reboot_secs < 60 {
                Color::Red // Less than 1 minute - urgent
            } else if reboot_secs < 300 {
                Color::Yellow // Less than 5 minutes - warning
            } else {
                Color::Green // Safe
            };

            lines.push(Line::from(vec![
                Span::raw("Reboot Countdown:         "),
                Span::styled(
                    reboot_str,
                    Style::default()
                        .fg(reboot_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            // Inventory section
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("Inventory Collection:     "),
                if eval_status.inventory_collected {
                    Span::styled(
                        "Complete",
                        Style::default().fg(Color::Green),
                    )
                } else {
                    Span::styled(
                        "In Progress",
                        Style::default().fg(Color::Yellow),
                    )
                },
            ]));

            if !eval_status.inventory_dir.is_empty() {
                lines.push(Line::from(vec![
                    Span::raw("Inventory Directory:      "),
                    Span::styled(
                        eval_status.inventory_dir.as_str(),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));
            }

            // Last update timestamp
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("Last Updated:             "),
                Span::styled(
                    eval_status.last_updated.as_str(),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));

            lines
        } else {
            vec![Line::from(Span::styled(
                "No EvalStatus information received yet - Device is not in evaluation mode",
                Style::default().fg(Color::DarkGray),
            ))]
        };

        let paragraph = Paragraph::new(text).block(block);
        frame.render_widget(paragraph, *area);
    }
}

