use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Lock {
    role: RoleName,
    agent: String,
    status: LockStatus,
    scopes: Vec<Scope>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RoleName(String);

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    path: String,
    access: ScopeAccess,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum ScopeAccess {
    Read,
    Edit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum LockStatus {
    Active,
    Idle,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct LockQuery {
    role: RolePattern,
    status: LockStatusPattern,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum RolePattern {
    Any,
    Exact(RoleName),
    Bind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum LockStatusPattern {
    Any,
    Exact(LockStatus),
    Bind,
}

impl Lock {
    pub fn new(
        role: RoleName,
        agent: impl Into<String>,
        status: LockStatus,
        scopes: Vec<Scope>,
    ) -> Self {
        Self {
            role,
            agent: agent.into(),
            status,
            scopes,
        }
    }
}

impl RoleName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Scope {
    pub fn new(path: impl Into<String>, access: ScopeAccess) -> Self {
        Self {
            path: path.into(),
            access,
        }
    }
}

impl LockQuery {
    pub fn new(role: RolePattern, status: LockStatusPattern) -> Self {
        Self { role, status }
    }
}
