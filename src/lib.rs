#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
#[macro_use]
#[allow(unused_imports)]
pub extern crate log;
#[cfg(feature = "impl")]
pub extern crate chrono;
#[cfg(feature = "impl")]
pub extern crate fern;
#[cfg(feature = "fail")]
pub extern crate failure;
#[cfg(feature = "fail")]
#[macro_use]
pub extern crate failure_derive;

pub use log::*;

#[cfg(feature = "fail")]
pub use failure_derive::*;

#[derive(Debug)]
pub struct Position {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "fail", derive(Fail))]
pub enum DiagError {
    #[cfg_attr(feature = "fail", fail(display = "unreachable code reached at {}", pos))]
    UnreachableCodeReached { pos: Position },
    #[cfg_attr(feature = "fail", fail(display = "unimplemented code reached at {}", pos))]
    UnimplementedCodeReached { pos: Position },
    #[cfg_attr(feature = "fail", fail(display = "internal error at {}", pos))]
    InternalError { pos: Position },
}

impl DiagError {

    pub fn unimplemented(pos: Position) -> Self {
        DiagError::UnimplementedCodeReached { pos }
    }

    pub fn unreachable(pos: Position) -> Self {
        DiagError::UnreachableCodeReached { pos }
    }

    pub fn internal_error(pos: Position) -> Self {
        DiagError::InternalError { pos }
    }
}

#[macro_export]
macro_rules! diag_position {
    () => {{
        $crate::Position {
            file: file!(),
            line: line!(),
            column: column!(),
        }
    }};
}

#[macro_export]
macro_rules! diag {
    ($($arg:tt)+) => (error!(target: "diagnostics", $($arg)*));
}

#[cfg(feature = "fail")]
#[macro_export]
macro_rules! diag_err {
    () => {{
        diag!("internal error at {}", diag_position!());
        $crate::failure::Error::from($crate::DiagError::InternalError { pos: diag_position!() })
    }};
    ($($arg:tt)+) => {{
        diag!($($arg)*);
        $crate::failure::Error::from($crate::DiagError::InternalError { pos: diag_position!() })
    }}
}

#[cfg(not(feature = "fail"))]
#[macro_export]
macro_rules! diag_err {
    () => {{
        diag!("internal error");
        $crate::DiagError::InternalError { pos: diag_position!() }
    }};
    ($($arg:tt)+) => {{
        diag!($($arg)*);
        $crate::DiagError::InternalError { pos: diag_position!() }
    }}
}

#[macro_export]
macro_rules! diag_unreachable {
    () => {{
        debug_assert!(false, "unreachable code reached");
        diag!("unreachable code reached at {}", diag_position!());
    }};
    ($($arg:tt)+) => {{
        debug_assert!(false, $($arg)*);
        diag!($($arg)*);
    }}
}

#[macro_export]
macro_rules! diag_unreachable_err {
    () => {{
        diag_unreachable!();
        return Err($crate::DiagError::UnreachableCodeReached { pos: diag_position!() }.into());
    }};
    ($($arg:tt)+) => {{
        diag_unreachable!($($arg)*);
        return Err($crate::DiagError::UnreachableCodeReached { pos: diag_position!() }.into());
    }}
}

#[macro_export]
macro_rules! diag_unimplemented {
    () => {{
        debug_assert!(false, "unimplemented code reached");
        diag!("unimplemented code reached at {}", diag_position!());
    }};
    ($($arg:tt)+) => {{
        debug_assert!(false, $($arg)*);
        diag!($($arg)*);
    }}
}

#[macro_export]
macro_rules! diag_unimplemented_err {
    () => {{
        diag_unreachable!();
        return Err($crate::DiagError::UnimplementedCodeReached { pos: diag_position!() }.into());
    }};
    ($($arg:tt)+) => {{
        diag_unreachable!($($arg)*);
        return Err($crate::DiagError::UnimplementedCodeReached { pos: diag_position!() }.into());
    }}
}

#[cfg(feature = "impl")]
pub fn stdout_dispatch() -> fern::Dispatch {
    use fern::colors::Color;
    let colors = fern::colors::ColoredLevelConfig::new()
        .trace(Color::White)
        .debug(Color::Blue)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {}{} {}: {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                colors.color(record.level()),
                if record.level() == log::Level::Info {
                    " "
                } else {
                    ""
                },
                record.target(),
                message,
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("diagnostics", log::LevelFilter::Off)
}

#[cfg(feature = "impl")]
pub fn diag_dispatch() -> fern::Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {}{} {}: {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S%.9f]"),
                record.level(),
                if record.level() == log::Level::Info {
                    " "
                } else {
                    ""
                },
                record.target(),
                message,
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("diagnostics", log::LevelFilter::Trace)
}

#[cfg(feature = "impl")]
pub fn init_logger(log_file: Option<impl AsRef<std::path::Path>>) -> std::io::Result<fern::Dispatch> {
    let mut dispatch = fern::Dispatch::new().chain(stdout_dispatch().chain(std::io::stdout()));
    let log_file = if let Some(log_file) = log_file {
        Some(log_file.as_ref().to_owned())
    } else if let Ok(exe_path) = std::env::current_exe() {
        exe_path.parent().map(|exe_dir| exe_dir.join(".diag.log"))
    } else {
        None
    };
    if let Some(log_file) = log_file {
        let log_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(log_file)?;
        dispatch = dispatch.chain(diag_dispatch().chain(log_file))
    }
    Ok(dispatch)
}

#[cfg(test)]
mod tests;
