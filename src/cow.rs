// Copyright 2016 Jeffrey Burdges

// //! Zeroing drop wrapper types.

use std::boxed::Box;
use std::ops::{Deref,DerefMut};
use std::convert::AsRef;
use std::borrow::Borrow;

use super::*;

/// Zeroing drop copy-on-write type for `Copy` types.
///
/// Assuming `T: Copy`, a `ZeroDrop<T>` wraps a `Box<T>`
/// and zeros it when dropped.  Unlike `Cow`, we must use 
/// `Box` because LLVM moves data wildly around the stack.
pub enum ZeroDropCow<'a, T: Copy + 'a> {
    /// Borrowed data.
    Borrowed(&'a T),

    /// Boxed data.
    Boxed(Box<T>),
}

/// Zero a `ZeroDrop<T>` when dropped.
impl<'a, T> Drop for ZeroDropCow<'a, T> where T: 'a+Copy {
    #[inline(never)]
    fn drop(&mut self) {
        match *self {
            ZeroDropCow::Borrowed(_) => { },
            ZeroDropCow::Boxed(ref mut b) => {
                let s: &mut T = b.deref_mut();
                unsafe { ::std::intrinsics::volatile_set_memory::<T>(s,0,1) }
            }
        }
    }
}

impl<'a, T> ZeroDropCow<'a, T> where T: 'a+Copy {
    /// Insecure as `t` likely gets placed on the stack
    pub fn new_insecure(t: T) -> ZeroDropCow<'a,T> {
        ZeroDropCow::Boxed(Box::new(t))
    }

    /// Secure but unsafe
    pub unsafe fn new_uninitialized() -> ZeroDropCow<'a,T> {
        ZeroDropCow::Boxed(Box::new(::std::mem::uninitialized::<T>()))
    }

    /// Create a `ZeroDrop<T>` for a `T: Copy` that borrows its argument.
    pub fn new(t: &'a T) -> ZeroDropCow<'a,T> {
        ZeroDropCow::Borrowed(t)
    }

    /// Create a `ZeroDrop<T>` for a `T: Copy` consisting of a `Box<T>`
    /// that will be zeroed when dropped.  Use `ZeroDrop` instead normally.
    pub fn new_copy(t: &T) -> ZeroDropCow<'a,T> {
        let mut b = Box::new(unsafe { ::std::mem::uninitialized::<T>() });
        unsafe { ::std::ptr::copy_nonoverlapping::<T>(t,b.deref_mut(),1) }
        ZeroDropCow::Boxed(b)
    }

    /// Convert a `ZeroDrowCow` into a `ZeroDrop`, copying if still borrowed.
    pub fn into_boxed(mut self) -> ZeroDrop<T> {
        match self {
            ZeroDropCow::Borrowed(b) => ZeroDrop::new_copy(b),
            ZeroDropCow::Boxed(ref mut o) => {
                let mut b = Box::new(unsafe { ::std::mem::uninitialized::<T>() });
                ::std::mem::swap::<T>(o,&mut b);
                ZeroDrop::new_box(b)
            }
        }
    }
}

impl<'a, T> Default for ZeroDropCow<'a, T> where T: 'a+Copy+Default {
    /// Creates a boxed `ZeroDropCow<'a, T>` with the default value for
    /// the contained owned value.
    fn default() -> ZeroDropCow<'a, T> {
        ZeroDropCow::Boxed(Default::default())
    }
}

/// Reborrow a `Borrowed` or `Clone` a `Boxed`
impl<'a, T> Clone for ZeroDropCow<'a, T> where T: 'a+Copy {
    fn clone(&self) -> ZeroDropCow<'a, T> {
        use self::ZeroDropCow::*;
        match *self {
            Borrowed(b) => Borrowed(b),
            Boxed(ref o) => ZeroDropCow::new_copy(o.deref()),
        }
    }
}

// Ain't clear this meshes well with `Cow`-like types
/// Delegate `Deref` to `Borrowed` or `Boxed`.
impl<'a, T> Deref for ZeroDropCow<'a, T> where T: 'a+Copy {
    type Target = T;

    fn deref(&self) -> &T {
        use self::ZeroDropCow::*;
        match *self {
            Borrowed(b) => b,
            Boxed(ref o) => o.deref(),  // Why does `Cow` use `o.borrow()` here?
        }
    }
}

/// Delegate `AsRef<_>` to `Borrowed` or `Boxed`.
// Why does `Cow` not provide `AsRef`?  Ain't clear how well `AsRef` works
// with `Cow`-like types or if it should call `.as_ref()` twice on `Boxed`.
impl<'a, T,U> AsRef<U> for ZeroDropCow<'a, T> where T: 'a+Copy+AsRef<U> {
    fn as_ref(&self) -> &U {
        use self::ZeroDropCow::*;
        match *self {
            Borrowed(b) => b.as_ref(),
            Boxed(ref o) => o.as_ref().as_ref(), 
        }
    }
}

/// Delegate `Borrow<T>` to `Borrowed` or `Boxed`.
impl<'a, T> Borrow<T> for ZeroDropCow<'a, T> where T: 'a+Copy {
    fn borrow(&self) -> &T {
        use self::ZeroDropCow::*;
        match *self {
            Borrowed(b) => b,
            Boxed(ref o) => o.borrow(), 
        }
    }
}
// I donno if any more `Borrow<_>`s make sense here.

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
        let s = ZeroDropCow::new_insecure([3u8; 32]);  
        p = s.deref();
        ::std::mem::drop(s);
        unsafe { assert_eq!(*p,[0u8; 32]); }
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn not_droped() {
        let p : *const [u8; 32];
        let s = ZeroDropCow::new_insecure([3u8; 32]);  
        p = s.deref();
        // ::std::mem::drop(s);
        unsafe { assert_eq!(*p,[0u8; 32]); }
    }
}
*/

