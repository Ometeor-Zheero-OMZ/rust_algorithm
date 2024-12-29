## Stack (FIFO: 後入れ先出し)

```rust
struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    fn push(&mut self, val: T) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Stack の最上位の要素を確認
    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

fn main() {
    let json1 = "{'key1': 'value1', 'key2': [1, 2, 3], 'key3': (1, 2, 3)}";
    let json2 = "{'key1': ['value1', 'key2': [1, 2, 3], 'key3': (1, 2, 3)}";
}
```

```bash
Pushing elements onto the stack
Top element of the stack: Some(3)
Popping elements from the stack
Popped: 3
Popped: 2
Popped: 1
Is the stack empty? true
```

## スタックに開き括弧を追加し、ペアを探す

```rust
use std::collections::HashMap;

fn validate_format(chars: &str) -> bool {
    let lookup: HashMap<char, char> = [
        ('{', '}'),
        ('[', ']'),
        ('(', ')')
    ]
    .iter()
    .cloned()
    .collect();

    let mut stack = Vec::new();

    for char in chars.chars() {
        if lookup.contains_key(&char) { // 開き括弧の場合
            stack.push(lookup[&char]);  // 対応する閉じ括弧をスタックに追加
        } else if lookup.values().any(|&v| v == char) {
            if stack.is_empty() || stack.pop() != Some(char) { // 対応する閉じ括弧がない場合
                return false;
            }
        }
    }

    stack.is_empty() // スタックが空であれば問題なし
}

fn main() {
    let json1 = "{'key1': 'value1', 'key2': [1, 2, 3], 'key3': (1, 2, 3)}";
    let json2 = "{'key1': ['value1', 'key2': [1, 2, 3], 'key3': (1, 2, 3)}";

    println!("{}", validate_format(json1)); // true
    println!("{}", validate_format(json2)); // false
}
```

```bash
true
false
```
