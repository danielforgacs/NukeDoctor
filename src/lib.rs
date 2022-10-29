pub mod project_modules {
    pub use clap::{Arg, ArgAction, Command};
    pub use serde::{Deserialize, Serialize};
    pub use std::fs::File;
    pub use std::io::prelude::*;
    pub use std::path::Path;
}
pub mod config;
pub mod parser;
pub mod structs;
pub mod utils;

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
