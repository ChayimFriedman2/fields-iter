# Superbitty

A bitfields crate.

```rust
use superbitty::{bitfields, BitFieldCompatible};

#[derive(BitFieldCompatible, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enum {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy)]
pub struct Rest(pub u8);

// SAFETY: We only set this via `Bitfields`, and thus the values are guaranteed
// to stay in range.
unsafe impl BitFieldCompatible for Rest {
    const SHIFT: u32 = 0;
    const BITS_LEN: u32 = 6;
    fn into_raw(self) -> u128 { self.0 as u128 }
    unsafe fn from_raw(v: u128) -> Self { Self(v as u8) }
}

bitfields! {
    pub struct Bitfields : u8 {
        pub e: Enum,
        pub r: Rest,
    }
}

fn main() {
    let mut instance = Bitfields::new(Enum::B, Rest(0b010));
    assert_eq!(instance.e(), Enum::B);
    instance.set_r(Rest(0b101));
    assert_eq!(instance.r().0, 0b101);
}
```
