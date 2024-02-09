pub trait RawPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;

    fn as_ptr(&self) -> *const T;
}

impl<T> RawPtr<T> for Box<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut() as *mut T
    }

    fn as_ptr(&self) -> *const T {
        self.as_ref() as *const T
    }
}
