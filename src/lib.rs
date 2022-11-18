#![cfg(fuzzing)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate dirs;
extern crate glob;
extern crate nix;
extern crate structopt;
#[macro_use]
extern crate failure;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate backtrace;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate test;

#[macro_use]
mod macros;
mod bash_server;
mod builtins;
mod context_parser;
mod dircolor;
mod eval;
mod expand;
mod fuzzy;
mod highlight;
mod history;
mod mainloop;
pub mod parser;
mod path;
mod pattern;
mod process;
mod prompt;
mod shell;
mod theme;
mod utils;
mod variable;

use crate::process::ExitStatus;
use crate::variable::Value;
use crossterm::tty::IsTty;
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::exit;
use structopt::StructOpt;

lazy_static! {
    pub static ref STARTED_AT: std::time::SystemTime = std::time::SystemTime::now();
}

const DEFAULT_PATH: &str = "/sbin:/usr/sbin:/usr/local/sbin:/bin:/usr/bin:/usr/local/bin";

