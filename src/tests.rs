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
fn logger() -> std::io::Result<()> {
    init_logger(".test.log".into())?.apply().unwrap();
    diag!("To file");
    debug!("Hidden");
    info!("Info");
    warn!("Warn");
    error!("Error");
    diag_err!("internal error");
    Ok(())
}

#[cfg(feature = "fail")]
#[test]
fn failure() {
    let e: failure::Error = DiagError::unimplemented(diag_position!()).into();
    error!("{}", e);
    diag_err!();
}
