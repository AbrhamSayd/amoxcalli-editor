use std::{ops::Range};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;
#[derive(Copy, Clone)]
enum GraphemeWidth {
    Half,
    Full,
}
impl GraphemeWidth {
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2),
        }
    }
}

struct TextFragment {
    grapheme: String,
    render_width: GraphemeWidth,
    replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = line_str.graphemes(true).map(|g| {
            let unicode_width = g.width();
            let render_width = match unicode_width {
                0 | 1 => GraphemeWidth::Half,
                _ => GraphemeWidth::Full,
            };

            let replacement = match unicode_width {
                0 => Some('�'), // Replacement character for non-printable graphemes
                _ => None,
            };

            TextFragment {
                grapheme: g.to_string(),
                render_width,
                replacement,
            }
        })
        .collect();
        Self { fragments }
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_index: usize) -> usize {
       self.fragments
       .iter()
       .take(grapheme_index)
       .map(|fragment| match fragment.render_width{
        GraphemeWidth::Half => 1,
        GraphemeWidth::Full => 2,
    })
    .sum()
    }

    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }

        let mut result = String::new();
        let mut current_pos = 0;

        for fragment in &self.fragments {
            let fragment_end = fragment.render_width.saturating_add(current_pos);
            if current_pos >= range.end {
                break;
            }
            if fragment_end > range.start {
                if fragment_end > range.end || current_pos < range.start {
                    result.push_str("...");
                } else if let Some(char) = fragment.replacement {
                    result.push(char);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }
            current_pos = fragment_end;
        }
        result
    }
}
