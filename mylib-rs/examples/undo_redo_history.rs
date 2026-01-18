//! 使用LinkedList实现简单的文本编辑器的撤销和重做功能
//! cargo run -p mylib-rs --example undo_redo_history

use std::fmt::Display;

use algods::collections::LinkedList;

fn main() {
    let mut editor = TextEditor::new("Hello, world!");

    editor.display();
    editor.edit("Hello, Rust!");
    editor.display();
    editor.edit("Goodbye, Rust!");
    editor.display();

    editor.undo();
    editor.display(); // 恢复到 "Hello, Rust!"
    editor.undo();
    editor.display(); // 恢复到 "Hello, world!"
    editor.undo(); // No more history to undo!

    editor.redo();
    editor.display(); // 恢复到 "Hello, Rust!"
    editor.redo();
    editor.display(); // 恢复到 "Goodbye, Rust!"
    editor.redo(); // No more history to redo!
}

#[derive(Debug, Clone)]
struct TextEdit {
    content: String,
}

struct TextEditor {
    text: TextEdit,
    undo_history: LinkedList<TextEdit>,
    redo_history: LinkedList<TextEdit>,
}

impl TextEdit {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }

    fn set_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }
}

impl Display for TextEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Current content: {}", self.content)
    }
}

impl TextEditor {
    fn new(init_text: &str) -> Self {
        Self {
            text: TextEdit::new(init_text),
            undo_history: LinkedList::new(),
            redo_history: LinkedList::new(),
        }
    }

    fn edit(&mut self, new_text: &str) {
        self.undo_history.push_front(self.text.clone()); // 保存当前文本状态到撤销历史
        self.text.set_content(new_text);
        self.redo_history.clear(); // 一旦修改，清空重做历史
    }

    fn undo(&mut self) {
        if let Some(prev_state) = self.undo_history.pop_front() {
            self.redo_history.push_front(self.text.clone()); // 当前状态保存到重做历史
            self.text = prev_state;
        } else {
            println!("No more history to undo!");
        }
    }

    fn redo(&mut self) {
        if let Some(state) = self.redo_history.pop_front() {
            self.undo_history.push_front(self.text.clone()); // 当前状态保存到撤销历史
            self.text = state;
        } else {
            println!("No more history to redo!");
        }
    }

    fn display(&self) {
        println!("{}", self.text);
    }
}
