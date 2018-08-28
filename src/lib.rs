#[macro_use]
pub extern crate log;

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
    Unimplemented(Position)
}

#[macro_export]
macro_rules! diag_position {
    () => {{
        Position {
            file: file!(),
            line: line!(),
            column: column!(),
        }
    }}
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

#[cfg(test)]
mod tests;
