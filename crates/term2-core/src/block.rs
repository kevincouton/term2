//! Warp-style terminal blocks: group each command with its output.
//!
//! Blocks are the fundamental unit for organizing terminal output. Every command
//! and its output is grouped into a single Block that can be copied, searched,
//! filtered, bookmarked, shared, and navigated independently.

use serde::{Deserialize, Serialize};

/// Identifier for a block within a session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockId(pub String);

impl BlockId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Execution status of a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockStatus {
    /// Command is still running.
    Running,
    /// Command exited successfully.
    Completed,
    /// Command exited with a non-zero status.
    Failed,
    /// Block was cancelled (e.g. Ctrl-C).
    Cancelled,
}

/// A single terminal block grouping a command with its output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: BlockId,
    /// The command line as typed by the user.
    pub command: String,
    /// Working directory when the command was invoked.
    pub cwd: String,
    /// Raw output bytes produced by the command.
    pub output: Vec<u8>,
    /// Human-readable rendered output lines.
    pub output_lines: Vec<String>,
    /// Exit code, if the command has finished.
    pub exit_code: Option<i32>,
    pub status: BlockStatus,
    /// Whether the block is bookmarked.
    pub bookmarked: bool,
    /// Optional metadata tags.
    pub tags: Vec<String>,
    /// Start timestamp (unix millis).
    pub started_at: u64,
    /// End timestamp (unix millis), if finished.
    pub ended_at: Option<u64>,
}

impl Block {
    pub fn new(id: BlockId, command: impl Into<String>, cwd: impl Into<String>) -> Self {
        Self {
            id,
            command: command.into(),
            cwd: cwd.into(),
            output: Vec::new(),
            output_lines: Vec::new(),
            exit_code: None,
            status: BlockStatus::Running,
            bookmarked: false,
            tags: Vec::new(),
            started_at: now_millis(),
            ended_at: None,
        }
    }

    /// Append raw output bytes to the block and update rendered lines.
    pub fn append_output(&mut self, data: &[u8]) {
        self.output.extend_from_slice(data);
        // Keep a simple line-based representation for tests and search.
        let text = String::from_utf8_lossy(data);
        for line in text.split_inclusive('\n') {
            if let Some(last) = self.output_lines.last_mut() {
                if !last.ends_with('\n') {
                    last.push_str(line);
                    continue;
                }
            }
            self.output_lines.push(line.to_string());
        }
    }

    /// Mark the block as finished.
    pub fn finish(&mut self, exit_code: i32) {
        self.exit_code = Some(exit_code);
        self.status = if exit_code == 0 {
            BlockStatus::Completed
        } else {
            BlockStatus::Failed
        };
        self.ended_at = Some(now_millis());
    }

    pub fn cancel(&mut self) {
        self.status = BlockStatus::Cancelled;
        self.ended_at = Some(now_millis());
    }

    pub fn toggle_bookmark(&mut self) {
        self.bookmarked = !self.bookmarked;
    }

    /// Copy the command text to clipboard representation.
    pub fn copy_command(&self) -> String {
        self.command.clone()
    }

    /// Copy the output text.
    pub fn copy_output(&self) -> String {
        String::from_utf8_lossy(&self.output).to_string()
    }

    /// Copy both command and output with a separator.
    pub fn copy_block(&self) -> String {
        format!("{}\n{}\n", self.command, self.copy_output())
    }

    /// Search the block output for a query (case-insensitive).
    pub fn output_contains(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.output_lines
            .iter()
            .any(|line| line.to_lowercase().contains(&query))
    }

    /// The first line of output, if any.
    pub fn first_output_line(&self) -> Option<&str> {
        self.output_lines.first().map(|s| s.trim_end_matches('\n'))
    }

    /// Scroll target: line index where the command output begins.
    pub fn output_start_line(&self) -> usize {
        // In a real renderer this would account for the command header.
        0
    }
}

/// A collection of blocks for a single terminal session.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BlockStore {
    blocks: Vec<Block>,
    selected_index: Option<usize>,
    bookmarks: Vec<BlockId>,
}

