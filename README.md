# duble-vec

<b>Rust library for optimizing 2D vectors 
</b>
<br>

<b>WARNING:</b> This library is primarily intended for our personal project.
I just want to share it with you so you can test it and, if you like, give me some feedback (or even use it)
<br>

install: ```$ cargo add duble-vec```.
<br>
documentation on: https://docs.rs/duble-vec/
<br>
### Example
```rust
use duble_vec::*;

fn main() {
    let mut vec: DubleVec<i32> = DubleVec::new(Size { w: 6, h: 5 });
    vec.push(5, Index { x: 1, y: 1 });
    if let Some(value) = vec.get(Index { x: 1, y: 1 }) {
        println!("Value: {}", value);
    } else {
        println!("No value at this index");
    }
    println!("Size: {}", vec.size());
    vec.print();
}
```
