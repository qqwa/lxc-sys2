use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};

/// Do not edit the rootfs to change the hostname
///
/// ---
/// **version:** 1.0.0
pub const LXC_CLONE_KEEPNAME: u32 = 1 << 0;
/// Do not change the MAC address on network interfaces
///
/// ---
/// **version:** 1.0.0
pub const LXC_CLONE_KEEPMACADDR: u32 = 1 << 1;
/// Snapshot the original filesystem(s)
///
/// ---
/// **version:** 1.0.0
pub const LXC_CLONE_SNAPSHOT: u32 = 1 << 2;
/// Use the same bdev type
///
/// ---
/// **version:** 1.0.0
pub const LXC_CLONE_KEEPBDEVTYPE: u32 = 1 << 3;
/// Snapshot only if bdev supports it, else copy
///
/// ---
/// **version:** 1.0.0
pub const LXC_CLONE_MAYBE_SNAPSHOT: u32 = 1 << 4;
/// Number of `LXC_CLONE_*` flags
///
/// ---
/// **version:** 1.0.0
pub const LXC_CLONE_MAXFLAGS: u32 = 1 << 5;
/// allow snapshot creation even if source container is running
///
/// ---
/// **version:** 4.0.0
pub const LXC_CLONE_ALLOW_RUNNING: u32 = 1 << 6;
/// Redirect `stdin` to `/dev/zero` and `stdout` and `stderr` to `/dev/null`
///
/// ---
/// **version:** 1.0.0
pub const LXC_CREATE_QUIET: u32 = 1 << 0;
/// Number of `LXC_CREATE*` flags
///
/// ---
/// **version:** 1.0.0
pub const LXC_CREATE_MAXFLAGS: u32 = 1 << 1;
/// No doc
///
/// ---
/// **version:** 4.0.0
pub const LXC_MOUNT_API_V1: u32 = 1;

/// Internal struct
///
/// ---
/// **version:** 1.0.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lxc_lock {
    _unused: [u8; 0],
}
/// lxc_mount struct
///
/// ---
/// **version:** 4.0.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lxc_mount {
    /// No Doc
    ///
    /// ---
    /// **version:** 4.0.0
    pub version: c_int,
}

