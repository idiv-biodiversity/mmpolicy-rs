use std::io::{self, Write};

use crate::types::{Exec, Name, Policy, Rule, RuleType, Show, Where};

impl Policy {
    /// Write the policy to `output`.
    ///
    /// # Errors
    ///
    /// Errors may only occur when writing to `output`.
    pub fn write(&self, output: &mut impl Write) -> io::Result<()> {
        let mut first = true;

        for rule in &self.rules {
            if first {
                first = false;
            } else {
                writeln!(output)?;
            }

            rule.write(output)?;
        }

        Ok(())
    }
}

impl Rule {
    fn write(&self, output: &mut impl Write) -> io::Result<()> {
        if let Some(name) = &self.0 {
            writeln!(output, "RULE '{}'", name.0)?;
        } else {
            writeln!(output, "RULE")?;
        }

        self.1.write(output)
    }
}

impl RuleType {
    fn write(&self, output: &mut impl Write) -> io::Result<()> {
        match self {
            Self::ExternalList(Name(name), Exec(cmd)) => {
                writeln!(output, "  EXTERNAL LIST '{name}'")?;
                writeln!(output, "  EXEC '{cmd}'")?;
            }

            Self::List(Name(name), directories_plus, show, filter) => {
                writeln!(output, "  LIST '{name}'")?;

                if directories_plus.0 {
                    writeln!(output, "  DIRECTORIES_PLUS")?;
                }

                if !show.is_empty() {
                    let s = show
                        .iter()
                        .map(|s| format!("VARCHAR({})", s.as_str()))
                        .collect::<Vec<_>>()
                        .join(" || ' ' || ");

                    writeln!(output, "  SHOW({s})")?;
                }

                if let Some(filter) = filter {
                    let s = match filter {
                        Where::Group(group) => format!("GROUP_ID = {group}"),
                        Where::User(user) => format!("USER_ID = {user}"),
                    };

                    writeln!(output, "  WHERE {s}")?;
                }
            }
        }

        Ok(())
    }
}

impl Show {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Mode => "MODE",
            Self::Nlink => "NLINK",
            Self::FileSize => "FILE_SIZE",
            Self::KbAllocated => "KB_ALLOCATED",
        }
    }
}

// ----------------------------------------------------------------------------
// tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::types::*;

    use indoc::indoc;

    #[test]
    fn list() {
        let mut policy = Policy::new("list");

        policy.rules.push(Rule::from(RuleType::ExternalList(
            Name("size".into()),
            Exec(String::new()),
        )));

        policy.rules.push(Rule::from(RuleType::List(
            Name("size".into()),
            DirectoriesPlus(true),
            vec![Show::Mode, Show::Nlink, Show::FileSize, Show::KbAllocated],
            Some(Where::User(1000)),
        )));

        let mut result: Vec<u8> = Vec::new();
        policy.write(&mut result).unwrap();
        let result = std::str::from_utf8(&result).unwrap();

        let expected = indoc! {"
            RULE
              EXTERNAL LIST 'size'
              EXEC ''

            RULE
              LIST 'size'
              DIRECTORIES_PLUS
              SHOW(VARCHAR(MODE) || ' ' || VARCHAR(NLINK) || ' ' || VARCHAR(FILE_SIZE) || ' ' || VARCHAR(KB_ALLOCATED))
              WHERE USER_ID = 1000
            "
        };

        assert_eq!(expected, result);
    }
}
