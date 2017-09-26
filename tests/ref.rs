#[macro_use]
extern crate index_fixed;

#[test]
fn const_ref() {
    let a = [1u8,2,3,6];
    let b = &a;
    assert_eq!(index_fixed!(&b; ..2), &[1u8,2]);
}

#[test]
fn mut_ref() {
    let mut a = [1u8,2,3,6];
    let b = &mut a;
    assert_eq!(index_fixed!(&mut b; ..2), &[1u8,2]);
}