/// An LXC container.
///
/// ---
/// **version:** 1.0.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lxc_container {
    /// Name of container.
    name: *mut c_char,
    /// Full path to configuration file
    configfile: *mut c_char,
    /// File to store pid.
    pidfile: *mut c_char,
    /// Container semaphore lock.
    slock: *mut lxc_lock,
    /// Container private lock.
    privlock: *mut lxc_lock,
    /// Number of references to this container.
    ///
    /// ---
    /// **note:** protected by privlock.
    numthreads: c_int,
    /// Container configuration.
    lxc_conf: *mut c_void,

    /// Human-readable string representing last error
    ///
    /// ---
    /// **version:** 1.0.0
    pub error_string: *mut c_char,
    /// Last error number
    ///
    /// ---
    /// **version:** 1.0.0
    pub error_num: c_int,
    /// Whether container wishes to be daemonized
    ///
    /// ---
    /// **version:** 1.0.0
    pub daemonize: bool,
    /// Full path to configuration file
    ///
    /// ---
    /// **version:** 1.0.0
    pub config_path: *mut c_char,

    /// Determine if `/var/lib/lxc/$name/config` exists.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` if container is defined, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub is_defined: unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// Determine state of container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// Static upper-case string representing state of container.
    ///
    /// ---
    /// **note:** Returned string must not be freed.
    ///
    /// ---
    /// **version:** 1.0.0
    pub state: unsafe extern "C" fn(c: *mut lxc_container) -> *const c_char,

    /// Determine if container is running.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub is_running: unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// Freeze running container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub freeze: unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// Thaw a frozen container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub unfeeze: unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// Determine process ID of the containers init process.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// pid of init process as seen from outside the container.
    ///
    /// ---
    /// **version:** 1.0.0
    pub init_pid: unsafe extern "C" fn(c: *mut lxc_container) -> c_uint,

    /// Load the specified configuration for the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **alt_file** Full path to alternate configuration file, or
    /// `NULL` to use the default configuration file.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub load_config: unsafe extern "C" fn(
        c: *mut lxc_container,
        alt_file: *const c_char,
    ) -> bool,

    /// Start the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **useinit** Use lxcinit rather than `/sbin/init`.
    ///
    /// **argv** Array of arguments to pass to init.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub start: unsafe extern "C" fn(
        c: *mut lxc_container,
        useinit: c_int,
        argv: *const *mut c_char,
    ) -> bool,

    /// Start the container (list variant).
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **useinit** Use lxcinit rather than `/sbin/init`.
    ///
    /// **...** Command-line to pass to init (must end in `NULL`).
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **note:** Identical to [start](#structfield.start) except that that the
    /// init arguments are specified via a list rather than an array of
    /// pointers.
    ///
    /// ---
    /// **version:** 1.0.0
    pub start1: unsafe extern "C" fn(
        c: *mut lxc_container,
        useinit: c_int,
        ...
    ) -> bool,

    /// Stop the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub stop: unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// Change whether the container wants to run disconnected
    /// from the terminal.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **state** Value for the daemonize bit (0 or 1).
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub want_daemonize:
        unsafe extern "C" fn(c: *mut lxc_container, state: bool) -> bool,

    /// Change whether the container wishes all file descriptors
    /// to be closed on startup.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **state** Value for the close_all_fds bit (0 or 1).
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub want_close_all_fds:
        unsafe extern "C" fn(c: *mut lxc_container, state: bool) -> bool,

    /// Return current config file name.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// config file name, or `NULL` on error.
    ///
    /// ---
    /// **note:** The result is allocated, so the caller must free the result.
    ///
    /// ---
    /// **version:** 1.0.0
    pub config_file_name:
        unsafe extern "C" fn(c: *mut lxc_container) -> *mut c_char,

    /// Wait for container to reach a particular state.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **state** State to wait for.
    ///
    /// **timeout** Timeout in seconds.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` if state reached within `timeout`, else `false`.
    ///
    /// ---
    /// **note:** A `timeout` of `-1` means wait forever. A `timeout`
    /// of `0` means do not wait.
    ///
    /// ---
    /// **version:** 1.0.0
    pub wait: unsafe extern "C" fn(
        c: *mut lxc_container,
        state: *const c_char,
        timeout: c_int,
    ) -> bool,

    /// Set a key/value configuration option.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **key** Name of option to set.
    ///
    /// **value** Value of `name` to set.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub set_config_item: unsafe extern "C" fn(
        c: *mut lxc_container,
        key: *const c_char,
        value: *const c_char,
    ) -> bool,

    /// Delete the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **note:** Container must be stopped and have no dependent snapshots.
    ///
    /// ---
    /// **version:** 1.0.0
    pub destroy: unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// Save configuration to a file.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **alt_file** Full path to file to save configuration in.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub save_config: unsafe extern "C" fn(
        c: *mut lxc_container,
        alt_file: *const c_char,
    ) -> bool,

    /// Create a container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container (with lxcpath, name and a starting configuration set).
    ///
    /// **t** Template to execute to instantiate the root filesystem and adjust
    /// the configuration.
    ///
    /// **bdevtype** Backing store type to use (if `NULL`, `dir` will be used).
    ///
    /// **specs** Additional parameters for the backing store (for example LVM
    /// volume group to use).
    ///
    /// **flags** `LXC_CREATE_*` options (currently only
    /// [LXC_CREATE_QUIET](constant.LXC_CREATE_QUIET.html) is supported).
    ///
    /// **argv** Arguments to pass to the template, terminated by `NULL`
    /// (if no arguments are required, just pass `NULL)`.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub create: unsafe extern "C" fn(
        c: *mut lxc_container,
        t: *const c_char,
        bdevtype: *const c_char,
        specs: *mut bdev_specs,
        flags: c_int,
        argv: *const *mut c_char,
    ) -> bool,

    /// Create a container (list variant).
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container (with lxcpath, name and a starting configuration set).
    ///
    /// **t** Template to execute to instantiate the root filesystem and adjust
    /// the configuration.
    ///
    /// **bdevtype** Backing store type to use (if `NULL`, `dir` will be used).
    ///
    /// **specs** Additional parameters for the backing store (for example LVM
    /// volume group to use).
    ///
    /// **flags** `LXC_CREATE_*` options (currently only
    /// [LXC_CREATE_QUIET](constant.LXC_CREATE_QUIET.html) is supported).
    ///
    /// **...** Command-line to pass to init (must end in `NULL)`.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **note:** Identical to [create](#structfield.create) except that the
    /// template arguments are specified as a list rather than an array of
    /// pointers.
    ///
    /// ---
    /// **version:** 1.0.0
    pub createl: unsafe extern "C" fn(
        c: *mut lxc_container,
        t: *const c_char,
        bdevtype: *const c_char,
        specs: *mut bdev_specs,
        flags: c_int,
        ...
    ) -> bool,

    /// Rename a container
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **newname**  New name to be used for the container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub rename: unsafe extern "C" fn(
        c: *mut lxc_container,
        newname: *const c_char,
    ) -> bool,

    /// Request the container reboot by sending it `SIGINT`.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    /// `true` if reboot request successful, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub reboot: unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// Request the container shutdown by sending it `SIGPWR`.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **timeout** Seconds to wait before returning false. (-1 to wait forever,
    /// 0 to avoid waiting).
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` if the container was shutdown successfully, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub shutdown:
        unsafe extern "C" fn(c: *mut lxc_container, timeout: c_int) -> bool,

    /// Completely clear the containers in-memory configuration.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **version:** 1.0.0
    pub clear_config: unsafe extern "C" fn(c: *mut lxc_container) -> c_void,

    /// Clear a configuration item.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **key** Name of option to clear.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **note:** Analog of [set_config_item](#structfield.set_config_item).
    ///
    /// ---
    /// **version:** 1.0.0
    pub clear_config_item:
        unsafe extern "C" fn(c: *mut lxc_container, key: *const c_char) -> bool,

    /// Retrieve the value of a config item.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **key** Name of option to get.
    ///
    /// **retv** *out* Caller-allocated buffer to write value of `key` into (or
    /// `NULL` to determine length of value).
    ///
    /// **inlen** Length of `retv` (may be zero).
    ///
    /// ---
    /// **Returns**
    ///
    /// Length of config items value, or < 0 on error.
    ///
    /// ---
    /// **note:** The caller can (and should) determine how large a buffer to
    /// allocate for `retv` by initially passing its value as `NULL` and
    /// considering the return value. This function can then be called again
    /// passing a newly-allocated suitably-sized buffer.
    ///
    /// **note:** If `retv` is NULL, `inlen` is ignored.
    ///
    /// **note:** If `inlen` is smaller than required, nothing will be written
    /// to `retv` and still return the length of config item value.
    ///
    /// ---
    /// **version:** 1.0.0
    pub get_config_item: unsafe extern "C" fn(
        c: *mut lxc_container,
        key: *const c_char,
        retv: *mut c_char,
        inlen: c_int,
    ) -> c_int,

    /// Retrieve the value of a config item from running container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **key** Name of option to get.
    ///
    /// ---
    /// **Returns**
    ///
    /// the item or NULL on error.
    ///
    /// ---
    /// **note:** Returned string must be freed by the caller.
    ///
    /// ---
    /// **version:** 1.0.0
    pub get_running_config_item: unsafe extern "C" fn(
        c: *mut lxc_container,
        key: *const c_char,
    ) -> *mut c_char,

    /// Retrieve a list of config item keys given a key prefix
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **key** Name of option to get.
    ///
    /// **retv** *out* Caller-allocated buffer to write list of keys to (or
    /// `NULL` to determine overall length of keys list).
    ///
    /// **inlen** Length of `retv` (may be zero).
    ///
    /// ---
    /// **Returns**
    ///
    /// Length of keys list, or < 0 on error.
    ///
    /// ---
    /// **note:** The list values written to `retv` are separated by a newline
    /// character ('\\n').
    ///
    /// **note:** The caller can (and should) determine how large a buffer to
    /// allocate for `retv` by initially passing its value as `NULL` and
    /// considering the return value. This function can then be called again
    /// passing a newly-allocated suitably-sized buffer.
    ///
    /// **note:** If `retv` is NULL, `inlen` is ignored.
    ///
    /// **note:** If `inlen` is smaller than required, the value written to
    /// `retv` will be truncated.
    ///
    /// ---
    /// **version:** 1.0.0
    pub get_keys: unsafe extern "C" fn(
        c: *mut lxc_container,
        key: *const c_char,
        retv: *mut c_char,
        inlen: c_int,
    ) -> c_int,

    /// Obtain a list of network interfaces.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// Newly-allocated array of network interfaces, or `NULL` on error.
    ///
    /// ---
    /// **note:** The returned array is allocated, so the caller must free it.
    ///
    /// **note:** The returned array is terminated with a `NULL` entry.
    ///
    /// ---
    /// **version:** 1.0.0
    pub get_interfaces:
        unsafe extern "C" fn(c: *mut lxc_container) -> *mut *mut c_char,

    /// Determine the list of container IP addresses.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **interface** Network interface name to consider.
    ///
    /// **family** Network family (for example "inet", "inet6").
    ///
    /// **scope** IPv6 scope id (ignored if `family` is not "inet6").
    ///
    /// ---
    /// **Returns**
    ///
    /// Newly-allocated array of network interfaces, or `NULL` on error.
    ///
    /// ---
    /// **note:** The returned array is allocated, so the caller must free it.
    ///
    /// **note:** The returned array is terminated with a `NULL` entry.
    ///
    /// ---
    /// **version:** 1.0.0
    pub get_ips: unsafe extern "C" fn(
        c: *mut lxc_container,
        interface: *const c_char,
        family: *const c_char,
        scope: c_int,
    ) -> *mut *mut c_char,

    /// Retrieve the specified cgroup subsystem value for the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **subsys** cgroup subsystem to retrieve.
    ///
    /// **retv** *out* Caller-allocated buffer to write value of `subsys` into
    /// (or `NULL` to determine length of value).
    ///
    /// **inlen** length of `retv` (may be zero).
    ///
    /// ---
    /// **Returns**
    ///
    /// Length of `subsys` value, or < 0 on error.
    ///
    /// ---
    /// **note:** If `retv` is `NULL`, `inlen` is ignored.
    ///
    /// **note:** If `inlen` is smaller than required, the value written to
    /// `retv` will be truncated.
    ///
    /// ---
    /// **version:** 1.0.0
    pub get_cgroup_item: unsafe extern "C" fn(
        c: *mut lxc_container,
        subsys: *const c_char,
        retv: *mut c_char,
        inlen: c_int,
    ) -> c_int,

    /// Set the specified cgroup subsystem value for the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **subsys** cgroup subsystem to consider.
    ///
    /// **value** Value to set for `subsys`.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub set_cgroup_item: unsafe extern "C" fn(
        c: *mut lxc_container,
        subsys: *const c_char,
        value: *const c_char,
    ) -> bool,

    /// Determine full path to the containers configuration file.
    /// Each container can have a custom configuration path. However
    /// by default it will be set to either the `LXCPATH` configure
    /// variable, or the lxcpath value in the `LXC_GLOBAL_CONF` configuration
    /// file (i.e. `/etc/lxc/lxc`.conf).
    /// The value for a specific container can be changed using
    /// [set_config_path](#structfield.set_config_path). There is no other way
    /// to specify this in general at the moment.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// Static string representing full path to configuration file.
    ///
    /// ---
    /// **note:** Returned string must not be freed.
    ///
    /// ---
    /// **version:** 1.0.0
    pub get_config_path:
        unsafe extern "C" fn(c: *mut lxc_container) -> *const c_char,

    /// Set the full path to the containers configuration file.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **path** Full path to configuration file.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub set_config_path: unsafe extern "C" fn(
        c: *mut lxc_container,
        path: *const c_char,
    ) -> bool,

    /// Copy a stopped container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Original container.
    ///
    /// **newname** New name for the container. If `NULL`, the same name is
    /// used and a new lxcpath MUST be specified.
    ///
    /// **lxcpath** lxcpath in which to create the new container. If `NULL`,
    /// the original container's lxcpath will be used.
    ///
    /// **flags** Additional `LXC_CLONE*` flags to change the cloning behaviour:
    /// - [LXC_CLONE_KEEPNAME](constant.LXC_CLONE_KEEPNAME.html)
    /// - [LXC_CLONE_KEEPMACADDR](constant.LXC_CLONE_KEEPMACADDR.html)
    /// - [LXC_CLONE_SNAPSHOT](constant.LXC_CLONE_SNAPSHOT.html)
    ///
    /// **bdevtype** Optionally force the cloned bdevtype to a specified plugin.
    /// By default the original is used (subject to snapshot requirements).
    ///
    /// **bdevdata** Information about how to create the new storage (i.e.
    /// fstype and fsdata).
    ///
    /// **newsize** In case of a block device backing store, an optional size.
    /// If `0`, the original backing store's size will be used if possible. Note
    /// this only applies to the rootfs. For any other filesystems, the original
    /// size will be duplicated.
    ///
    /// **hookargs** Additional arguments to pass to the clone hook script.
    ///
    /// ---
    /// **Returns**
    ///
    /// Newly-allocated copy of container `c`, or `NULL` on error.
    ///
    /// ---
    /// **note:** If devtype was not specified, and `flags` contains
    /// [LXC_CLONE_SNAPSHOT](constant.LXC_CLONE_SNAPSHOT.html) then use the
    /// native `bdevtype` if possible, else use an overlayfs.
    ///
    /// ---
    /// **version:** 1.0.0
    pub clone: unsafe extern "C" fn(
        c: *mut lxc_container,
        newname: *const c_char,
        lxcpath: *const c_char,
        flags: c_int,
        bdevtype: *const c_char,
        bdevdata: *const c_char,
        newsize: u64,
        hookargs: *mut *mut c_char,
    ) -> *mut lxc_container,

    /// Allocate a console tty for the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **ttynum** *in,out* Terminal number to attempt to allocate, or `-1` to
    /// allocate the first available tty.
    ///
    /// **ptxfd** *out* File descriptor referring to the ptx side of the pty.
    ///
    /// ---
    /// **Returns**
    ///
    /// tty file descriptor number on success, or `-1` on failure.
    ///
    /// ---
    /// **note:** On successful return, `ttynum` will contain the tty number
    /// that was allocated.
    ///
    /// **note:** The returned file descriptor is used to keep the tty
    /// allocated. The caller should call close(2) on the returned file
    /// descriptor when no longer required so that it may be allocated
    /// by another caller.
    ///
    /// ---
    /// **version:** 1.0.0
    pub console_getfd: unsafe extern "C" fn(
        c: *mut lxc_container,
        ttynum: *mut c_int,
        ptxfd: *mut c_int,
    ) -> c_int,

    /// Allocate and run a console tty.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **ttynum** Terminal number to attempt to allocate, `-1` to
    /// allocate the first available tty or `0` to allocate the
    /// console.
    ///
    /// **stdinfd** File descriptor to read input from.
    ///
    /// **stdoutfd** File descriptor to write output to.
    ///
    /// **stderrfd** File descriptor to write error output to.
    ///
    /// **escape** The escape character (1 == 'a', 2 == 'b', ...).
    ///
    /// ---
    /// **Returns**
    ///
    /// `0` on success, `-1` on failure.
    ///
    /// ---
    /// **note:** This function will not return until the console has been
    /// exited by the user.
    ///
    /// ---
    /// **version:** 1.0.0
    pub console: unsafe extern "C" fn(
        c: *mut lxc_container,
        ttynum: c_int,
        stdinfd: c_int,
        stdoutfd: c_int,
        stderrfd: c_int,
        escape: c_int,
    ) -> c_int,

    /// Create a sub-process attached to a container and run a function inside
    /// it.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **exec_function** Function to run.
    ///
    /// **exec_payload** Data to pass to `exec_function`.
    ///
    /// **options** See
    /// [lxc_attach_options_t](struct.lxc_attach_options_t.html).
    ///
    /// **attached_process** *out* Process ID of process running inside
    /// container `c` that is running `exec_function`.
    ///
    /// ---
    /// **Returns**
    ///
    /// `0` on success, `-1` on error.
    ///
    /// ---
    /// **version:** 1.0.0
    pub attach: unsafe extern "C" fn(
        c: *mut lxc_container,
        exec_function: crate::attach_options::lxc_attach_exec_t,
        exec_payload: *mut c_void,
        options: *const crate::attach_options::lxc_attach_options_t,
        attached_process: *mut c_uint,
    ) -> c_int,

    /// Run a program inside a container and wait for it to exit.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **options** See
    /// [lxc_attach_options_t](struct.lxc_attach_options_t.html).
    ///
    /// **program** Full path inside container of program to run.
    ///
    /// **argv** Array of arguments to pass to `program`.
    ///
    /// ---
    /// **Returns**
    ///
    /// `waitpid(2)` status of exited process that ran `program`, or `-1` on
    /// error.
    ///
    /// ---
    /// **version:** 1.0.0
    pub attach_run_wait: unsafe extern "C" fn(
        c: *mut lxc_container,
        options: crate::attach_options::lxc_attach_options_t,
        program: *const c_char,
        argv: *const *const c_char,
    ) -> c_int,

    /// Run a program inside a container and wait for it to exit (list variant).
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **options** See
    /// [lxc_attach_options_t](struct.lxc_attach_options_t.html).
    ///
    /// **program** Full path inside container of program to run.
    ///
    /// **...** Command-line to pass to `program` (must end in `NULL)`.
    ///
    /// ---
    /// **Returns**
    ///
    /// `waitpid(2)` status of exited process that ran `program`, or `-1` on
    /// error.
    ///
    /// ---
    /// **version:** 1.0.0
    pub attach_run_wait1: unsafe extern "C" fn(
        c: *mut lxc_container,
        options: crate::attach_options::lxc_attach_options_t,
        program: *const c_char,
        ...
    ) -> c_int,

    /// Create a container snapshot.
    ///
    /// Assuming default paths, snapshots will be created as
    /// `/var/lib/lxc/<c>/snaps/snap<n>` where `<c>` represents the container
    /// name and `<n>` represents the zero-based snapshot number.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **commentfile** Full path to file containing a description of the
    /// snapshot.
    ///
    /// ---
    /// **Returns**
    ///
    /// -1 on error, or zero-based snapshot number.
    ///
    /// ---
    /// **note:** `commentfile` may be `NULL` but this is discouraged.
    ///
    /// ---
    /// **version:** 1.0.0
    pub snapshot: unsafe extern "C" fn(
        c: *mut lxc_container,
        commentfile: *const c_char,
    ) -> c_int,

    /// Obtain a list of container snapshots.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **snapshots** Dynamically-allocated Array of lxc_snapshot's.
    ///
    /// ---
    /// **Returns**
    ///
    /// Number of snapshots.
    ///
    /// ---
    /// **note:** The array returned in `snapshots` is allocated, so the caller
    /// must free it.
    ///
    /// **note:** To free an individual snapshot as returned in \p
    /// snapshots, call the snapshots `free` function (see
    /// `src/tests/snapshot.c` for an example).
    ///
    /// ---
    /// **version:** 1.0.0
    pub snapshot_list: unsafe extern "C" fn(
        c: *mut lxc_container,
        snapshots: *mut *mut lxc_snapshot,
    ) -> c_int,

    /// Create a new container based on a snapshot.
    ///
    /// The restored container will be a copy (not snapshot) of the snapshot,
    /// and restored in the lxcpath of the original container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **snapname** Name of snapshot.
    ///
    /// **newname** Name to be used for the restored snapshot.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **warning:** If `newname` is the same as the current container
    /// name, the container will be destroyed. However, this will
    /// fail if the  snapshot is overlay-based, since the snapshots
    /// will pin the original container.
    ///
    /// **note:** As an example, if the container exists as `/var/lib/lxc/c1`,
    /// snapname might be `snap0` (representing `/var/lib/lxc/c1/snaps/snap0)`.
    /// If `newname` is `c2`, then `snap0` will be copied to `/var/lib/lxc/c2`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub snapshot_restore: unsafe extern "C" fn(
        c: *mut lxc_container,
        snapname: *const c_char,
        newname: *const c_char,
    ) -> bool,

    /// Destroy the specified snapshot.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **snapname** Name of snapshot.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub snapshot_destroy: unsafe extern "C" fn(
        c: *mut lxc_container,
        snapname: *const c_char,
    ) -> bool,

    /// Determine if the caller may control the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `false` if there is a control socket for the container monitor and the
    /// caller may not access it, otherwise returns `true`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub may_control: unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// Add specified device to the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **src_path** Full path of the device.
    ///
    /// **dest_path** Alternate path in the container (or `NULL` to use
    /// `src_path)`.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub add_device_node: unsafe extern "C" fn(
        c: *mut lxc_container,
        src_path: *const c_char,
        dest_path: *const c_char,
    ) -> bool,

    /// Remove specified device from the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **src_path** Full path of the device.
    ///
    /// **dest_path** Alternate path in the container (or `NULL` to use
    /// `src_path)`.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.0.0
    pub remove_device_node: unsafe extern "C" fn(
        c: *mut lxc_container,
        src_path: *const c_char,
        dest_path: *const c_char,
    ) -> bool,

    /// Add specified netdev to the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **dev** name of net device.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.1.0
    pub attach_interface: unsafe extern "C" fn(
        c: *mut lxc_container,
        dev: *const c_char,
        dst_dev: *const c_char,
    ) -> bool,

    /// Remove specified netdev from the container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **dev** name of net device.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.1.0
    pub detach_interface: unsafe extern "C" fn(
        c: *mut lxc_container,
        dev: *const c_char,
        dst_dev: *const c_char,
    ) -> bool,

    /// Checkpoint a container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **directory** The directory to dump the container to.
    ///
    /// **stop** Whether or not to stop the container after checkpointing.
    ///
    /// **verbose** Enable criu's verbose logs.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.1.0
    pub checkpoint: unsafe extern "C" fn(
        c: *mut lxc_container,
        directory: *const c_char,
        stop: bool,
        verbose: bool,
    ) -> bool,

    /// Restore a container from a checkpoint.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **directory** The directory to restore the container from.
    ///
    /// **verbose** Enable criu's verbose logs.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.1.0
    pub restore: unsafe extern "C" fn(
        c: *mut lxc_container,
        directory: *const c_char,
        verbose: bool,
    ) -> bool,

    /// Delete the container and all its snapshots.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **note:** Container must be stopped.
    ///
    /// ---
    /// **version:** 1.1.0
    pub destroy_with_snapshots:
        unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// Destroy all the container's snapshot.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, else `false`.
    ///
    /// ---
    /// **version:** 1.1.0
    pub snapshot_destroy_all:
        unsafe extern "C" fn(c: *mut lxc_container) -> bool,

    /// An API call to perform various migration operations
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **cmd** One of the MIGRATE_ contstants
    ///
    /// **opts** A migrate_opts struct filled with relevant options.
    ///
    /// **size** The size of the migrate_opts struct, i.e.
    /// sizeof(struct migrate_opts).
    ///
    /// ---
    /// **Returns**
    ///
    /// `0` on success, nonzero on failure.
    ///
    /// ---
    /// **version:** 2.0.0
    pub migrate: unsafe extern "C" fn(
        c: *mut lxc_container,
        cmd: c_uint,
        opts: *mut migrate_opts,
        size: c_uint,
    ) -> c_int,

    /// Query the console log of a container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **log** A [lxc_console_log] struct filled with relevant options.
    ///
    /// ---
    /// **Returns**
    ///
    /// `0` on success, nonzero on failure.
    ///
    /// ---
    /// **version:** 3.0.0
    pub console_log: unsafe extern "C" fn(
        c: *mut lxc_container,
        log: *mut lxc_console_log,
    ) -> c_int,

    /// Request the container reboot by sending it `SIGINT`.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// **timeout** Seconds to wait before returning false.
    /// (-1 to wait forever, 0 to avoid waiting).
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` if the container was rebooted successfully, else `false`.
    ///
    /// ---
    /// **version:** 3.0.0
    pub reboot2:
        unsafe extern "C" fn(c: *mut lxc_container, timeout: c_int) -> bool,

    /// Mount the host's path `source` onto the container's path `target`.
    ///
    /// ---
    /// **version:** 4.0.0
    pub mount: unsafe extern "C" fn(
        c: *mut lxc_container,
        source: *mut c_char,
        target: *mut c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
        mnt: *mut lxc_mount,
    ) -> bool,

    /// Unmount the container's path `target`.
    ///
    /// ---
    /// **version:** 4.0.0
    pub umount: unsafe extern "C" fn(
        c: *mut lxc_container,
        target: *mut c_char,
        mountflags: c_ulong,
        mnt: *mut lxc_mount,
    ) -> bool,

    /// Retrieve a file descriptor for the container's seccomp filter.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// file descriptor for container's seccomp filter
    ///
    /// ---
    /// **version:** 4.0.0
    pub seccomp_notify_fd: unsafe extern "C" fn(c: *mut lxc_container) -> c_int,

    /// Retrieve a pidfd for the container's init process.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// pidfd of init process of the container.
    ///
    /// ---
    /// **version:** 4.0.0
    pub init_pidfd: unsafe extern "C" fn(c: *mut lxc_container) -> c_int,
}

