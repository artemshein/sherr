#[macro_use]
pub extern crate log;
#[cfg(feature = "impl")]
pub extern crate chrono;
#[cfg(feature = "impl")]
pub extern crate fern;

pub use log::*;

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

pub enum DiagError {
    UnreachableCodeReached(Position),
    Unimplemented(Position),
}

#[macro_export]
macro_rules! diag_position {
    () => {{
        Position {
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
        return Err(::DiagError::UnreachableCodeReached(diag_position!()).into());
    }};
    ($($arg:tt)+) => {{
        diag_unreachable!($($arg)*);
        return Err(::DiagError::UnreachableCodeReached(diag_position!()).into());
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
        return Err(::DiagError::Unimplemented(diag_position!()).into());
    }};
    ($($arg:tt)+) => {{
        diag_unreachable!($($arg)*);
        return Err(::DiagError::Unimplemented(diag_position!()).into());
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
        .level_for("diagnostics", log::LevelFilter::Trace)
}

#[cfg(feature = "impl")]
pub fn init_logger() -> fern::Dispatch {
    let mut dispatch = fern::Dispatch::new().chain(stdout_dispatch().chain(std::io::stdout()));
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if let Ok(log_file) = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(exe_dir.join(".diag.log"))
            {
                dispatch = dispatch.chain(diag_dispatch().chain(log_file))
            }
        }
    }
    dispatch
}

#[cfg(test)]
mod tests;
