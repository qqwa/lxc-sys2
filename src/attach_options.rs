use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr::null_mut;

/// LXC environment policy
///
/// ---
/// **version:** 1.0.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum lxc_attach_env_policy_t {
    /// Retain the environment
    ///
    /// ---
    /// **version:** 1.0.0
    LXC_ATTACH_KEEP_ENV,
    /// Clear the environment
    ///
    /// ---
    /// **version:** 1.0.0
    LXC_ATTACH_CLEAR_ENV,
}

// Options on by default
/// Move to cgroup
///
/// ---
/// **version:** 1.0.0
pub const LXC_ATTACH_MOVE_TO_CGROUP: u32 = 0x00000001;
/// Drop capabilities
///
/// ---
/// **version:** 1.0.0
pub const LXC_ATTACH_DROP_CAPABILITIES: u32 = 0x00000002;
/// Set personality
///
/// ---
/// **version:** 1.0.0
pub const LXC_ATTACH_SET_PERSONALITY: u32 = 0x00000004;
/// Execute under a Linux Security Module
///
/// ---
/// **version:** 1.0.0
pub const LXC_ATTACH_LSM_EXEC: u32 = 0x00000008;

// Options off by default
/// Remount /proc filesystem
///
/// ---
/// **version:** 1.0.0
pub const LXC_ATTACH_REMOUNT_PROC_SYS: u32 = 0x00010000;
/// FIXME: unknown
///
/// ---
/// **version:** 1.0.0
pub const LXC_ATTACH_LSM_NOW: u32 = 0x00020000;
/// Set PR_SET_NO_NEW_PRIVS to block execve() gainable privileges.
///
/// ---
/// **version:** 2.1.0
pub const LXC_ATTACH_NO_NEW_PRIVS: u32 = 0x00040000;
/// Allocate new terminal for attached process.
///
/// ---
/// **version:** 3.0.0
pub const LXC_ATTACH_TERMINAL: u32 = 0x00080000;

/// Mask of flags to apply by default
///
/// ---
/// **version:** 1.0.0
pub const LXC_ATTACH_DEFAULT: u32 = 0x0000FFFF;

/// All Linux Security Module flags
///
/// ---
/// **version:** 1.0.0
pub const LXC_ATTACH_LSM: u32 = LXC_ATTACH_LSM_EXEC | LXC_ATTACH_LSM_NOW;

/// LXC attach function type.
///
/// Function to run in container
///
/// ---
/// **Parameters**
///
/// **payload** [lxc_attach_command_t] to run.
///
/// ---
/// **Returns**
///
/// Function should return `0` on success, and any other value to denote
/// failure.
///
/// ---
/// **version:** 1.0.0
pub type lxc_attach_exec_t = extern "C" fn(payload: *mut c_void) -> c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// LXC attach options for \ref lxc_container `attach()`.
///
/// ---
/// **version:** 1.0.0
pub struct lxc_attach_options_t {
    /// Any combination of LXC_ATTACH_* flags
    ///
    /// ---
    /// **version:** 1.0.0
    pub attach_flags: c_uint,
    /// The namespaces to attach to (CLONE_NEW... flags)
    ///
    /// ---
    /// **version:** 1.0.0
    pub namespaces: c_int,
    /// Initial personality (`-1` to autodetect).
    ///
    /// ---
    /// **warning:** This may be ignored if lxc is compiled without personality
    /// support)
    ///
    /// ---
    /// **version:** 1.0.0
    pub personality: c_long,
    /// Inital current directory, use `NULL` to use cwd.
    /// If the current directory does not exist in the container, the
    /// root directory will be used instead because of kernel defaults.
    ///
    /// ---
    /// **version:** 1.0.0
    pub initial_cwd: *mut c_char,
    /// The user-id to run as.
    ///
    /// ---
    /// **note:** Set to `-1` for default behaviour (init uid for userns
    /// containers or `0` (super-user) if detection fails).
    ///
    /// ---
    /// **version:** 1.0.0
    pub uid: c_uint,
    ///  The group-id to run as.
    ///
    /// ---
    /// **note:** Set to `-1` for default behaviour (init gid for userns
    /// containers or `0` (super-user) if detection fails).
    ///
    /// ---
    /// **version:** 1.0.0
    pub gid: c_uint,
    /// Environment policy
    ///
    /// ---
    /// **version:** 1.0.0
    pub env_policy: lxc_attach_env_policy_t,
    /// Extra environment variables to set in the container environment
    ///
    /// ---
    /// **version:** 1.0.0
    pub extra_env_vars: *mut *mut c_char,
    /// Names of environment variables in existing environment to retain
    /// in container environment.
    ///
    /// ---
    /// **version:** 1.0.0
    pub extra_keep_env: *mut *mut c_char,

    /// stdin file descriptor
    ///
    /// ---
    /// **version:** 1.0.0
    pub stdin_fd: c_int,
    /// stdout file descriptor
    ///
    /// ---
    /// **version:** 1.0.0
    pub stdout_fd: c_int,
    /// stderr file descriptor
    ///
    /// ---
    /// **version:** 1.0.0
    pub stderr_fd: c_int,
    /// File descriptor to log output.
    ///
    /// ---
    /// **version:** 3.4.0
    pub log_fd: c_int,
}

/// Default attach options to use
///
/// **log_fd** is set to `-EBADF`, which is `-9`. `EBADF` being a
/// Bad file descriptor
///
/// ---
/// **version:** 1.0.0
impl std::default::Default for lxc_attach_options_t {
    fn default() -> Self {
        lxc_attach_options_t {
            attach_flags: LXC_ATTACH_DEFAULT,
            namespaces: -1,
            personality: -1,
            initial_cwd: null_mut(),
            uid: c_uint::MAX,
            gid: c_uint::MAX,
            env_policy: lxc_attach_env_policy_t::LXC_ATTACH_KEEP_ENV,
            extra_env_vars: null_mut(),
            extra_keep_env: null_mut(),
            stdin_fd: 0,
            stdout_fd: 1,
            stderr_fd: 2,
            log_fd: -9,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// Representation of a command to run in a container.
///
/// ---
/// **version:** 1.0.0
pub struct lxc_attach_command_t {
    /// The program to run (passed to execvp)
    ///
    /// ---
    /// **version:** 1.0.0
    pub program: *mut c_char,
    /// The argv pointer of that program, including the program itself in
    /// argv\[0\]
    ///
    /// ---
    /// **version:** 1.0.0
    pub argv: *mut *mut c_char,
}

extern "C" {
    /// Run a command in the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **payload** [lxc_attach_command_t] to run.
    ///
    /// ---
    /// **Returns**
    ///
    /// `-1` on error, exit code of lxc_attach_command_t program on success.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn lxc_attach_run_command(payload: *mut c_void) -> c_int;

    /// Run a shell command in the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **payload** Not used.
    ///
    /// ---
    /// **Returns**
    ///
    /// Exit code of shell.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn lxc_attach_run_shell(payload: *mut c_void) -> c_int;
}
