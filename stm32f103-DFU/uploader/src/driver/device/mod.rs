use std::time;

pub mod generic;
pub mod identified;
pub mod identifier;
pub mod iterator;
pub mod opened;

pub const DEFAULT_IO_TIMEOUT: time::Duration = time::Duration::from_secs(1);