/// An LXC container snapshot.
///
/// ---
/// **version:** 1.0.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lxc_snapshot {
    /// Name of snapshot
    ///
    /// ---
    /// **version:** 1.0.0
    pub name: *mut c_char,
    /// Full path to snapshots comment file (may be `NULL)`
    ///
    /// ---
    /// **version:** 1.0.0
    pub comment_pathname: *mut c_char,
    /// Time snapshot was created
    ///
    /// ---
    /// **version:** 1.0.0
    pub timestamp: *mut c_char,
    /// Full path to LXCPATH for snapshot
    ///
    /// ---
    /// **version:** 1.0.0
    pub lxcpath: *mut c_char,

    /// De-allocate the snapshot.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **s** Snapshot.
    ///
    /// ---
    /// **version:** 1.0.0
    pub free: unsafe extern "C" fn(s: *mut lxc_snapshot) -> c_void,
}

/// Specifications for how to create a new backing store
///
/// ---
/// **version:** 1.0.4
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bdev_specs {
    /// Filesystem type
    ///
    /// ---
    /// **version:** 1.0.4
    pub fstype: *mut c_char,
    /// Filesystem size in bytes
    ///
    /// ---
    /// **version:** 1.0.4
    pub fssize: u64,
    /// See [zfs].
    ///
    /// ---
    /// **version:** 1.0.4
    pub zfs: zfs,
    /// See [lvm].
    ///
    /// ---
    /// **version:** 1.0.4
    pub lvm: lvm,
    /// Directory path
    ///
    /// ---
    /// **version:** 1.0.4
    pub dir: *mut c_char,
    /// See [rbd].
    ///
    /// ---
    /// **version:** 2.0.0
    pub rbd: rbd,
}

