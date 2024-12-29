## Queue（先だし後入れ）

```rust
use std::collections::VecDeque;

struct Queue<T> {
    queue: VecDeque<T>
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, data: T) {
        self.queue.push_back(data);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.front()
    }
}

fn main() {
    let mut queue = Queue::new();

    println!("Peeking a queue before enqueue: {:?}", queue.peek());
    println!("{:?}", queue.queue);
    queue.enqueue(1);
    println!("{:?}", queue.queue);
    queue.enqueue(2);
    println!("{:?}", queue.queue);
    queue.enqueue(3);
    println!("{:?}", queue.queue);
    queue.enqueue(4);
    println!("{:?}", queue.queue);

    println!("Peeking a queue after enqueue: {:?}", queue.peek());
    queue.dequeue();
    println!("{:?}", queue.queue);
    queue.dequeue();
    println!("{:?}", queue.queue);
    queue.dequeue();
    println!("{:?}", queue.queue);
    queue.dequeue();
    println!("{:?}", queue.queue);
    println!("Peeking a queue after dequeue: {:?}", queue.peek());

}
```

```bash
Peeking a queue before enqueue: None
[]
[1]
[1, 2]
[1, 2, 3]
[1, 2, 3, 4]
Peeking a queue after enqueue: Some(1)
[2, 3, 4]
[3, 4]
[4]
[]
Peeking a queue after dequeue: None
```

## Vec で実装する場合

```rust
struct Queue<T> {
    queue: Vec<T>
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    fn enqueue(&mut self, data: T) {
        self.queue.push(data);
    }

    fn dequeue(&mut self) -> Option<T> {
        // panic を防ぐために None を返す
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

fn main() {
    let mut queue = Queue::new();

    println!("Peeking a queue before enqueue: {:?}", queue.peek());
    println!("{:?}", queue.queue);
    queue.enqueue(1);
    println!("{:?}", queue.queue);
    queue.enqueue(2);
    println!("{:?}", queue.queue);
    queue.enqueue(3);
    println!("{:?}", queue.queue);
    queue.enqueue(4);
    println!("{:?}", queue.queue);

    println!("Peeking a queue after enqueue: {:?}", queue.peek());
    queue.dequeue();
    println!("{:?}", queue.queue);
    queue.dequeue();
    println!("{:?}", queue.queue);
    queue.dequeue();
    println!("{:?}", queue.queue);
    queue.dequeue();
    println!("{:?}", queue.queue);
    println!("Peeking a queue after dequeue: {:?}", queue.peek());

    queue.dequeue();
    println!("Never panic because of checking if or not queue is empty: {:?}", queue.queue);
}
```

```bash
Peeking a queue before enqueue: None
[]
[1]
[1, 2]
[1, 2, 3]
[1, 2, 3, 4]
Peeking a queue after enqueue: Some(1)
[2, 3, 4]
[3, 4]
[4]
[]
Peeking a queue after dequeue: None
Never panic because of checking if or not queue is empty: []
```

## Queue を反転

```rust
use std::collections::VecDeque;

fn reverse(queue: &mut VecDeque<i32>) {
    let mut new_queue = VecDeque::new();

    while let Some(val) = queue.pop_front() {
        new_queue.push_front(val);
    }

    *queue = new_queue;
}

fn main() {
    let mut queue = VecDeque::new();

    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    queue.push_back(4);

    println!("Original queue: {:?}", queue);

    reverse(&mut queue);

    println!("Reversed queue: {:?}", queue);
}
```

```bash
Original queue: [1, 2, 3, 4]
Reversed queue: [4, 3, 2, 1]
```
