use roboprec::ir::{helper::clear_all_names, program::clear_program};
use std::sync::{Mutex, OnceLock};

// only one test should run at a time
static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn setup_default_test() {
    clear_all_names();
    clear_program();
}

fn teardown_default_test() {
}

// Run setup_default_test and teardown_default_test automatically
struct TestGuard {
    _lock: std::sync::MutexGuard<'static, ()>,
}

impl TestGuard {
    fn new() -> Self {
        let _lock = TEST_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        setup_default_test();
        TestGuard { _lock }
    }
}

impl Drop for TestGuard {
    fn drop(&mut self) {
        teardown_default_test();
        // _lock is automatically dropped here, releasing the mutex
    }
}

// only going to be used in tests
pub fn run_default_test<T>(test_func: T)
where
    T: FnOnce(),
{
    // even if test_func panics, the guard will be dropped and teardown will be called
    let _guard = TestGuard::new();
    test_func();
}

// only going to be used in tests
pub fn run_default_test_with_special_config<T>(test_func: T, _config: roboprec::ir::precision::Precision)
where
    T: FnOnce(),
{
    // even if test_func panics, the guard will be dropped and teardown will be called
    let _guard = TestGuard::new();
    test_func();
}
