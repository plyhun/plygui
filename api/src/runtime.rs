use crate::inner::application::Application;
use std::cell::RefCell;

static mut READY: bool = false;

thread_local! {
    pub static APPLICATION: RefCell<usize> = RefCell::new(0);
}

pub fn get<T: Application>() -> Option<&'static mut T> {
    let ready = unsafe { READY };
    if ready {
        APPLICATION.with(|a| {
            let a = *a.borrow() as *mut T;
            if a.is_null() {
                panic!("Trying to access Application from a non-UI thread")
            } else {
                Some(unsafe { &mut *a })
            }
        })
    } else {
        None
    }
}

pub fn init<T: Application>(app: &mut T) {
    if unsafe { READY } {
        panic!("Already initialized!") 
    } else {
        // TODO here may come the race!
        APPLICATION.with(|a| {
            *a.borrow_mut() = app as *mut _ as usize;
        });
        unsafe {
            READY = true;
        }
    }
}
pub fn deinit() {
    if unsafe { READY } {
        // TODO here may come the race!
        APPLICATION.with(|a| {
            *a.borrow_mut() = 0;
        });
        unsafe {
            READY = false;
        }
    }
}
