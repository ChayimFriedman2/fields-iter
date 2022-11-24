use fields_iter::{FieldsInspect, FieldsIter, FieldsIterMut};

fn _assert_object_safe(_: &dyn FieldsInspect) {}

#[derive(FieldsInspect)]
struct HasFieldsInspect {
    a: i32,
    b: String,
}

#[test]
fn fields_iter_gives_all_fields_in_declaration_order_and_no_more() {
    let v = HasFieldsInspect { a: 123, b: "abc".to_owned() };
    let fields = FieldsIter::new(&v).collect::<Vec<_>>();
    assert!(fields.len() == 2);
    assert!(fields[0].0 == "a" && std::ptr::eq(fields[0].1, &v.a));
    assert!(fields[1].0 == "b" && std::ptr::eq(fields[1].1, &v.b));
}

#[test]
fn fields_iter_mut_does_not_trigger_miri() {
    let mut v = HasFieldsInspect { a: 123, b: "abc".to_owned() };
    let mut fields = FieldsIterMut::new(&mut v as &mut dyn FieldsInspect).collect::<Vec<_>>();
    assert!(fields.len() == 2);
    assert!(fields[0].0 == "a" && *fields[0].1.downcast_ref::<i32>().unwrap() == 123);
    assert!(fields[1].0 == "b" && *fields[1].1.downcast_ref::<String>().unwrap() == "abc");
    *fields[0].1.downcast_mut::<i32>().unwrap() = 456;
    *fields[1].1.downcast_mut::<String>().unwrap() = "def".to_owned();
    *fields[0].1.downcast_mut::<i32>().unwrap() = 789;
    assert_eq!(v.a, 789);
    assert_eq!(v.b, "def");
}

#[test]
fn fields_iter_double_ended() {
    let v = HasFieldsInspect { a: 0, b: String::new() };

    let mut fields = FieldsIter::new(&v);
    assert!(fields.next_back().unwrap().0 == "b");
    assert!(fields.next_back().unwrap().0 == "a");
    assert!(fields.next_back().is_none());

    let mut fields = FieldsIter::new(&v);
    assert!(fields.next_back().unwrap().0 == "b");
    assert!(fields.next().unwrap().0 == "a");
    assert!(fields.next_back().is_none());
}

#[test]
fn fields_iter_mut_double_ended() {
    let mut v = HasFieldsInspect { a: 0, b: String::new() };

    let mut fields = FieldsIterMut::new(&mut v);
    assert!(fields.next_back().unwrap().0 == "b");
    assert!(fields.next_back().unwrap().0 == "a");
    assert!(fields.next_back().is_none());

    let mut fields = FieldsIterMut::new(&mut v);
    assert!(fields.next_back().unwrap().0 == "b");
    assert!(fields.next().unwrap().0 == "a");
    assert!(fields.next_back().is_none());
}
