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

#[cfg(feature = "impl")]
#[test]
fn logger() {
    init_logger().apply().unwrap();
    diag!("To file");
    debug!("Hidden");
    info!("Info");
    error!("Error");
}
