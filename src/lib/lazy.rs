use std::mem;
use std::sync;

pub struct Lazy<T> {
    pub lock: sync::Once,
    pub ptr: *const T,
}

impl<T> Lazy<T> {
    pub fn get(&'static self, init: || -> T) -> &'static T {
        unsafe {
            self.lock.doit(|| {
                mem::transmute::<&Lazy<T>, &mut Lazy<T>>(self).ptr = mem::transmute(box init())
            });
            mem::transmute(self.ptr)
        }
    }
}

pub const ONCE_INIT: sync::Once = sync::ONCE_INIT;
