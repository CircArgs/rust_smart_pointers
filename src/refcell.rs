use crate::cell::Cell;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;
#[derive(Copy, Clone)]
enum ReferenceType {
    Shared(usize),
    Unshared,
    Mutable,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<ReferenceType>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(ReferenceType::Unshared),
        }
    }
    pub fn borrow(&self) -> Option<Ref<T>> {
        match self.state.get() {
            ReferenceType::Unshared => {
                self.state.set(ReferenceType::Shared(1));
                Some(Ref { refcell: self })
            }
            ReferenceType::Shared(n) => {
                self.state.set(ReferenceType::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            _ => None,
        }
    }
    pub fn borrow_mut(&self) -> Option<RefMut<T>> {
        match self.state.get() {
            ReferenceType::Unshared => {
                self.state.set(ReferenceType::Mutable);
                Some(RefMut { refcell: self })
            }
            _ => None,
        }
    }
}

pub struct Ref<'a, T> {
    refcell: &'a RefCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            ReferenceType::Unshared | ReferenceType::Mutable => unreachable![],
            ReferenceType::Shared(n) => self.refcell.state.set(ReferenceType::Shared(n - 1)),
        }
    }
}

pub struct RefMut<'a, T> {
    refcell: &'a RefCell<T>,
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            ReferenceType::Unshared | ReferenceType::Shared(_) => unreachable![],
            ReferenceType::Mutable => self.refcell.state.set(ReferenceType::Unshared),
        }
    }
}