impl BlockStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Block> {
        self.blocks.get_mut(index)
    }

    pub fn find_by_id(&self, id: &BlockId) -> Option<usize> {
        self.blocks.iter().position(|b| b.id == *id)
    }

    pub fn select(&mut self, index: usize) -> bool {
        if index < self.blocks.len() {
            self.selected_index = Some(index);
            true
        } else {
            false
        }
    }

    pub fn selected(&self) -> Option<&Block> {
        self.selected_index.and_then(|i| self.blocks.get(i))
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn select_next(&mut self) {
        if let Some(i) = self.selected_index {
            if i + 1 < self.blocks.len() {
                self.selected_index = Some(i + 1);
            }
        } else if !self.blocks.is_empty() {
            self.selected_index = Some(0);
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(i) = self.selected_index {
            if i > 0 {
                self.selected_index = Some(i - 1);
            }
        }
    }

    pub fn clear(&mut self) {
        self.blocks.clear();
        self.selected_index = None;
        self.bookmarks.clear();
    }

    /// Return indices of blocks whose command or output matches the query.
    pub fn search(&self, query: &str) -> Vec<usize> {
        let query = query.to_lowercase();
        self.blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| {
                b.command.to_lowercase().contains(&query) || b.output_contains(query.as_str())
            })
            .map(|(i, _)| i)
            .collect()
    }

    /// Filter blocks by status.
    pub fn filter_by_status(&self, status: BlockStatus) -> Vec<usize> {
        self.blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| b.status == status)
            .map(|(i, _)| i)
            .collect()
    }

    /// Toggle bookmark on a block and maintain the bookmark list.
    pub fn toggle_bookmark(&mut self, id: &BlockId) {
        if let Some(idx) = self.find_by_id(id) {
            self.blocks[idx].toggle_bookmark();
            let bookmarked = self.blocks[idx].bookmarked;
            if bookmarked {
                if !self.bookmarks.contains(id) {
                    self.bookmarks.push(id.clone());
                }
            } else {
                self.bookmarks.retain(|b| b != id);
            }
        }
    }

    pub fn bookmarks(&self) -> &[BlockId] {
        &self.bookmarks
    }

    /// Re-input selected command(s) as a single string.
    pub fn reinput_selected(&self) -> Option<String> {
        self.selected().map(|b| b.command.clone())
    }

    /// Share representation: command and output formatted as markdown.
    pub fn share_block(&self, id: &BlockId) -> Option<String> {
        self.blocks
            .iter()
            .find(|b| b.id == *id)
            .map(|b| format!("```bash\n{}\n```\n```\n{}\n```", b.command, b.copy_output()))
    }
}

