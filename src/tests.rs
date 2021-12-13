use crate::*;

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

#[test]
#[should_panic]
fn diag_unreachable_err() {
    let _ = diag_unreachable_err!();
}

#[test]
#[should_panic]
fn diag_unimplemented_err() {
    let _ = diag_unimplemented_err!();
}

#[test]
#[should_panic]
fn diag_unreachable_err_msg() {
    let _ = diag_unreachable_err!("message {}", "here");
}

#[test]
#[should_panic]
fn diag_unimplemented_err_msg() {
    let _ = diag_unimplemented_err!("message {}", "here");
}

#[test]
fn bail_diag() {
    assert!((|| -> Result<(), Error> {
        if true {
            bail_diag!("should return error");
        }
        Ok(())
    })().is_err());
}

#[cfg(feature = "impl")]
#[test]
fn logger() -> Result<()> {
    init_logger(".test.log".into())?.apply().unwrap();
    diag!("To file");
    debug!("Hidden");
    info!("Info");
    warn!("Warn");
    error!("Error");
    diag_err!();
    diag_err!("some internal error message {}", "here");
    Ok(())
}
