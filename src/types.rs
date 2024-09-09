use libc::{gid_t, uid_t};

/// Policy with rules.
#[derive(Debug)]
pub struct Policy {
    /// The name will be used when running the policy.
    pub name: Name,

    /// The rules of the policy.
    pub rules: Vec<Rule>,
}

/// Constructors.
impl Policy {
    /// Returns an empty, named policy.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Name(name.into()),
            rules: vec![],
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

/// Filter.
#[derive(Debug)]
pub enum Where {
    /// `WHERE GROUP_ID = {0}`
    Group(gid_t),

    /// `WHERE USER_ID = {0}`
    User(uid_t),
}

/// Policy rule types.
#[derive(Debug)]
pub enum RuleType {
    /// `RULE EXTERNAL LIST`
    ExternalList(Name, Exec),

    /// `RULE LIST`
    List(Name, DirectoriesPlus, Vec<Show>, Option<Where>),
}