/// Internal struct used by [bdev_specs]
///
/// ---
/// **version:** 1.0.4
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zfs {
    /// ZFS root path
    ///
    /// ---
    /// **version:** 1.0.4
    pub zfsroot: *mut c_char,
}

/// Internal struct used by [bdev_specs]
///
/// ---
/// **version:** 1.0.4
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lvm {
    /// LVM Volume Group name
    ///
    /// ---
    /// **version:** 1.0.4
    pub vg: *mut c_char,
    /// LVM Logical Volume name
    ///
    /// ---
    /// **version:** 1.0.4
    pub lv: *mut c_char,
    /// LVM thin pool to use, if any
    ///
    /// ---
    /// **version:** 1.0.4
    pub thinpool: *mut c_char,
}

/// Internal struct used by [bdev_specs]
///
/// ---
/// **version:** 2.0.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rbd {
    /// RBD image name
    ///
    /// ---
    /// **version:** 2.0.0
    pub rbdname: *mut c_char,
    /// Ceph pool name
    ///
    /// ---
    /// **version:** 2.0.0
    pub rbdpool: *mut c_char,
}

/// Command for the migrate API call.
///
/// ---
/// **version:** 2.0.0
pub const MIGRATE_PRE_DUMP: u32 = 0;
/// Command for the migrate API call.
///
/// ---
/// **version:** 2.0.0
pub const MIGRATE_DUMP: u32 = 1;
/// Command for the migrate API call.
///
/// ---
/// **version:** 2.0.0
pub const MIGRATE_RESTORE: u32 = 2;
/// Command for the migrate API call.
///
/// ---
/// **version:** 3.0.0
pub const MIGRATE_FEATURE_CHECK: u32 = 3;

