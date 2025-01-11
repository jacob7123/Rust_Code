# Rust Smart Pointer Comparison: `Box`, `Rc`, and `RefCell`

This document provides a concise comparison of Rust's smart pointers: `Box<T>`, `Rc<T>`, and `RefCell<T>`. These pointers enable flexible ownership and borrowing in different scenarios.

## Overview

### 1. **Box<T>**
- **Supports:** Mutable and immutable data.
- **Borrow Checking:** Compile-time.
- **Use Case:** Stores data on the heap with a single owner. Ideal when ownership and borrowing rules are straightforward.

### 2. **Rc<T>** (Reference Counting Pointer)
- **Supports:** Immutable data sharing only.
- **Borrow Checking:** Compile-time.
- **Use Case:** Enables multiple ownership for shared immutable data in a single-threaded context. Cannot directly handle mutable data without combining with `RefCell<T>`.

### 3. **RefCell<T>** (Interior Mutability Pointer)
- **Supports:** Mutable and immutable access via "interior mutability."
- **Borrow Checking:** Runtime.
- **Use Case:** Allows mutable access to data even when the outer structure is immutable. Suitable for single-threaded scenarios where borrowing rules need runtime enforcement.

---

## Comparison Table

| Pointer Type  | Mutability Support                  | Borrow Checking  | Primary Use Case                                |
|---------------|-------------------------------------|------------------|-----------------------------------------------|
| **Box<T>**    | Mutable and immutable               | Compile-time     | Heap storage with single ownership.           |
| **Rc<T>**     | Immutable only                      | Compile-time     | Shared ownership of immutable data.           |
| **RefCell<T>**| Mutable and immutable (interior mutability) | Runtime       | Runtime-checked borrowing for single-threaded interior mutability. |

---

## Notes
- For **multi-threaded environments**, consider:
  - `Arc<T>` for shared ownership of immutable data.
  - `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for mutable shared data with thread safety.
- Combining `Rc<T>` and `RefCell<T>` allows for shared ownership and mutability in single-threaded contexts.

---

## Example Code

### Box<T> Example
```rust
fn main() {
    let mut b = Box::new(5); // Mutable Box
    *b += 1; // Perform mutable operation
    println!("{}", b); // Output: 6
}
```

### Rc<T> Example
```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5); // Immutable Rc
    let b = Rc::clone(&a); // Shared ownership
    println!("{}", b); // Output: 5
}
```

### RefCell<T> Example
```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    *data.borrow_mut() += 1; // Mutable operation
    println!("{}", data.borrow()); // Output: 6
}
```