fn now_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_block(command: &str) -> Block {
        Block::new(
            BlockId::new(format!("block-{}", command.replace(' ', "-"))),
            command,
            "/home",
        )
    }

    #[test]
    fn block_starts_running() {
        let block = sample_block("ls -la");
        assert_eq!(block.status, BlockStatus::Running);
        assert!(block.exit_code.is_none());
    }

    #[test]
    fn block_appends_output_and_splits_lines() {
        let mut block = sample_block("echo hello");
        block.append_output(b"hello\nworld\n");
        assert_eq!(block.output_lines.len(), 2);
        assert_eq!(block.first_output_line(), Some("hello"));
        assert!(block.output_contains("world"));
    }

    #[test]
    fn block_appends_partial_line_then_completes_it() {
        let mut block = sample_block("printf slow");
        block.append_output(b"part1 ");
        block.append_output(b"part2\n");
        assert_eq!(block.output_lines.len(), 1);
        assert_eq!(block.output_lines[0], "part1 part2\n");
    }

    #[test]
    fn block_finish_sets_status_from_exit_code() {
        let mut block = sample_block("true");
        block.finish(0);
        assert_eq!(block.status, BlockStatus::Completed);
        assert_eq!(block.exit_code, Some(0));
        assert!(block.ended_at.is_some());

        let mut block = sample_block("false");
        block.finish(1);
        assert_eq!(block.status, BlockStatus::Failed);
    }

    #[test]
    fn block_cancel_changes_status() {
        let mut block = sample_block("sleep 10");
        block.cancel();
        assert_eq!(block.status, BlockStatus::Cancelled);
    }

    #[test]
    fn block_copy_command_and_output() {
        let mut block = sample_block("echo warp");
        block.append_output(b"warp\n");
        assert_eq!(block.copy_command(), "echo warp");
        assert_eq!(block.copy_output(), "warp\n");
        assert!(block.copy_block().contains("echo warp"));
        assert!(block.copy_block().contains("warp\n"));
    }

    #[test]
    fn block_bookmark_toggle() {
        let mut block = sample_block("git status");
        assert!(!block.bookmarked);
        block.toggle_bookmark();
        assert!(block.bookmarked);
        block.toggle_bookmark();
        assert!(!block.bookmarked);
    }

    #[test]
    fn block_store_navigation() {
        let mut store = BlockStore::new();
        store.add(sample_block("a"));
        store.add(sample_block("b"));
        store.add(sample_block("c"));

        assert_eq!(store.len(), 3);
        assert!(store.select(1));
        assert_eq!(store.selected().unwrap().command, "b");

        store.select_next();
        assert_eq!(store.selected_index(), Some(2));
        store.select_previous();
        assert_eq!(store.selected_index(), Some(1));

        // Cannot select beyond bounds.
        assert!(!store.select(10));
    }

    #[test]
    fn block_store_search_command_and_output() {
        let mut store = BlockStore::new();
        let mut b1 = sample_block("cargo build");
        b1.append_output(b"Compiling term2\n");
        let mut b2 = sample_block("cargo test");
        b2.append_output(b"running tests\n");
        store.add(b1);
        store.add(b2);

        let hits: Vec<_> = store.search("cargo");
        assert_eq!(hits, vec![0, 1]);

        let hits: Vec<_> = store.search("test");
        assert_eq!(hits, vec![1]);

        let hits: Vec<_> = store.search("Compiling");
        assert_eq!(hits, vec![0]);
    }

    #[test]
    fn block_store_filter_by_status() {
        let mut store = BlockStore::new();
        let mut b1 = sample_block("ok");
        b1.finish(0);
        let mut b2 = sample_block("err");
        b2.finish(2);
        let b3 = sample_block("running");
        store.add(b1);
        store.add(b2);
        store.add(b3);

        assert_eq!(store.filter_by_status(BlockStatus::Completed), vec![0]);
        assert_eq!(store.filter_by_status(BlockStatus::Failed), vec![1]);
        assert_eq!(store.filter_by_status(BlockStatus::Running), vec![2]);
    }

    #[test]
    fn block_store_bookmark_list_is_maintained() {
        let mut store = BlockStore::new();
        let b1 = sample_block("one");
        let id1 = b1.id.clone();
        let b2 = sample_block("two");
        let id2 = b2.id.clone();
        store.add(b1);
        store.add(b2);

        store.toggle_bookmark(&id1);
        store.toggle_bookmark(&id2);
        assert_eq!(store.bookmarks().len(), 2);

        store.toggle_bookmark(&id1);
        assert_eq!(store.bookmarks().len(), 1);
        assert_eq!(store.bookmarks()[0], id2);
    }

    #[test]
    fn block_store_reinput_selected_command() {
        let mut store = BlockStore::new();
        let b = sample_block("kubectl get pods");
        store.add(b);
        store.select(0);
        assert_eq!(
            store.reinput_selected(),
            Some("kubectl get pods".to_string())
        );
    }

    #[test]
    fn block_store_share_format() {
        let mut store = BlockStore::new();
        let mut b = sample_block("uname -a");
        let id = b.id.clone();
        b.append_output(b"Linux\n");
        store.add(b);

        let shared = store.share_block(&id).unwrap();
        assert!(shared.contains("```bash"));
        assert!(shared.contains("uname -a"));
        assert!(shared.contains("Linux"));
    }

    #[test]
    fn block_store_clear_removes_all() {
        let mut store = BlockStore::new();
        store.add(sample_block("x"));
        store.select(0);
        store.clear();
        assert!(store.is_empty());
        assert!(store.selected().is_none());
    }

    // Scenarios derived from Warp docs:

    #[test]
    fn every_command_creates_a_block() {
        // Warp groups every command and its output into a Block.
        let mut store = BlockStore::new();
        store.add(Block::new(BlockId::new("1"), "ls", "/home"));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn block_output_can_be_copied_independently() {
        // Blocks enable copying a command's output.
        let mut block = Block::new(BlockId::new("1"), "echo hi", "/");
        block.append_output(b"hi\n");
        assert_eq!(block.copy_output(), "hi\n");
    }

    #[test]
    fn block_scroll_to_start_of_output() {
        // Blocks let you scroll directly to the start of a command's output.
        let block = Block::new(BlockId::new("1"), "seq 1 100", "/");
        assert_eq!(block.output_start_line(), 0);
    }

    #[test]
    fn bookmarked_blocks_are_tracked() {
        // Blocks can be bookmarked and navigated.
        let mut store = BlockStore::new();
        let b = Block::new(BlockId::new("bm"), "important", "/");
        let id = b.id.clone();
        store.add(b);
        store.toggle_bookmark(&id);
        assert!(store.bookmarks().contains(&id));
    }

    #[test]
    fn block_status_reflects_exit_code() {
        // Completed vs Failed status is determined by exit code.
        let mut b = Block::new(BlockId::new("1"), "cmd", "/");
        b.finish(1);
        assert_eq!(b.status, BlockStatus::Failed);
    }

    #[test]
    #[ignore = "rendering not yet implemented"]
    fn block_long_output_virtualization() {
        // A block with 50,000 lines should still be searchable and scrollable
        // without exhausting memory.
    }

    #[test]
    #[ignore = "rendering not yet implemented"]
    fn block_sticky_header_shows_command_while_scrolling() {
        // While scrolling inside a block, the command header remains visible.
    }

    #[test]
    #[ignore = "sharing backend not yet implemented"]
    fn block_can_be_shared_as_formatted_link() {
        // Share both a command and its output with formatting.
    }

    #[test]
    #[ignore = "filtering UI not yet implemented"]
    fn block_filter_hides_non_matching_blocks() {
        // Filter blocks by command or output text.
    }
}
