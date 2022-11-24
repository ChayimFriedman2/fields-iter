use fields_iter::FieldsInspect;

#[derive(FieldsInspect)]
enum Foo {}

#[derive(FieldsInspect)]
union Bar {
    _foo: (),
}

fn main() {}