/// Available feature checks.
///
/// ---
/// **version:** 3.0.0
pub const FEATURE_MEM_TRACK: u32 = 1 << 0;
/// Available feature checks.
///
/// ---
/// **version:** 3.0.0
pub const FEATURE_LAZY_PAGES: u32 = 1 << 1;

/// Options for the migrate API call.
///
/// ---
/// **version:** 2.0.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct migrate_opts {
    /// ---
    /// **version:** 2.0.0
    pub directory: *mut c_char,
    /// ---
    /// **version:** 2.0.0
    pub verbose: bool,

    /// stop the container after dump?
    ///
    /// ---
    /// **version:** 2.0.0
    pub stop: bool,

    /// relative to directory above
    ///
    /// ---
    /// **version:** 2.0.0
    pub predump_dir: *mut c_char,

    /// where should memory pages be send?
    ///
    /// ---
    /// **version:** 2.0.0
    pub pageserver_address: *mut c_char,

    /// ---
    /// **version:** 2.0.0
    pub pageserver_port: *mut c_char,

    /// This flag indicates whether or not the container's rootfs will have
    /// the same inodes on checkpoint and restore. In the case of e.g. zfs
    /// send or btrfs send, or an LVM snapshot, this will be true, but it
    /// won't if e.g. you rsync the filesystems between two machines.
    ///
    /// ---
    /// **version:** 2.0.1
    pub preserves_inodes: bool,

    /// Path to an executable script that will be registered as a criu
    /// "action script"
    ///
    /// ---
    /// **version:** 2.0.4
    pub action_script: *mut c_char,

    /// If CRIU >= 2.4 is detected the option to skip in-flight connections
    /// will be enabled by default. The flag 'disable_skip_in_flight' will
    /// unconditionally disable this feature. In-flight connections are
    /// not fully established TCP connections: SYN, SYN-ACK
    ///
    /// ---
    /// **version:** 2.0.4
    pub disable_skip_in_flight: bool,

    /// This is the maximum file size for deleted files (which CRIU calls
    /// "ghost" files) that will be handled. 0 indicates the CRIU default,
    /// which at this time is 1MB.
    ///
    /// ---
    /// **version:** 2.0.4
    pub ghost_limit: u64,

    /// Some features cannot be checked by comparing the CRIU version.
    /// Features like dirty page tracking or userfaultfd depend on
    /// the architecture/kernel/criu combination. This is a bitmask
    /// in which the desired feature checks can be encoded.
    ///
    /// ---
    /// **version:** 3.0.0
    pub features_to_check: u64,
}

