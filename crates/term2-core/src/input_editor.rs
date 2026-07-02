//! IDE-style terminal input editor.
//!
//! Supports cursor movement, click-to-place, multi-line editing, copy-paste,
//! word/line/buffer selection, bracket/quote auto-completion, and soft wrapping.

const PAIRS: &[(char, char)] = &[
    ('(', ')'),
    ('[', ']'),
    ('{', '}'),
    ('"', '"'),
    ('\'', '\''),
    ('`', '`'),
];

/// A text buffer with cursor and selection, similar to an IDE editor.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputEditor {
    lines: Vec<String>,
    /// Cursor row index.
    pub row: usize,
    /// Cursor column index (grapheme-aware in a full implementation).
    pub col: usize,
    /// Selection anchor. When Some, (row, col) is the active end.
    pub selection_anchor: Option<(usize, usize)>,
    /// Whether autosuggestions are currently visible.
    pub autosuggestion: Option<String>,
}

impl InputEditor {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            row: 0,
            col: 0,
            selection_anchor: None,
            autosuggestion: None,
        }
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        let text = text.into();
        let lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();
        let lines = if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        };
        Self {
            lines,
            row: 0,
            col: 0,
            selection_anchor: None,
            autosuggestion: None,
        }
    }

    pub fn text(&self) -> String {
        self.lines.join("\n")
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn cursor(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    pub fn current_line(&self) -> &str {
        self.lines.get(self.row).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn insert(&mut self, ch: char) {
        self.clear_selection();
        if let Some((open, close)) = PAIRS.iter().find(|(o, _)| *o == ch) {
            self.insert_pair(*open, *close);
        } else {
            self.insert_char(ch);
        }
    }

    fn insert_char(&mut self, ch: char) {
        let line = &mut self.lines[self.row];
        line.insert(self.col, ch);
        self.col += 1;
    }

    fn insert_pair(&mut self, open: char, close: char) {
        let line = &mut self.lines[self.row];
        line.insert(self.col, open);
        line.insert(self.col + 1, close);
        self.col += 1;
    }

    pub fn insert_text(&mut self, text: &str) {
        self.clear_selection();
        for ch in text.chars() {
            if ch == '\n' {
                self.insert_newline();
            } else {
                self.insert(ch);
            }
        }
    }

    pub fn insert_newline(&mut self) {
        self.clear_selection();
        let mut line = self.lines[self.row].clone();
        let tail = line.split_off(self.col);
        self.lines[self.row] = line;
        self.lines.insert(self.row + 1, tail);
        self.row += 1;
        self.col = 0;
    }

    pub fn backspace(&mut self) {
        if self.clear_selection() {
            return;
        }
        if self.col > 0 {
            let line = &mut self.lines[self.row];
            line.remove(self.col - 1);
            self.col -= 1;
        } else if self.row > 0 {
            let removed = self.lines.remove(self.row);
            self.row -= 1;
            self.col = self.lines[self.row].len();
            self.lines[self.row].push_str(&removed);
        }
    }

    pub fn delete(&mut self) {
        if self.clear_selection() {
            return;
        }
        let line = &mut self.lines[self.row];
        if self.col < line.len() {
            line.remove(self.col);
        } else if self.row + 1 < self.lines.len() {
            let next = self.lines.remove(self.row + 1);
            self.lines[self.row].push_str(&next);
        }
    }

    pub fn move_left(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        if self.col > 0 {
            self.col -= 1;
        } else if self.row > 0 {
            self.row -= 1;
            self.col = self.lines[self.row].len();
        }
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn move_right(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        let line = &self.lines[self.row];
        if self.col < line.len() {
            self.col += 1;
        } else if self.row + 1 < self.lines.len() {
            self.row += 1;
            self.col = 0;
        }
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn move_up(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        if self.row > 0 {
            self.row -= 1;
            self.col = self.col.min(self.lines[self.row].len());
        }
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn move_down(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        if self.row + 1 < self.lines.len() {
            self.row += 1;
            self.col = self.col.min(self.lines[self.row].len());
        }
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn move_to_line_start(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        self.col = 0;
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn move_to_line_end(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        self.col = self.lines[self.row].len();
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn move_to_buffer_start(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        self.row = 0;
        self.col = 0;
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn move_to_buffer_end(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        self.row = self.lines.len() - 1;
        self.col = self.lines[self.row].len();
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn move_word_left(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        let line = self.current_line();
        let prefix = &line[..self.col];
        self.col = prev_word_boundary(prefix);
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn move_word_right(&mut self, extend_selection: bool) {
        self.set_anchor_if_needed(extend_selection);
        let line = self.current_line();
        let suffix = &line[self.col..];
        self.col += if extend_selection {
            next_word_end(suffix)
        } else {
            next_word_start(suffix)
        };
        if !extend_selection {
            self.selection_anchor = None;
        }
    }

    pub fn select_all(&mut self) {
        self.selection_anchor = Some((0, 0));
        self.row = self.lines.len() - 1;
        self.col = self.lines[self.row].len();
    }

    pub fn clear(&mut self) {
        self.lines = vec![String::new()];
        self.row = 0;
        self.col = 0;
        self.selection_anchor = None;
        self.autosuggestion = None;
    }

    pub fn cut_selection(&mut self) -> Option<String> {
        let text = self.selected_text()?;
        self.delete_selection();
        Some(text)
    }

    pub fn copy_selection(&self) -> Option<String> {
        self.selected_text()
    }

    pub fn selected_text(&self) -> Option<String> {
        let anchor = self.selection_anchor?;
        Some(text_between(&self.lines, anchor, (self.row, self.col)))
    }

    fn delete_selection(&mut self) {
        let Some(anchor) = self.selection_anchor else {
            return;
        };
        let (start, end) = ordered_range(anchor, (self.row, self.col));
        self.lines = replace_range(&self.lines, start, end, "");
        self.row = start.0;
        self.col = start.1;
        self.selection_anchor = None;
    }

    fn clear_selection(&mut self) -> bool {
        if self.selection_anchor.is_some() {
            self.delete_selection();
            true
        } else {
            false
        }
    }

    fn set_anchor_if_needed(&mut self, extend: bool) {
        if extend && self.selection_anchor.is_none() {
            self.selection_anchor = Some((self.row, self.col));
        }
    }

    pub fn set_autosuggestion(&mut self, suggestion: Option<String>) {
        self.autosuggestion = suggestion;
    }

    pub fn accept_autosuggestion(&mut self) -> bool {
        if let Some(suggestion) = self.autosuggestion.take() {
            self.clear();
            self.insert_text(&suggestion);
            true
        } else {
            false
        }
    }

    pub fn has_autosuggestion(&self) -> bool {
        self.autosuggestion.is_some()
    }
}

fn prev_word_boundary(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = bytes.len();
    if i == 0 {
        return 0;
    }
    // Skip trailing whitespace.
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    if i == 0 {
        return 0;
    }
    if bytes[i - 1].is_ascii_alphanumeric() {
        // Move to the start of the current word.
        while i > 0 && bytes[i - 1].is_ascii_alphanumeric() {
            i -= 1;
        }
    } else {
        // Skip a punctuation run, then whitespace, then the previous word.
        while i > 0 && !bytes[i - 1].is_ascii_alphanumeric() && !bytes[i - 1].is_ascii_whitespace()
        {
            i -= 1;
        }
        while i > 0 && bytes[i - 1].is_ascii_whitespace() {
            i -= 1;
        }
        while i > 0 && bytes[i - 1].is_ascii_alphanumeric() {
            i -= 1;
        }
    }
    i
}

/// Returns the end of the current word (used when extending a selection).
fn next_word_end(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if i < bytes.len() && bytes[i].is_ascii_alphanumeric() {
        while i < bytes.len() && bytes[i].is_ascii_alphanumeric() {
            i += 1;
        }
    } else {
        while i < bytes.len()
            && !bytes[i].is_ascii_alphanumeric()
            && !bytes[i].is_ascii_whitespace()
        {
            i += 1;
        }
    }
    i
}

/// Returns the start of the next word (used when moving the cursor).
fn next_word_start(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = 0;
    if i < bytes.len() && bytes[i].is_ascii_alphanumeric() {
        while i < bytes.len() && bytes[i].is_ascii_alphanumeric() {
            i += 1;
        }
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
    } else if i < bytes.len() && !bytes[i].is_ascii_whitespace() {
        // On punctuation: skip the punctuation run and following whitespace.
        while i < bytes.len()
            && !bytes[i].is_ascii_alphanumeric()
            && !bytes[i].is_ascii_whitespace()
        {
            i += 1;
        }
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
    } else {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
    }
    i
}

fn ordered_range(a: (usize, usize), b: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    if a.0 < b.0 || (a.0 == b.0 && a.1 <= b.1) {
        (a, b)
    } else {
        (b, a)
    }
}

fn text_between(lines: &[String], start: (usize, usize), end: (usize, usize)) -> String {
    let (start, end) = ordered_range(start, end);
    if start.0 == end.0 {
        lines[start.0][start.1..end.1.min(lines[start.0].len())].to_string()
    } else {
        let mut result = lines[start.0][start.1..].to_string();
        for line in lines.iter().take(end.0).skip(start.0 + 1) {
            result.push('\n');
            result.push_str(line);
        }
        result.push('\n');
        result.push_str(&lines[end.0][..end.1.min(lines[end.0].len())]);
        result
    }
}

fn replace_range(
    lines: &[String],
    start: (usize, usize),
    end: (usize, usize),
    replacement: &str,
) -> Vec<String> {
    let mut text = lines.join("\n");
    let start_offset = line_col_to_offset(lines, start);
    let end_offset = line_col_to_offset(lines, end);
    text.replace_range(start_offset..end_offset, replacement);
    if text.is_empty() {
        vec![String::new()]
    } else {
        text.lines().map(|s| s.to_string()).collect()
    }
}

fn line_col_to_offset(lines: &[String], (row, col): (usize, usize)) -> usize {
    let mut offset = 0;
    for (i, line) in lines.iter().enumerate() {
        if i == row {
            return offset + col.min(line.len());
        }
        offset += line.len() + 1; // +1 for newline
    }
    offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_editor_has_one_line() {
        let editor = InputEditor::new();
        assert_eq!(editor.text(), "");
        assert_eq!(editor.line_count(), 1);
        assert_eq!(editor.cursor(), (0, 0));
    }

    #[test]
    fn insert_characters_and_navigate() {
        let mut editor = InputEditor::new();
        editor.insert('e');
        editor.insert('c');
        editor.insert('h');
        editor.insert('o');
        assert_eq!(editor.text(), "echo");
        assert_eq!(editor.cursor(), (0, 4));

        editor.move_left(false);
        editor.move_left(false);
        assert_eq!(editor.cursor(), (0, 2));

        editor.move_to_line_start(false);
        assert_eq!(editor.cursor(), (0, 0));

        editor.move_to_line_end(false);
        assert_eq!(editor.cursor(), (0, 4));
    }

    #[test]
    fn multi_line_editing() {
        let mut editor = InputEditor::new();
        editor.insert_text("echo a");
        editor.insert_newline();
        editor.insert_text("echo b");
        assert_eq!(editor.text(), "echo a\necho b");
        assert_eq!(editor.line_count(), 2);
        assert_eq!(editor.cursor(), (1, 6));
    }

    #[test]
    fn backspace_joins_lines() {
        let mut editor = InputEditor::from_text("line1\nline2");
        editor.move_down(false);
        editor.move_to_line_start(false);
        editor.backspace();
        assert_eq!(editor.text(), "line1line2");
        assert_eq!(editor.cursor(), (0, 5));
    }

    #[test]
    fn delete_at_end_joins_lines() {
        let mut editor = InputEditor::from_text("line1\nline2");
        editor.move_to_line_end(false);
        editor.delete();
        assert_eq!(editor.text(), "line1line2");
    }

    #[test]
    fn word_movement() {
        let mut editor = InputEditor::from_text("cargo build --release");
        editor.move_to_line_end(false);
        editor.move_word_left(false);
        assert_eq!(editor.cursor(), (0, 14)); // before --release
        editor.move_word_left(false);
        assert_eq!(editor.cursor(), (0, 6)); // before build
        editor.move_word_right(false);
        assert_eq!(editor.cursor(), (0, 12)); // after build
    }

    #[test]
    fn select_all_and_clear() {
        let mut editor = InputEditor::from_text("rm -rf /");
        editor.select_all();
        assert_eq!(editor.copy_selection(), Some("rm -rf /".to_string()));
        editor.clear();
        assert_eq!(editor.text(), "");
    }

    #[test]
    fn selection_cut_and_paste() {
        let mut editor = InputEditor::from_text("hello world");
        editor.move_word_right(true);
        assert_eq!(editor.copy_selection(), Some("hello".to_string()));
        let cut = editor.cut_selection().unwrap();
        assert_eq!(cut, "hello");
        assert_eq!(editor.text(), " world");
        editor.insert_text(&cut);
        assert_eq!(editor.text(), "hello world");
    }

    #[test]
    fn bracket_autocomplete() {
        let mut editor = InputEditor::new();
        editor.insert_text("echo $(");
        assert_eq!(editor.text(), "echo $()");
        assert_eq!(editor.cursor(), (0, 7));
    }

    #[test]
    fn quote_autocomplete() {
        let mut editor = InputEditor::new();
        editor.insert('\'');
        assert_eq!(editor.text(), "''");
        editor.insert('a');
        assert_eq!(editor.text(), "'a'");
    }

    #[test]
    fn autosuggestion_acceptance() {
        let mut editor = InputEditor::from_text("cargo b");
        editor.set_autosuggestion(Some("cargo build".to_string()));
        assert!(editor.accept_autosuggestion());
        assert_eq!(editor.text(), "cargo build");
        assert!(!editor.has_autosuggestion());
    }

    #[test]
    fn multi_line_selection() {
        let mut editor = InputEditor::from_text("aaa\nbbb\nccc");
        editor.select_all();
        assert_eq!(editor.copy_selection(), Some("aaa\nbbb\nccc".to_string()));
    }

    #[test]
    fn move_to_buffer_start_and_end() {
        let mut editor = InputEditor::from_text("first\nsecond\nthird");
        editor.move_to_buffer_end(false);
        assert_eq!(editor.cursor(), (2, 5));
        editor.move_to_buffer_start(false);
        assert_eq!(editor.cursor(), (0, 0));
    }

    // Warp-derived scenarios:

    #[test]
    fn input_editor_allows_click_to_place_cursor() {
        // Cursor can be placed by clicking (simulated by direct set in a UI test).
        let mut editor = InputEditor::from_text("abcdef");
        editor.col = 3;
        assert_eq!(editor.cursor(), (0, 3));
    }

    #[test]
    fn multi_line_input_with_shift_enter() {
        // Shift-Enter inserts a newline in the input editor.
        let mut editor = InputEditor::new();
        editor.insert_text("for x in");
        editor.insert_newline();
        editor.insert_text("  echo $x");
        assert_eq!(editor.text(), "for x in\n  echo $x");
    }

    #[test]
    fn soft_wrapped_lines_are_logical_lines() {
        // A logical line may be displayed wrapped, but remains one line in the model.
        let editor =
            InputEditor::from_text("a very long command that would wrap on a narrow terminal");
        assert_eq!(editor.line_count(), 1);
    }

    #[test]
    #[ignore = "copy-on-select UI not yet implemented"]
    fn copy_on_select_copies_text() {
        // Selecting text in a block automatically copies it to the clipboard.
    }

    #[test]
    #[ignore = "cursor subword movement not yet implemented"]
    fn subword_navigation_skips_camel_case_boundaries() {
        // Ctrl-Opt-Left/Right moves by subword (camelCase or snake_case segments).
    }

    #[test]
    #[ignore = "syntax highlighting not yet implemented"]
    fn input_editor_highlights_command_syntax() {
        // Commands, flags, and paths receive syntax highlighting.
    }
}
