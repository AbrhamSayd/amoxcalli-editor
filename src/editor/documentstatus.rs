#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    pub total_lines: usize,
    pub current_line_index: usize,
    pub is_modified: bool,
    pub file_name: String,
}

impl DocumentStatus {
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("(modified)")
        } else {
            String::new()
        }
    }
    pub fn line_count_to_string(&self) -> String {
        let line_count = if self.total_lines == 0 { 1 } else { self.total_lines };
        format!("{} lines", line_count)
    }
    pub fn position_indicator_to_string(&self) -> String {
        let total = if self.total_lines == 0 { 1 } else { self.total_lines };
        format!(
            "{}/{}",
            self.current_line_index.saturating_add(1),
            total
        )
    }
}