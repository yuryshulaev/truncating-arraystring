# truncating-arraystring

[ArrayString](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayString.html) wrapper with truncating Write.

```rust
use std::fmt::Write;
use truncating_arraystring::TruncatingArrayString;

fn main() {
    let mut buf = TruncatingArrayString::<5>::new();
    assert_eq!(write!(buf, "{}", "12"), Ok(()));
    assert_eq!(write!(buf, "{}", "3456789"), Err(std::fmt::Error));
    assert_eq!(&buf.0[..], "12345");
}
```
