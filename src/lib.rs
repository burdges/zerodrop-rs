// Copyright 2016 Jeffrey Burdges

//! Zeroing drop wrapper types.

#![feature(box_syntax)]

extern crate consistenttime;

use std::boxed::Box;
use std::ops::{Deref,DerefMut};
use std::convert::{AsRef,AsMut};
// Conflicts with `impl Borrow<T> for T`
// use std::borrow::{Borrow,BorrowMut};

/// Zeroing drop wrapper type.
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
pub struct ZeroDrop<T>(Box<T>) where T: Copy+Default;

/// Zero a `ZeroDrop<T>` when dropped.
impl<T> Drop for ZeroDrop<T> where T: Copy+Default {
    #[inline(never)]
    fn drop(&mut self) {
        *self.0 = Default::default();
    }
}

/// Create a `ZeroDrop<T>` for a `T: Copy` consisting of a `Box<T>`
/// that will be zeroed when dropped.  `Box` is essential because LLVM
/// moves data on the stack willy nilly.
impl<T> ZeroDrop<T> where T: Copy+Default {
    pub fn new_default() -> ZeroDrop<T> {
        ZeroDrop(Box::new(Default::default()))
    }
    pub fn new_clone(t: &T) -> ZeroDrop<T> {
        ZeroDrop(box *t)
    }
    pub fn new(t: T) -> ZeroDrop<T> {
        ZeroDrop(box t)
    }
        // Avoid box syntax by using the following instead.
        // let b = Self::new_default();
        // *b = *t;
        // ZeroDrop(b)
}

impl<T> Default for ZeroDrop<T> where T: Copy+Default {
    fn default() -> ZeroDrop<T> {
        ZeroDrop(Box::new(Default::default()))
    }
}


#[derive(Debug)]
pub struct ZeroDropDrop<T>(Box<T>) where T: Drop+Default;

/// Zero a `ZeroDrop<T>` when dropped.
impl<T> Drop for ZeroDropDrop<T> where T: Drop+Default {
    #[inline(never)]
    fn drop(&mut self) {
        unsafe { ::std::ptr::drop_in_place(self) } 
        *self.0 = Default::default();
    }
}


/// Create a `ZeroDropDrop<T>` for a `T: Drop` that invokes 
/// `<T as Drop>::drop` before zeroing `T`.  We observe that
/// `<T as Drop>::drop` will be invoked a second time on a
/// `<T as Default>::default()` for each drop, so this must
/// be safe and desirable.
///
/// Warning: `ZeroDropDrop<T>` cannot deeply zero 
impl<T> ZeroDropDrop<T> where T: Drop+Default {
    pub fn new_default() -> ZeroDropDrop<T> {
        ZeroDropDrop(Default::default())
    }
    // TODO How can we promote a &T to a Box<T>?
    // pub fn new(t: &T) -> ZeroDropDrop<T> {
    //     ZeroDropDrop(box *t)
    // }
}


macro_rules! impl_ZeroDrop { ($s:ident,$cd:ident) => {

/// `Clone` the underlying `Box`
impl<T> Clone for $s<T> where T: $cd+Default {
    fn clone(&self) -> $s<T> {
        $s(self.0.clone())
    }
    fn clone_from(&mut self, source: &$s<T>) {
        self.0.clone_from(&source.0);
    }
}

/// Delegate `Deref` to `Box`
impl<T> Deref for $s<T> where T: $cd+Default {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.deref()
    }
}

/// Delegate `DerefMut` to `Box`
impl<T> DerefMut for $s<T> where T: $cd+Default {
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}

/// Delegate `AsRef<_>` to `Box`
impl<T,U> AsRef<U> for $s<T> where T: $cd+Default+AsRef<U> {
    fn as_ref(&self) -> &U {
        self.0.as_ref().as_ref()
    }
}

/// Delegate `AsMut<_>` to `Box`
impl<T,U> AsMut<U> for $s<T> where T: $cd+Default+AsMut<U> {
    fn as_mut(&mut self) -> &mut U {
        self.0.as_mut().as_mut()
    }
}

/*
/// Delegate `Borrow<_>` to `Box`
impl<T,U> Borrow<U> for $s<T> where T: $cd+Default+Borrow<U> {
    fn borrow(&self) -> &U {
        self.0.borrow()
    }
}

/// Delegate `BorrowMut<_>` to `Box`
impl<T,U> BorrowMut<U> for $s<T> where T: $cd+Default+BorrowMut<U> {
    fn borrow_mut(&mut self) -> &mut U {
        self.0.borrow_mut()
    }
}
*/

} }  // impl_Boxy


impl_ZeroDrop!(ZeroDrop,Copy);
// impl_ZeroDrop!(ZeroDropDrop,Drop);


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
        let s = ZeroDrop::new([3u8; 32]);  
        p = s.deref();
        ::std::mem::drop(s);
        unsafe { assert_eq!(*p,[0u8; 32]); }
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn not_droped() {
        let p : *const [u8; 32];
        let s = ZeroDrop::new([3u8; 32]);  
        p = s.deref();
        // ::std::mem::drop(s);
        unsafe { assert_eq!(*p,[0u8; 32]); }
    }
}