/// Doc missing
///
/// ---
/// **version:** 3.0.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lxc_console_log {
    /// Clear the console log.
    ///
    /// ---
    /// **version:** 3.0.0
    pub clear: bool,

    /// Retrieve the console log.
    ///
    /// ---
    /// **version:** 3.0.0
    pub read: bool,

    /// This specifies the maximum size to read from the ringbuffer. Setting
    /// it to 0 means that the a read can be as big as the whole ringbuffer.
    /// On return callers can check how many bytes were actually read.
    /// If "read" and "clear" are set to false and a non-zero value is
    /// specified then up to "read_max" bytes of data will be discarded from
    /// the ringbuffer.
    ///
    /// ---
    /// **version:** 3.0.0
    pub read_max: *mut u64,

    /// Data that was read from the ringbuffer. If "read_max" is 0 on return
    /// "data" is invalid.
    ///
    /// ---
    /// **version:** 3.0.0
    pub data: *mut c_char,
}

extern "C" {
    /// Create a new container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **name** Name to use for container.
    ///
    /// **configpath** Full path to configuration file to use.
    ///
    /// ---
    /// **Returns**
    ///
    /// Newly-allocated container, or `NULL` on error.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn lxc_container_new(
        name: *const c_char,
        configpath: *const c_char,
    ) -> *mut lxc_container;

    /// Add a reference to the specified container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `true` on success, `false` on error.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn lxc_container_get(lxc_container: *mut lxc_container) -> c_int;

    /// Drop a reference to the specified container.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **c** Container.
    ///
    /// ---
    /// **Returns**
    ///
    /// `0` on success, `1` if reference was successfully dropped and container
    /// has been freed, and `-1` on error.
    ///
    /// ---
    /// **warning:** If `1` is returned, `c` is no longer valid.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn lxc_container_put(lxc_container: *mut lxc_container) -> c_int;

    /// Obtain a list of all container states.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **states** *out* states Caller-allocated array to hold all states
    /// (may be `NULL)`.
    ///
    /// ---
    /// **Returns**
    ///
    /// Number of container states.
    ///
    /// ---
    /// **note:** Passing `NULL` for `states` allows the caller to first
    /// calculate how many states there are before calling the function again,
    /// the second time providing a suitably-sized array to store the static
    /// string pointers in.
    ///
    /// **note:** The `states` array should be freed by the caller, but not the
    /// strings the elements point to.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn lxc_get_wait_states(states: *mut *const c_char) -> c_int;

    /// Get the value for a global config key
    ///
    /// ---
    /// **Parameters**
    ///
    /// **key** The name of the config key
    ///
    /// ---
    /// **Returns**
    ///
    /// String representing the current value for the key.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn lxc_get_global_config_item(key: *const c_char) -> *const c_char;

    /// Determine version of LXC.
    ///
    /// ---
    /// **Returns**
    ///
    /// Static string representing version of LXC in use.
    ///
    /// ---
    /// **note:** Returned string must not be freed.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn lxc_get_version() -> *const c_char;

    /// Get a list of defined containers in a lxcpath.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **lxcpath** lxcpath under which to look.
    ///
    /// **names** If not `NULL`, then a list of container names will be returned
    /// here.
    ///
    /// **cret** If not `NULL`, then a list of lxc_containers will be returned
    /// here.
    ///
    /// ---
    /// **Returns**
    ///
    /// Number of containers found, or `-1` on error.
    ///
    /// ---
    /// **note:** Values returned in `cret` are sorted by container name.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn list_defined_containers(
        lxcpath: *const c_char,
        names: *mut *mut *mut c_char,
        cret: *mut *mut *mut lxc_container,
    ) -> c_int;

    /// Get a list of active containers for a given lxcpath.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **lxcpath** Full `LXCPATH` path to consider.
    ///
    /// **names** *out* Dynamically-allocated array of container names.
    ///
    /// **cret** *out* Dynamically-allocated list of containers.
    ///
    /// ---
    /// **Returns**
    /// Number of containers found, or -1 on error.
    ///
    /// ---
    /// **note:** Some of the containers may not be "defined".
    ///
    /// **note:** Values returned in `cret` are sorted by container name.
    ///
    /// **note:** `names` and `cret` may both (or either) be specified as
    /// `NULL`.
    ///
    /// **note:** `names` and `cret` must be freed by the caller.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn list_active_containers(
        lxcpath: *const c_char,
        names: *mut *mut *mut c_char,
        cret: *mut *mut *mut lxc_container,
    ) -> c_int;

    /// Get a complete list of all containers for a given lxcpath.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **lxcpath** Full `LXCPATH` path to consider.
    ///
    /// **names** *out* Dynamically-allocated array of container names.
    ///
    /// **cret** *out* Dynamically-allocated list of containers.
    ///
    /// ---
    /// **Returns**
    /// Number of containers found, or -1 on error.
    ///
    /// ---
    /// **note:** Some of the containers may not be "defined".
    ///
    /// **note:** Values returned in `cret` are sorted by container name.
    ///
    /// **note:** `names` and `cret` may both (or either) be specified as
    /// `NULL`.
    ///
    /// **note:** `names` and `cret` must be freed by the caller.
    ///
    /// ---
    /// **version:** 1.0.0
    pub fn list_all_containers(
        lxcpath: *const c_char,
        names: *mut *mut *mut c_char,
        cret: *mut *mut *mut lxc_container,
    ) -> c_int;
}

