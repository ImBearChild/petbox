use getset::{CopyGetters, Getters, Setters};
use std::path::PathBuf;
use std::ffi::{OsStr, OsString};

#[derive(Default, Clone, Copy)]
pub enum NamespaceType {
    Mount,
    Cgroup,
    Uts,
    Ipc,
    #[default]
    User,
    Pid,
    Network,
    Time,
}

#[derive(Getters, Setters, CopyGetters, Default, Clone)]
/// LinuxNamespace is the configuration for a Linux namespace.
pub struct Namespace {
    #[getset(get_copy = "pub", set = "pub")]
    /// Type is the type of namespace.
    pub(crate) typ: NamespaceType,

    #[getset(get = "pub", set = "pub")]
    /// Path is a path to an existing namespace persisted on disk that can
    /// be joined and is of the same type
    pub(crate) fd: Option<std::os::fd::RawFd>,
}

impl Namespace {
    pub fn new(typ: NamespaceType) -> Self {
        Self { typ , fd: None }
    }
}

#[derive(Getters, Setters, CopyGetters, Default, Clone)]
pub struct Root {
    #[getset(get = "pub", set = "pub")]
    path: PathBuf,

    #[getset(get = "pub", set = "pub")]
    readonly: Option<bool>,
}

#[derive(Getters, Setters, CopyGetters, Default, Clone)]
pub struct Mount {
    #[getset(get = "pub", set = "pub")]
    destination: PathBuf,
    // Path values for bind mounts are either absolute or relative to the
    // bundle. A mount is a bind mount if it has either bind or rbind in the options.
    #[getset(get = "pub", set = "pub")]
    typ: Option<String>,
    #[getset(get = "pub", set = "pub")]
    source: Option<PathBuf>,
    #[getset(get = "pub", set = "pub")]
    options: Option<Vec<String>>,
}

#[derive(Getters, Setters, CopyGetters, Default, Clone)]
/// Process contains information to start a specific application inside the
/// container.
pub struct Process {
    #[getset(get = "pub", set = "pub")]
    /// User specifies user information for the process.
    user: Option<User>,

    #[getset(get = "pub", set = "pub")]
    bin: OsString,

    #[getset(get = "pub", set = "pub")]
    /// Args specifies the arguments for the application to
    /// execute.
    args: Vec<String>,

    #[getset(get = "pub", set = "pub")]
    /// Env populates the process environment for the process.
    env: Option<Vec<String>>,

    #[getset(get = "pub", set = "pub")]
    /// Cwd is the current working directory for the process and must be
    /// relative to the container's root.
    cwd: PathBuf,
}

#[derive(Getters, Setters, CopyGetters, Default, Clone)]
pub struct User {
    #[getset(get_copy = "pub", set = "pub")]
    /// UID is the user id.
    uid: u32,

    #[getset(get_copy = "pub", set = "pub")]
    /// GID is the group id.
    gid: u32,
}

#[derive(Getters, Setters, CopyGetters, Default, Clone)]
/// LinuxIDMapping specifies UID/GID mappings.
pub struct LinuxIdMapping {
    #[getset(get_copy = "pub", set = "pub")]
    /// HostID is the starting UID/GID on the host to be mapped to
    /// `container_id`.
    host_id: u32,
    #[getset(get_copy = "pub", set = "pub")]
    /// ContainerID is the starting UID/GID in the container.
    container_id: u32,

    #[getset(get_copy = "pub", set = "pub")]
    /// Size is the number of IDs to be mapped.
    size: u32,
}