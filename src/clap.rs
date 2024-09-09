//! Integration with [`clap`](https://docs.rs/clap).
//!
//! # Defaults
//!
//! - only long options, all prefixed with `--mm`
//! - all are grouped in a help heading
//! - hidden in short help
//!
//! # Examples
//!
//! ```
//! # use std::path::PathBuf;
//! use clap::Command;
//! use mmpolicy::prelude::*;
//!
//! let cli = Command::new("mmx").args(mmpolicy::clap::args_all());
//!
//! let args = cli.get_matches_from(vec![
//!     "mmx",
//!     "--mm-N", "filer1,filer2,filer3",
//!     "--mm-g", "/work/.policy/mmx",
//!     "--mm-s", "/work/.policy/mmx",
//!     "--mm-choice-algorithm", "fast",
//!     "--mm-I", "defer",
//!     "--mm-L", "0",
//! ]);
//!
//! let options = RunOptions::from(&args);
//!
//! assert_eq!(options.nodes, Some(String::from("filer1,filer2,filer3")));
//! assert_eq!(options.global_work_dir, Some(PathBuf::from("/work/.policy/mmx")));
//! assert_eq!(options.local_work_dir, Some(PathBuf::from("/work/.policy/mmx")));
//! assert_eq!(options.choice_algorithm, Some(String::from("fast")));
//! assert_eq!(options.action, Some(String::from("defer")));
//! assert_eq!(options.information_level, Some(String::from("0")));
//! ```

use std::path::PathBuf;

use clap::value_parser;
use clap::{Arg, ArgMatches};

use crate::run::Options;

const ARG_ACTION: &str = "mm-action";
const ARG_CHOICE_ALGORITHM: &str = "mm-choice-algorithm";
const ARG_GLOBAL_WORK_DIR: &str = "mm-global-work-dir";
const ARG_INFORMATION_LEVEL: &str = "mm-information-level";
const ARG_LOCAL_WORK_DIR: &str = "mm-local-work-dir";
const ARG_NODES: &str = "mm-nodes";

const HELP_HEADING: &str = "Forwarded to `mmapplypolicy`";

impl From<&ArgMatches> for Options {
    fn from(args: &ArgMatches) -> Self {
        let mut options = Self::default();

        if let Some(value) = args.get_one::<String>(ARG_ACTION) {
            options.action = Some(value.to_owned());
        }

        if let Some(value) = args.get_one::<String>(ARG_CHOICE_ALGORITHM) {
            options.choice_algorithm = Some(value.to_owned());
        }

        if let Some(value) = args.get_one::<PathBuf>(ARG_GLOBAL_WORK_DIR) {
            options.global_work_dir = Some(value.to_owned());
        }

        if let Some(value) = args.get_one::<String>(ARG_INFORMATION_LEVEL) {
            options.information_level = Some(value.to_owned());
        }

        if let Some(value) = args.get_one::<PathBuf>(ARG_LOCAL_WORK_DIR) {
            options.local_work_dir = Some(value.to_owned());
        }

        if let Some(value) = args.get_one::<String>(ARG_NODES) {
            options.nodes = Some(value.to_owned());
        }

        options
    }
}

// ----------------------------------------------------------------------------
// argument groups
// ----------------------------------------------------------------------------

/// Returns all arguments.
#[must_use]
pub fn args_all() -> Vec<Arg> {
    vec![
        arg_nodes(),
        arg_local_work_dir(),
        arg_global_work_dir(),
        arg_action(),
        arg_information_level(),
        arg_choice_algorithm(),
    ]
}

/// Returns the arguments used for parallel execution `-N`, `-s`, and `-g`.
#[must_use]
pub fn args_parallel() -> Vec<Arg> {
    vec![arg_nodes(), arg_local_work_dir(), arg_global_work_dir()]
}

// ----------------------------------------------------------------------------
// arguments
// ----------------------------------------------------------------------------

/// Returns the argumunt forwarded to `mmapplypolicy -I`.
#[must_use]
pub fn arg_action() -> Arg {
    Arg::new(ARG_ACTION)
        .long("mm-I")
        .hide_short_help(true)
        .long_help("Action performed on files, used with `mmapplypolicy -I`.")
        .value_name("yes|defer|test|prepare")
        .help_heading(HELP_HEADING)
}

/// Returns the argumunt forwarded to `mmapplypolicy --choice-algorithm`.
#[must_use]
pub fn arg_choice_algorithm() -> Arg {
    Arg::new(ARG_CHOICE_ALGORITHM)
        .long("mm-choice-algorithm")
        .hide_short_help(true)
        .long_help(
            "Algorithm to select candidate files, used with `mmapplypolicy --choice-algorithm`.",
        )
        .value_name("best|exact|fast")
        .help_heading(HELP_HEADING)
}

/// Returns the argumunt forwarded to `mmapplypolicy -g`.
#[must_use]
pub fn arg_global_work_dir() -> Arg {
    Arg::new(ARG_GLOBAL_WORK_DIR)
        .long("mm-g")
        .hide_short_help(true)
        .long_help("Global work directory, used with `mmapplypolicy -g`.")
        .value_name("DIR")
        .value_parser(value_parser!(PathBuf))
        .help_heading(HELP_HEADING)
}

/// Returns the argumunt forwarded to `mmapplypolicy -L`.
#[must_use]
pub fn arg_information_level() -> Arg {
    Arg::new(ARG_INFORMATION_LEVEL)
        .long("mm-L")
        .hide_short_help(true)
        .long_help("Information level, used with `mmapplypolicy -L`.")
        .value_name("0|1|...|6")
        .help_heading(HELP_HEADING)
}

/// Returns the argumunt forwarded to `mmapplypolicy -s`.
#[must_use]
pub fn arg_local_work_dir() -> Arg {
    Arg::new(ARG_LOCAL_WORK_DIR)
        .long("mm-s")
        .hide_short_help(true)
        .long_help("Local work directory, used with `mmapplypolicy -s`.")
        .value_name("DIR")
        .value_parser(value_parser!(PathBuf))
        .help_heading(HELP_HEADING)
}

/// Returns the argumunt forwarded to `mmapplypolicy -N`.
#[must_use]
pub fn arg_nodes() -> Arg {
    Arg::new(ARG_NODES)
        .long("mm-N")
        .hide_short_help(true)
        .long_help("List of nodes, used with `mmapplypolicy -N`.")
        .value_name("all|mount|Node,...|NodeFile|NodeClass")
        .help_heading(HELP_HEADING)
}
