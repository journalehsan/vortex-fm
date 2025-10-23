// Bulk rename dialog widget for Vortex File Manager

use cosmic::{
    Element,
    widget,
    iced::{Length, Padding, Alignment},
};
use regex::Regex;
use chrono::Local;

use crate::{
    app::Message,
};

#[derive(Debug, Clone)]
pub struct BulkRenameDialog {
    pub is_open: bool,
    pub selected_files: Vec<String>,
    pub pattern: String,
    pub replacement: String,
    pub use_regex: bool,
    pub preview_results: Vec<(String, String)>, // (original, new)
    pub case_sensitive: bool,
    pub match_all: bool,
}

impl BulkRenameDialog {
    pub fn new() -> Self {
        Self {
            is_open: false,
            selected_files: Vec::new(),
            pattern: String::new(),
            replacement: String::new(),
            use_regex: false,
            preview_results: Vec::new(),
            case_sensitive: true,
            match_all: false,
        }
    }

    pub fn open(&mut self, selected_files: Vec<String>) {
        self.is_open = true;
        self.selected_files = selected_files;
        self.pattern = String::new();
        self.replacement = String::new();
        self.preview_results.clear();
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.selected_files.clear();
        self.pattern.clear();
        self.replacement.clear();
        self.preview_results.clear();
    }

    pub fn update_pattern(&mut self, pattern: String) {
        self.pattern = pattern;
        self.update_preview();
    }

    pub fn update_replacement(&mut self, replacement: String) {
        self.replacement = replacement;
        self.update_preview();
    }

    pub fn toggle_regex(&mut self) {
        self.use_regex = !self.use_regex;
        self.update_preview();
    }

    pub fn toggle_case_sensitive(&mut self) {
        self.case_sensitive = !self.case_sensitive;
        self.update_preview();
    }

    pub fn toggle_match_all(&mut self) {
        self.match_all = !self.match_all;
        self.update_preview();
    }

    fn update_preview(&mut self) {
        self.preview_results.clear();
        
        for file in &self.selected_files {
            let new_name = if self.use_regex {
                self.apply_regex_pattern(file)
            } else {
                self.apply_simple_pattern(file)
            };
            self.preview_results.push((file.clone(), new_name));
        }
    }

    fn apply_simple_pattern(&self, filename: &str) -> String {
        if self.pattern.is_empty() {
            return filename.to_string();
        }

        let pattern = if self.case_sensitive {
            self.pattern.clone()
        } else {
            self.pattern.to_lowercase()
        };

        let search_text = if self.case_sensitive {
            filename.to_string()
        } else {
            filename.to_lowercase()
        };

        if self.match_all {
            search_text.replace(&pattern, &self.replacement)
        } else {
            if let Some(pos) = search_text.find(&pattern) {
                let mut result = filename.to_string();
                result.replace_range(pos..pos + pattern.len(), &self.replacement);
                result
            } else {
                filename.to_string()
            }
        }
    }

    fn apply_regex_pattern(&self, filename: &str) -> String {
        if self.pattern.is_empty() {
            return filename.to_string();
        }

        let flags = if self.case_sensitive { "" } else { "(?i)" };
        let pattern = format!("{}{}", flags, self.pattern);

        match Regex::new(&pattern) {
            Ok(regex) => {
                if self.match_all {
                    regex.replace_all(filename, &self.replacement).to_string()
                } else {
                    regex.replace(filename, &self.replacement).to_string()
                }
            }
            Err(_) => filename.to_string(),
        }
    }

    fn apply_date_patterns(&self, text: &str) -> String {
        let now = Local::now();
        let mut result = text.to_string();

        // Date patterns
        result = result.replace("$YYYY", &now.format("%Y").to_string());
        result = result.replace("$YY", &now.format("%y").to_string());
        result = result.replace("$Y", &now.format("%Y").to_string().chars().last().unwrap_or('0').to_string());
        
        // Month patterns
        result = result.replace("$MMMM", &now.format("%B").to_string());
        result = result.replace("$MMM", &now.format("%b").to_string());
        result = result.replace("$MM", &now.format("%m").to_string());
        result = result.replace("$M", &now.format("%-m").to_string());
        
        // Day patterns
        result = result.replace("$DDDD", &now.format("%A").to_string());
        result = result.replace("$DDD", &now.format("%a").to_string());
        result = result.replace("$DD", &now.format("%d").to_string());
        result = result.replace("$D", &now.format("%-d").to_string());
        
        // Time patterns
        result = result.replace("$hh", &now.format("%H").to_string());
        result = result.replace("$h", &now.format("%-H").to_string());
        result = result.replace("$mm", &now.format("%M").to_string());
        result = result.replace("$m", &now.format("%-M").to_string());
        result = result.replace("$ss", &now.format("%S").to_string());
        result = result.replace("$s", &now.format("%-S").to_string());
        result = result.replace("$fff", &now.format("%3f").to_string());
        
        // Combined patterns
        result = result.replace("$date", &now.format("%Y-%m-%d").to_string());
        result = result.replace("$date-time", &now.format("%Y-%m-%d_%H-%M-%S").to_string());

        result
    }

