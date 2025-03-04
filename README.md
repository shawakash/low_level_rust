# Rust Smart Pointers Implementation

This project contains custom implementations of Rust's smart pointers and interior mutability types, helping to understand how they work under the hood.

## Implementations

### Cell<T>
A mutable memory location that provides interior mutability through a safe interface. This implementation shows how `Cell` enables mutation through shared references by using `UnsafeCell`.

Features:
- `new()`: Creates a new Cell containing a value
- `get()`: Returns a copy of the contained value
- `set()`: Modifies the contained value

### RefCell<T>
A mutable memory location with dynamically checked borrowing rules. This implementation demonstrates how `RefCell` manages mutable and immutable borrows at runtime.

Features:
- `new()`: Creates a new RefCell containing a value
- `borrow()`: Returns an immutable reference if allowed by borrowing rules
- `borrow_mut()`: Returns a mutable reference if allowed by borrowing rules

## Usage

```rust
let cell = Cell::new('a');
println!("{}", cell.get());
cell.set('c');
println!("{}", cell.get());

let ref_cell = RefCell::new(42);
if let Some(value) = ref_cell.borrow() {
    println!("Borrowed value: {}", value);
}
```

## Safety Notes

These implementations use `unsafe` code internally but provide a safe interface to users. They maintain Rust's memory safety guarantees through runtime checks and careful management of internal state.

## TODOs
- [ ] Implement Rc (Reference Counting)
- [ ] Add more test cases
- [ ] Add documentation
- [ ] Implement Arc (Atomic Reference Counting)
- [ ] Add thread safety features

## Learning Resources

This project is useful for:
- Understanding interior mutability in Rust
- Learning about smart pointers
- Exploring unsafe Rust
- Understanding borrowing rules
