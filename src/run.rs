use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::types::{Policy, RuleType};

/// Errors.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error when running `mmapplypolicy`.
    #[error("{0}: {1}")]
    ApplyPolicy(String, std::io::Error),

    /// Error when running `mmapplypolicy`.
    #[error("{0}")]
    ApplyPolicyFailed(String),

    /// Create policy file.
    #[error("creating policy file `{0}`: {1}")]
    CreatePolicyFile(PathBuf, std::io::Error),

    /// Invalid file list prefix.
    #[error("{0}")]
    InvalidFileListPrefix(String),

    /// Write policy to file.
    #[error("writing policy file to `{0}`: {1}")]
    WritePolicy(PathBuf, std::io::Error),
}

type Result<T> = ::std::result::Result<T, Error>;

/// Options for running `mmapplypolicy`.
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// Returns the choice algorithm used with `--choice-algorithm`.
    pub choice_algorithm: Option<String>,

    /// Returns the nodes for parallel execution used with `-N`.
    pub nodes: Option<String>,

    /// Returns the local work directory for parallel execution used with `-s`.
    pub local_work_dir: Option<PathBuf>,

    /// Returns the global work directory for parallel execution used with
    /// `-g`.
    pub global_work_dir: Option<PathBuf>,

    /// Returns the action performed on files used with `-I`.
    pub action: Option<String>,

    /// Returns the level of information displayed used with `-L`.
    pub information_level: Option<String>,
}

impl Policy {
    /// Write and run the policy.
    ///
    /// # Errors
    ///
    /// - creating the policy file
    /// - writing to the policy file
    /// - running the `mmapplypolicy` command
    pub fn run(
        &self,
        dev_or_dir: impl AsRef<OsStr>,
        policy_path: impl AsRef<Path>,
        file_list_prefix: Option<&Path>,
        options: &Options,
    ) -> Result<Vec<PathBuf>> {
        if file_list_prefix.map_or(false, |prefix| {
            !prefix.is_dir() && prefix.file_name().is_none()
        }) {
            return Err(Error::InvalidFileListPrefix(String::from(
"prefix needs to be either an existing directory or have a file name \
 component in its path"
            )));
        }

        let policy_path = policy_path.as_ref();

        let mut policy_file = File::create(policy_path).map_err(|error| {
            Error::CreatePolicyFile(policy_path.to_owned(), error)
        })?;

        if let Some(error) = self.write(&mut policy_file).err() {
            return Err(Error::WritePolicy(policy_path.to_owned(), error));
        }

        let mut mmapplypolicy = Command::new("mmapplypolicy");
        mmapplypolicy.arg(dev_or_dir.as_ref());
        mmapplypolicy.arg("-P").arg(policy_path);

        if let Some(action) = &options.action {
            mmapplypolicy.arg("-I").arg(action);
        }

        if let Some(information_level) = &options.information_level {
            mmapplypolicy.arg("-L").arg(information_level);

            if information_level == "0" {
                mmapplypolicy.stdout(Stdio::null());
            } else {
                mmapplypolicy.stdout(std::io::stderr());
            }
        }

        if let Some(choice_algorithm) = &options.choice_algorithm {
            mmapplypolicy
                .arg("--choice-algorithm")
                .arg(choice_algorithm);
        }

        if let Some(prefix) = file_list_prefix {
            mmapplypolicy.arg("-f").arg(prefix);
        }

        if let Some(nodes) = &options.nodes {
            mmapplypolicy.arg("-N").arg(nodes);
        }

        if let Some(local_work_dir) = &options.local_work_dir {
            mmapplypolicy.arg("-s").arg(local_work_dir);
        }

        if let Some(global_work_dir) = &options.global_work_dir {
            mmapplypolicy.arg("-g").arg(global_work_dir);
        }

        #[cfg(feature = "log")]
        log::debug!("running external process: {mmapplypolicy:?}");

        let mut mmapplypolicy = mmapplypolicy.spawn().map_err(|error| {
            Error::ApplyPolicy("`mmapplypolicy` failed to start".into(), error)
        })?;

        let mmapplypolicy = mmapplypolicy.wait().map_err(|error| {
            Error::ApplyPolicy(
                "failed waiting on `mmapplypolicy`".into(),
                error,
            )
        })?;

        if mmapplypolicy.success() {
            let reports = self
                .rules
                .iter()
                .filter_map(|rule| match &rule.1 {
                    RuleType::ExternalList(name, _) => {
                        file_list_prefix.map(|prefix| {
                            if prefix.is_dir() {
                                prefix.join(format!("list.{}", name.0))
                            } else {
                                let mut file_name =
                                    prefix.file_name().map_or_else(
                                        || OsString::from(&self.name.0), // TODO
                                        ToOwned::to_owned,
                                    );

                                file_name.push(".list.");
                                file_name.push(&name.0);

                                prefix.with_file_name(file_name)
                            }
                        })
                    }

                    RuleType::List(_, _, _, _) => None,
                })
                .collect();

            Ok(reports)
        } else {
            let mut message = String::new();
            message.push_str("`mmapplypolicy` failed");

            if let Some(rc) = mmapplypolicy.code() {
                message.push_str(&format!(" with exit status {rc}"));
            };

            Err(Error::ApplyPolicyFailed(message))
        }
    }
}
