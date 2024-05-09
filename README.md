# linked-rs
## Custom linked-list implementation

`linked-rs` is an implementation of a doubly linked list in Rust, along with performance testing scripts.

## Features

- Doubly linked list data structure with additional features than the `std::collections::LinkedList`.
- Allows insertions at any place pointed by a pointer.
- Allows removals at any place pointed by a pointer.
- Custom implementation optimized for performance.

## Usage

To use the doubly linked list in your Rust project, simply include the `doubly_linked_list.rs` file in your project's source directory.

Example usage:

```rust
use doubly_linked_list::DoublyLinkedList;

fn main() {
    // Create a new doubly linked list
    let mut list = DoublyLinkedList::new();

    // Insert elements into the list
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // Remove elements from the list
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_back(), Some(3));
}
```

### Way Forward
Some performance anomalies need to be further investigated and analyzed.