/// ---
/// **version:** 2.1.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lxc_log {
    /// ---
    /// **version:** 2.1.0
    pub name: *const c_char,

    /// ---
    /// **version:** 2.1.0
    pub lxcpath: *const c_char,

    /// ---
    /// **version:** 2.1.0
    pub file: *const c_char,

    /// ---
    /// **version:** 2.1.0
    pub level: *const c_char,

    /// ---
    /// **version:** 2.1.0
    pub prefix: *const c_char,

    /// ---
    /// **version:** 2.1.0
    pub quiet: bool,
}

extern "C" {
    /// Initialize the log
    ///
    /// ---
    /// **Parameters**
    ///
    /// **log** lxc log configuration.
    ///
    /// ---
    /// **version:** 2.1.0
    pub fn lxc_log_init(log: *mut lxc_log) -> c_int;

    /// Close log file.
    ///
    /// ---
    /// **version:** 1.0.2
    pub fn lxc_log_close();

    /// Check if the configuration item is supported by this LXC instance.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **key** Configuration item to check for.
    ///
    /// ---
    /// **version:** 2.1.0
    pub fn lxc_config_item_is_supported(key: *mut c_char) -> bool;

    /// Check if an API extension is supported by this LXC instance.
    ///
    /// ---
    /// **Parameters**
    ///
    /// **extension** API extension to check for.
    ///
    /// ---
    /// **version:** 4.0.0
    pub fn lxc_has_api_extension(extension: *mut c_char) -> bool;
}
