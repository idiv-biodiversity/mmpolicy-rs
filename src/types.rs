//! Type definitions.

use std::fmt;

use libc::{gid_t, uid_t};

/// Policy with rules.
#[derive(Debug)]
pub struct Policy {
    /// The name will be used when running the policy.
    pub name: Name,

    /// Definitions for use by all rules.
    pub defines: Vec<Definition>,

    /// The rules of the policy.
    pub rules: Vec<Rule>,
}

/// Constructors.
impl Policy {
    /// Returns an empty, named policy.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Name(name.into()),
            defines: vec![],
            rules: vec![],
        }
    }
}

/// Definition to be used by all rules.
#[derive(Debug)]
pub struct Definition {
    /// Name.
    pub name: String,

    /// Value.
    pub value: String,
}

impl Definition {
    /// Returns a new definition.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// Single policy rule.
#[derive(Debug)]
pub struct Rule(pub Option<Name>, pub RuleType);

impl From<RuleType> for Rule {
    fn from(rule_type: RuleType) -> Self {
        Self(None, rule_type)
    }
}

/// Name for entities.
#[derive(Debug)]
pub struct Name(pub String);

/// Execution for certain rule types.
#[derive(Debug)]
pub struct Exec(pub String);

/// Whether to apply the policy to all objects, not just regular files.
#[derive(Debug)]
pub struct DirectoriesPlus(pub bool);

/// Attributes to show.
#[derive(Debug)]
pub enum Show {
    /// `VARCHAR(MODE)`
    Mode,

    /// `VARCHAR(NLINK)`
    Nlink,

    /// `VARCHAR(FILE_SIZE)`
    FileSize,

    /// `VARCHAR(KB_ALLOCATED)`
    KbAllocated,
}

/// Age filter.
#[derive(Debug)]
pub enum Age {
    /// Age in days.
    Days(u32),
}

/// Relation.
#[derive(Debug)]
pub enum X {
    /// Greater.
    G,

    /// Less.
    L,
}

impl fmt::Display for X {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::G => write!(f, ">"),
            Self::L => write!(f, "<"),
        }
    }
}

/// Filter.
#[derive(Debug)]
pub enum Where {
    /// `WHERE DAYS(CURRENT_TIMESTAMP) - DAYS(ACCESS_TIME) < 365`
    Access(X, Age),

    /// `WHERE GROUP_ID = {0}`
    Group(gid_t),

    /// `WHERE DAYS(CURRENT_TIMESTAMP) - DAYS(MODIFICATION_TIME) < 365`
    Modification(X, Age),

    /// `WHERE USER_ID = {0}`
    User(uid_t),

    /// `WHERE {0}`
    Free(String),
}

/// Policy rule types.
#[derive(Debug)]
pub enum RuleType {
    /// `RULE EXTERNAL LIST`
    ExternalList(Name, Exec),

    /// `RULE LIST`
    List(Name, DirectoriesPlus, Vec<Show>, Option<Where>),
}
