use alloc::vec::Vec;
use rustix_futex_sync::Mutex;

/// Functions registered with `at_fork`.
static FORK_FUNCS: Mutex<RegisteredForkFuncs> = Mutex::new(RegisteredForkFuncs::new());

/// A type for holding `fork` callbacks.
#[derive(Default)]
struct RegisteredForkFuncs {
    /// Functions called before calling `fork`.
    pub(crate) prepare: Vec<unsafe extern "C" fn()>,

    /// Functions called after calling `fork`, in the parent.
    pub(crate) parent: Vec<unsafe extern "C" fn()>,

    /// Functions called after calling `fork`, in the child.
    pub(crate) child: Vec<unsafe extern "C" fn()>,
}

impl RegisteredForkFuncs {
    pub(crate) const fn new() -> Self {
        Self {
            prepare: Vec::new(),
            parent: Vec::new(),
            child: Vec::new(),
        }
    }
}

/// Register functions to be called when `fork` is called.
///
/// The handlers for each phase are called in the following order:
/// - the prepare handlers are called in reverse order of registration;
/// - the parent and child handlers are called in the order of registration.
pub(crate) fn at_fork(
    prepare_func: Option<unsafe extern "C" fn()>,
    parent_func: Option<unsafe extern "C" fn()>,
    child_func: Option<unsafe extern "C" fn()>,
) {
    let mut funcs = FORK_FUNCS.lock();

    // Add the callbacks to the lists.
    funcs.prepare.extend(prepare_func);
    funcs.parent.extend(parent_func);
    funcs.child.extend(child_func);
}

/// Fork implementation.
///
/// # Safety
///
/// Wildly unsafe. See the documentation comment for [`rustix::runtime::fork`].
/// On top of that, this calls the unsafe functions registered with
/// [`at_fork`].
pub(crate) unsafe fn fork() -> rustix::io::Result<Option<rustix::process::Pid>> {
    let funcs = FORK_FUNCS.lock();

    // Callbacks before calling `fork`.
    funcs.prepare.iter().rev().for_each(|func| func());

    // Call `fork`.
    match rustix::runtime::fork()? {
        rustix::runtime::Fork::Child(pid) => {
            // The child's thread record is copied from the parent;
            // update it with the child's current-thread-id.
            #[cfg(feature = "thread")]
            origin::thread::set_current_thread_id_after_a_fork(pid);
            #[cfg(not(feature = "thread"))]
            let _ = pid;

            // Callbacks after calling `fork`, in the child.
            funcs.child.iter().for_each(|func| func());
            Ok(None)
        }
        rustix::runtime::Fork::Parent(child_pid) => {
            // Callbacks after calling `fork`, in the parent.
            funcs.parent.iter().for_each(|func| func());
            Ok(Some(child_pid))
        }
    }
}