    fn apply_counter_patterns(&self, text: &str, index: usize) -> String {
        let mut result = text.to_string();
        
        // Simple counter
        result = result.replace("${}", &index.to_string());
        
        // Counter with padding
        if let Some(start) = result.find("${padding=") {
            if let Some(end) = result[start..].find("}") {
                let padding_str = &result[start + 10..start + end];
                if let Ok(padding) = padding_str.parse::<usize>() {
                    let padded = format!("{:0width$}", index, width = padding);
                    result.replace_range(start..start + end + 1, &padded);
                }
            }
        }
        
        // Counter with increment
        if let Some(start) = result.find("${increment=") {
            if let Some(end) = result[start..].find("}") {
                let increment_str = &result[start + 13..start + end];
                if let Ok(increment) = increment_str.parse::<usize>() {
                    let value = index * increment;
                    result.replace_range(start..start + end + 1, &value.to_string());
                }
            }
        }
        
        // Counter with start value
        if let Some(start) = result.find("${start=") {
            if let Some(end) = result[start..].find("}") {
                let start_str = &result[start + 8..start + end];
                if let Ok(start_val) = start_str.parse::<usize>() {
                    let value = start_val + index;
                    result.replace_range(start..start + end + 1, &value.to_string());
                }
            }
        }

        result
    }

    pub fn build(&self) -> Element<'_, Message> {
        if !self.is_open {
            log::debug!("🔧 Bulk rename dialog is not open");
            return widget::text("").into();
        }
        
        log::info!("🔧 Building bulk rename dialog with {} files", self.selected_files.len());

        let title = widget::text("Bulk Rename Files")
            .size(18);

        let pattern_input = widget::text_input("Pattern (use * for wildcard)", &self.pattern)
            .on_input(|text| Message::BulkRenamePattern(text))
            .width(Length::Fill);

        let replacement_input = widget::text_input("Replacement", &self.replacement)
            .on_input(|text| Message::BulkRenameReplacement(text))
            .width(Length::Fill);

        let regex_toggle = widget::button::standard(
            if self.use_regex { "Regex: ON" } else { "Regex: OFF" }
        )
        .on_press(Message::BulkRenameToggleRegex);

        let case_toggle = widget::button::standard(
            if self.case_sensitive { "Case: ON" } else { "Case: OFF" }
        )
        .on_press(Message::BulkRenameToggleCase);

        let match_all_toggle = widget::button::standard(
            if self.match_all { "Match All: ON" } else { "Match All: OFF" }
        )
        .on_press(Message::BulkRenameToggleMatchAll);

        let controls = widget::row::with_children(vec![
            regex_toggle.into(),
            case_toggle.into(),
            match_all_toggle.into(),
        ])
        .spacing(8)
        .align_y(Alignment::Center);

        let preview_title = widget::text("Preview")
            .size(16);

        let preview_items: Vec<Element<'_, Message>> = self.preview_results
            .iter()
            .enumerate()
            .map(|(i, (original, new))| {
                let item = widget::row::with_children(vec![
                    widget::text(format!("{}. {}", i + 1, original))
                        .size(12)
                        .into(),
                    widget::text("→")
                        .size(12)
                        .into(),
                    widget::text(new)
                        .size(12)
                        .into(),
                ])
                .spacing(8)
                .align_y(Alignment::Center);

                widget::container(item)
                    .padding(Padding::from([4, 8]))
                    .into()
            })
            .collect();

        let preview_scroll = if preview_items.is_empty() {
            widget::text("No files selected or pattern is empty")
                .size(12)
                .into()
        } else {
            widget::scrollable(
                widget::column::with_children(preview_items)
                    .spacing(4)
                    .padding(8)
            )
            .height(Length::Fixed(200.0))
            .into()
        };

        let action_buttons = widget::row::with_children(vec![
            widget::button::standard("Cancel")
                .on_press(Message::BulkRenameCancel)
                .into(),
            widget::button::standard("Apply")
                .on_press(Message::BulkRenameApply)
                .into(),
        ])
        .spacing(8)
        .align_y(Alignment::Center);

        let help_text = widget::text("Patterns: * (wildcard), $date, $date-time, ${} (counter), ${padding=N} (padded counter)")
            .size(10);

        let content = widget::column::with_children(vec![
            title.into(),
            widget::Space::with_height(Length::Fixed(16.0)).into(),
            pattern_input.into(),
            widget::Space::with_height(Length::Fixed(8.0)).into(),
            replacement_input.into(),
            widget::Space::with_height(Length::Fixed(8.0)).into(),
            controls.into(),
            widget::Space::with_height(Length::Fixed(16.0)).into(),
            preview_title.into(),
            widget::Space::with_height(Length::Fixed(8.0)).into(),
            preview_scroll,
            widget::Space::with_height(Length::Fixed(16.0)).into(),
            help_text.into(),
            widget::Space::with_height(Length::Fixed(16.0)).into(),
            action_buttons.into(),
        ])
        .spacing(8)
        .padding(16)
        .align_x(Alignment::Start);

        widget::container(content)
            .width(Length::Fixed(500.0))
            .height(Length::Fixed(600.0))
            .into()
    }
}

impl Default for BulkRenameDialog {
    fn default() -> Self {
        Self::new()
    }
}
