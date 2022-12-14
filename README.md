# fields-iter

A crate that allows iterating over over struct's fields, getting their name and a mutable/shared
reference to them.

## Examples

Printing the values of all field whose name starts with "a" and are strings:
```rust
use fields_iter::{FieldsInspect, FieldsIter};

fn print_starts_with_a(v: &dyn FieldsInspect) {
    for (name, value) in FieldsIter::new(v) {
        if !name.starts_with('a') { continue };
        let Some(value) = value.downcast_ref::<String>() else { continue };
        println!("{name}={value}");
    }
}
```

Adding one to the field `add_here`:
```rust
use fields_iter::{FieldsInspect, FieldsIterMut};

let v: &mut dyn FieldsInspect;
let field = FieldsIterMut::new(v)
    .find(|&(name, _)| name == "add_here")
    .expect("no `add_here` field")
    .1
    .downcast_mut::<i32>()
    .expect("field `add_here` is not of type `i32`");
*field += 1;
# assert_eq!(original.add_here, 1);
```
