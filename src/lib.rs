//! Construct, write and run IBM Storage Scale file system policies.
//!
//! # Examples
//!
//! ## Write Policy
//!
//! ```
//! use mmpolicy::prelude::*;
//!
//! let mut policy = Policy::new("size");
//!
//! policy.rules.push(Rule::from(RuleType::ExternalList(
//!     Name("size".into()),
//!     Exec(String::new()),
//! )));
//!
//! policy.rules.push(
//!     Rule(
//!         Some(Name("size".into())),
//!         RuleType::List(
//!             Name("size".into()),
//!             DirectoriesPlus(true),
//!             vec![Show::KbAllocated],
//!             None,
//!         )
//!     )
//! );
//!
//! let mut result: Vec<u8> = Vec::new();
//! policy.write(&mut result).unwrap();
//! let result = std::str::from_utf8(&result).unwrap();
//!
//! let expected = indoc::indoc! {"
//!     RULE
//!       EXTERNAL LIST 'size'
//!       EXEC ''
//!
//!     RULE 'size'
//!       LIST 'size'
//!       DIRECTORIES_PLUS
//!       SHOW(VARCHAR(KB_ALLOCATED))
//! "};
//!
//! assert_eq!(expected, result);
//!
//! // write to file
//! // let mut file = std::fs::File::create("policy.txt").unwrap();
//! // policy.write(&mut file).unwrap();
//! ```
//!
//! ## Run Policy
//!
//! ```no_run
//! # use std::path::Path;
//! use mmpolicy::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut policy = Policy::new("size");
//! // add rules like above
//!
//! // options for an EXTERNAL LIST policy
//! let mut options = RunOptions::default();
//! options.action = Some("defer".into());
//! options.choice_algorithm = Some("fast".into());
//! options.information_level = Some("0".into());
//!
//! let reports = policy.run(
//!     "/data/test",
//!     "/work/.policy/size.policy",
//!     Some(Path::new("/work/.policy/report")),
//!     &options
//! )?;
//!
//! // for report in reports {
//!     // parse the file manually
//!     // this library does not yet provide a way to do this
//! // }
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

#[cfg(feature = "clap")]
pub mod clap;
mod run;
mod types;
mod write;

/// The important stuff.
pub mod prelude {
    pub use crate::run::Options as RunOptions;
    pub use crate::types::*;
}
