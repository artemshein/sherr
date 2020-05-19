pub use backtrace;
#[cfg(feature = "impl")]
pub use chrono;
#[cfg(feature = "impl")]
pub use fern;
#[cfg(feature = "impl")]
pub use libc;
#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
#[allow(unused_imports)]
pub use log;

pub use log::*;
pub use anyhow::*;
pub use anyhow;

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
    ($($arg:tt)+) => {{
        error!(target: "diagnostics", $($arg)*);
    }};
}

#[macro_export]
macro_rules! diag_backtrace {
    () => {{
        error!(target: "diagnostics", "{:?}", $crate::backtrace::Backtrace::new());
    }}
}

#[macro_export]
macro_rules! diag_err {
    () => {{
        diag!("internal error at {}", diag_position!());
        diag_backtrace!();
        $crate::anyhow::anyhow!("internal error")
    }};
    ($($arg:tt)+) => {{
        diag!("internal error at {}", diag_position!());
        diag!($($arg)*);
        diag_backtrace!();
        $crate::anyhow::anyhow!($($arg)*)
    }}
}

#[macro_export]
macro_rules! diag_unreachable {
    () => {{
        debug_assert!(false, "unreachable code reached");
        diag!("unreachable code reached at {}", diag_position!());
        diag_backtrace!();
    }};
    ($($arg:tt)+) => {{
        debug_assert!(false, $($arg)*);
        diag!($($arg)*);
        diag_backtrace!();
    }}
}

#[macro_export]
macro_rules! diag_unreachable_err {
    () => {{
        diag_unreachable!();
        $crate::anyhow::anyhow!("unreachable code reached at {}", diag_position!())
    }};
    ($($arg:tt)+) => {{
        diag_unreachable!($($arg)*);
        $crate::anyhow::anyhow!($($arg)*)
    }}
}

#[macro_export]
macro_rules! diag_unimplemented {
    () => {{
        debug_assert!(false, "unimplemented code reached");
        diag!("unimplemented code reached at {}", diag_position!());
        diag_backtrace!();
    }};
    ($($arg:tt)+) => {{
        debug_assert!(false, $($arg)*);
        diag!($($arg)*);
        diag_backtrace!();
    }}
}

#[macro_export]
macro_rules! diag_unimplemented_err {
    () => {{
        diag_unreachable!();
        $crate::anyhow::anyhow!("unimplemented code reached at {}", diag_position!())
    }};
    ($($arg:tt)+) => {{
        diag_unreachable!($($arg)*);
        $crate::anyhow::anyhow!($($arg)*)
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
                "{} {}{}: {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                if atty::is(atty::Stream::Stdout) {
                    format!("{}", colors.color(record.level()))
                } else {
                    format!("{}", record.level())
                },
                if record.level() == log::Level::Info || record.level() == log::Level::Warn {
                    " "
                } else {
                    ""
                },
                message,
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("diagnostics", log::LevelFilter::Off)
}

#[cfg(feature = "impl")]
pub fn stdout_dispatch_with_target() -> fern::Dispatch {
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
                if record.level() == log::Level::Info || record.level() == log::Level::Warn {
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
pub fn init_logger(
    log_file: Option<impl AsRef<std::path::Path>>,
) -> std::io::Result<fern::Dispatch> {
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
