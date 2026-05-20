use std::ffi::c_void;
use std::sync::{Arc, Mutex, Condvar, OnceLock};
use std::collections::HashMap;


// Opaque struct for semaphore
pub struct Semaphore {
    mutex: Mutex<u32>,
    cond: Condvar,
}

impl Semaphore {
    pub fn new(value: u32) -> Self {
        Semaphore {
            mutex: Mutex::new(value),
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut count = self.mutex.lock().unwrap();
        while *count == 0 {
            count = self.cond.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn signal(&self) {
        let mut count = self.mutex.lock().unwrap();
        *count += 1;
        self.cond.notify_one();
    }
}




static ONCE_MAP: OnceLock<Mutex<HashMap<usize, bool>>> = OnceLock::new();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread_once(
    lock: *mut c_void,
    init_routine: Option<unsafe extern "C" fn()>,
) -> i32 {
    if let Some(routine) = init_routine {
        let lock_addr = lock as usize;
        let map_mutex = ONCE_MAP.get_or_init(|| Mutex::new(HashMap::new()));
        
        let mut should_run = false;
        {
            let mut map = map_mutex.lock().unwrap();
            if !map.contains_key(&lock_addr) {
                map.insert(lock_addr, true);
                should_run = true;
            }
        }
        
        if should_run {
            routine();
        }
        0
    } else {
        -1
    }
}
