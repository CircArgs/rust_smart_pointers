use std::{marker::PhantomData, ops::Deref};

pub struct RcInner<T> {
    value: T,
    refcount: usize,
}

pub struct Rc<T> {
    inner: *mut RcInner<T>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner { value, refcount: 1 });
        Rc {
            inner: Box::into_raw(inner),
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &mut *self.inner };
        inner.refcount += 1;
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        let temp = unsafe { &*self.inner };
        &temp.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { &mut *self.inner };
        if inner.refcount < 2 {
            {
                inner
            };
            let _ = unsafe { Box::from_raw(self.inner) };
        } else {
            inner.refcount -= 1;
        }
    }
}
