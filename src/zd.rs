// Copyright 2016 Jeffrey Burdges

// //! Zeroing drop wrapper types.

use std::boxed::Box;
use std::ops::{Deref,DerefMut};
use std::convert::{AsRef,AsMut};
use std::borrow::{Borrow,BorrowMut};

/// Zeroing drop wrapper type for `Copy` type.
///
/// Assuming `T: Copy`, a `ZeroDrop<T>` wraps a `Box<T>`
/// and zeros it when dropped.  We must use `Box` because 
/// LLVM moves data on the stack willy nilly.
///
/// ```rust
/// let p : *const [u8; 32];
/// let s = zerodrop::ZeroDrop::new_clone(&[3u8; 32]);  
/// p = &*s;
/// std::mem::drop(s);
/// unsafe { assert_eq!(*p,[0u8; 32]); }
/// ```
///
/// We recommend abstracting usage of `ZeroDrop` as follows because
/// `ZeroDrop` does not `mlock` data.
/// ```rust,ignore
/// type Secret<T> = ZeroDrop<T> where T: Copy+Default;
/// ```
/// We similarly encurage wrapping `ZeroDrop` yourself so as to limit
/// where and how secret data can be used in your code, including avoiding
/// any trait magic that seems overly subtle.
/// ```rust,ignore
/// struct MySecret(pub ZeroDrop<[u8; 32]>);
/// ```
#[derive(Debug)]
pub struct ZeroDrop<T>(Box<T>) where T: Copy;

/// Zero a `ZeroDrop<T>` when dropped.
impl<T> Drop for ZeroDrop<T> where T: Copy {
    #[inline(never)]
    fn drop(&mut self) {
        let s: &mut T = self.0.deref_mut();
        unsafe { ::std::intrinsics::volatile_set_memory::<T>(s,0,1) }
        // We could do this if we had default
        // *self.0 = Default::default();
    }
}

/// Create a `ZeroDrop<T>` for a `T: Copy` consisting of a `Box<T>`
/// that will be zeroed when dropped. 
impl<T> ZeroDrop<T> where T: Copy {
    /// Insecure as `t` likely gets placed on the stack
    pub fn new_insecure(t: T) -> ZeroDrop<T> {
        ZeroDrop(Box::new(t))
    }

    /// Use provided `Box<T>`
    pub fn new_box(b: Box<T>) -> ZeroDrop<T> {
        ZeroDrop(b)
    }

    /// Secure but unsafe
    pub unsafe fn new_uninitialized() -> ZeroDrop<T> {
        ZeroDrop(Box::new(::std::mem::uninitialized::<T>()))
    }

    /// Allocate box and copy data into it from reference
    pub fn new_copy(t: &T) -> ZeroDrop<T> {
        let mut b = Box::new(unsafe { ::std::mem::uninitialized::<T>() });
        unsafe { ::std::ptr::copy_nonoverlapping::<T>(t,b.deref_mut(),1) }
        ZeroDrop(b)
    }

    pub unsafe fn zero_out(&mut self) {
        let s: &mut T = self.0.deref_mut();
        ::std::intrinsics::volatile_set_memory::<T>(s,0,1)
    }

    pub fn new_zeroed() -> ZeroDrop<T> {
        // let mut z = unsafe { Self::new_uninitialized() };
        // unsafe { z.zero_out() }
        let mut b = Box::new(unsafe { ::std::mem::uninitialized::<T>() });
        unsafe { ::std::intrinsics::volatile_set_memory::<T>(b.deref_mut(),0,1) }
        ZeroDrop(b)
    }
}

impl<T> Default for ZeroDrop<T> where T: Copy+Default {
    fn default() -> ZeroDrop<T> {
        ZeroDrop(Default::default())
    }
}

/// `Clone` the underlying `Box`
impl<T> Clone for ZeroDrop<T> where T: Copy {
    fn clone(&self) -> ZeroDrop<T> {
        ZeroDrop(self.0.clone())
    }
    fn clone_from(&mut self, source: &ZeroDrop<T>) {
        self.0.clone_from(&source.0);
    }
}

/// Delegate `Deref` to `Box`
impl<T> Deref for ZeroDrop<T> where T: Copy {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.deref()
    }
}

/// Delegate `DerefMut` to `Box`
impl<T> DerefMut for ZeroDrop<T> where T: Copy {
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}

/// Delegate `AsRef<_>` to `Box`
impl<T,U> AsRef<U> for ZeroDrop<T> where T: Copy+AsRef<U> {
    fn as_ref(&self) -> &U {
        self.0.as_ref().as_ref()
    }
}

/// Delegate `AsMut<_>` to `Box`
impl<T,U> AsMut<U> for ZeroDrop<T> where T: Copy+AsMut<U> {
    fn as_mut(&mut self) -> &mut U {
        self.0.as_mut().as_mut()
    }
}

/// Delegate `Borrow<_>` to `Box`
impl<T> Borrow<T> for ZeroDrop<T> where T: Copy {
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}
// I donno if any more `Borrow<_>` make sense here.

/// Delegate `BorrowMut<_>` to `Box`
impl<T> BorrowMut<T> for ZeroDrop<T> where T: Copy {
    fn borrow_mut(&mut self) -> &mut T {
        self.0.borrow_mut()
    }
}
// I donno if any more `BorrowMut<_>` make sense here.



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


