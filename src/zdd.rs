// Copyright 2016 Jeffrey Burdges

// //! Zeroing drop wrapper types.

use std::boxed::Box;
use std::ops::{Deref,DerefMut};
use std::convert::{AsRef,AsMut};
use std::borrow::{Borrow,BorrowMut};

/// Zeroing drop wrapper type for `Drop` types.
///
/// `ZeroDropDrop<T>` wraps a `Box<T>` where `T: Drop+Default`.
/// It's drop method invokes `<T as Drop>::drop` before replacing
/// `T` with `<T as Default>::default()`, which gets dropped again.
///
/// We warn `ZeroDropDrop<T>` cannot deeply zero data, so do not
/// use it with objects like `Vec<T>`, `HashMap<T>`, etc.
/// For those, you'll need a custom allocator or a whole custom
/// data structure.
#[derive(Debug)]
pub struct ZeroDropDrop<T>(Box<T>) where T: Drop+Default;

/// Zero a `ZeroDrop<T>` when dropped.
impl<T> Drop for ZeroDropDrop<T> where T: Drop+Default {
    #[inline(never)]
    fn drop(&mut self) {
        let s: &mut T = self.0.deref_mut();
        unsafe {
            ::std::ptr::drop_in_place::<T>(s);
            ::std::ptr::write::<T>(s,Default::default());
        }
    }
}

/// Create a `ZeroDropDrop<T>` for a `T: Drop` that invokes 
/// `<T as Drop>::drop` before zeroing `T`.  We observe that
/// `<T as Drop>::drop` will be invoked a second time on a
/// `<T as Default>::default()` for each drop, so this must
/// be safe and desirable.  `ZeroDropDrop<T>` does not 
/// deeply zero data, so do not use it with data type that
/// employ allocation, like `Vec<T>`, `HashMap<T>`, etc.
impl<T> ZeroDropDrop<T> where T: Drop+Default {
    pub fn new_default() -> ZeroDropDrop<T> {
        ZeroDropDrop(Default::default())
    }

    pub unsafe fn new_uninitialized() -> ZeroDropDrop<T> {
        ZeroDropDrop(Box::new(::std::mem::uninitialized::<T>()))
    }

    pub unsafe fn zero_out(&mut self) {
        let s: &mut T = self.0.deref_mut();
        ::std::intrinsics::volatile_set_memory::<T>(s,0,1)
    }

    // Is b.clone_from(t) safe when b is uninitialized?
    /*
    pub fn new_clone(t: &T) -> ZeroDropDrop<T> where T: Clone {
        let mut b = Box::new(unsafe { ::std::mem::uninitialized::<T>() });
        b.clone_from(t);
        ZeroDropDrop(b)
    }
    */
}

/// `Clone` the underlying `Box`
impl<T> Clone for ZeroDropDrop<T> where T: Drop+Default+Clone {
    fn clone(&self) -> ZeroDropDrop<T> {
        ZeroDropDrop(self.0.clone())
    }
    fn clone_from(&mut self, source: &ZeroDropDrop<T>) {
        self.0.clone_from(&source.0);
    }
}

/// Delegate `Deref` to `Box`
impl<T> Deref for ZeroDropDrop<T> where T: Drop+Default {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.deref()
    }
}

/// Delegate `DerefMut` to `Box`
impl<T> DerefMut for ZeroDropDrop<T> where T: Drop+Default {
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}

/// Delegate `AsRef<_>` to `Box`
impl<T,U> AsRef<U> for ZeroDropDrop<T> where T: Drop+Default+AsRef<U> {
    fn as_ref(&self) -> &U {
        self.0.as_ref().as_ref()
    }
}

/// Delegate `AsMut<_>` to `Box`
impl<T,U> AsMut<U> for ZeroDropDrop<T> where T: Drop+Default+AsMut<U> {
    fn as_mut(&mut self) -> &mut U {
        self.0.as_mut().as_mut()
    }
}

/// Delegate `Borrow<_>` to `Box`
impl<T> Borrow<T> for ZeroDropDrop<T> where T: Drop+Default {
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

/// Delegate `BorrowMut<_>` to `Box`
impl<T> BorrowMut<T> for ZeroDropDrop<T> where T: Drop+Default {
    fn borrow_mut(&mut self) -> &mut T {
        self.0.borrow_mut()
    }
}


/*
trait ConstantTimeEq {
    fn constant_time_eq(a: &Self, b: &Self) -> bool;
}

impl<T> ConstantTimeEq for [T] where T: ConstantTimeEq {
    fn constant_time_eq(x: &Self, y: &Self) -> bool {
        ;
    }
}

/// We implement `PartialEq` to be a constant time comparison, so that
/// noone may define it otherwise.  
impl<T> PartialEq for ZeroDrop<T> where T: ConstantTimeEq {
    fn eq(&self, other: &ZeroDrop<T>) -> bool {
        ::consistenttime::ct_u8_slice_eq(&self.0, &other.0)
    }
}
impl<T> Eq for Secret<T>  where T: Copy { }
*/


/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroing_drops() {
        let p : *const [u8; 32];
        let s = ZeroDrop::new_insecure([3u8; 32]);  
        p = s.deref();
        ::std::mem::drop(s);
        unsafe { assert_eq!(*p,[0u8; 32]); }
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn not_droped() {
        let p : *const [u8; 32];
        let s = ZeroDrop::new_insecure([3u8; 32]);  
        p = s.deref();
        // ::std::mem::drop(s);
        unsafe { assert_eq!(*p,[0u8; 32]); }
    }
}
*/


