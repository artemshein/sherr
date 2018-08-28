use super::*;

#[test]
#[should_panic]
fn diag_unreachable() {
    diag_unreachable!()
}

#[test]
#[should_panic]
fn diag_unimplemented() {
    diag_unimplemented!()
}
