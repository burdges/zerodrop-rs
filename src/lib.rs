// Copyright 2016 Jeffrey Burdges and David Stainton

//! Zeroing drop

extern crate consistenttime;


/// Zeroing 
#[derive(Debug)]
pub struct SemiSecret<T>(Box(T)) where T: Copy+Default;

impl<T> Drop for SemiSecret<T> where T: Copy+Default {
    #[inline(never)]
    fn drop(&mut self) {
        *self.0 = Default::default();
    }
}

#[feature(box_syntax)]
impl<T> SemiSecret<T> where T: Copy+Default {
    pub fn new(t: &T) -> SemiSecret<T> {
        box *t
    }
}

// Avoid box syntax by uncommenting the following.
/*
impl<T> SemiSecret<T> where T: Copy+Default {
    pub fn new(t: &T) -> SemiSecret<T> {
        let b = Box::new(Default::default());
        *b = *t;
        SemiSecret(b)
    }
}
*/


macro_rules! impl_Boxy { ($s:ident) => {

/// `Clone` the underlying `Box`
impl<T: Clone> Clone for $s<T> {
    fn clone(&self) -> $s<T> {
        $s(self.0.clone())
    }
    fn clone_from(&mut self, source: &$s<T>) {
        self.0.clone_from(&source.0);
    }
}

/// Delegate `Deref` to `Box`
impl<T> Deref for $s<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.deref()
    }
}

/// Delegate `DerefMut` to `Box`
impl<T> DerefMut for $s<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}

/// Delegate `Borrow<_>` to `Box`
impl<T,U> Borrow<U> for $s<T> {
    fn borrow(&self) -> &U {
        self.0.borrow()
    }
}

/// Delegate `BorrowMut<_>` to `Box`
impl<T,U> BorrowMut<U> for $s<T> {
    fn borrow_mut(&mut self) -> &mut U {
        self.0.borrow_mut()
    }
}

/// Delegate `AsRef<_>` to `Box`
impl<T,U> AsRef<U> for $s<T> {
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}

/// Delegate `AsMut<_>` to `Box`
impl<T,U> AsMut<U> for $s<T> {
    fn as_mut(&mut self) -> &mut U {
        self.0.as_mut()
    }
}

} }  // impl_Boxy


impl_Boxy!(SemiSecret);
// impl_Boxy!(DropSecret);


/// We implement `PartialEq` to be a constant time comparison, so that
/// noone may define it otherwise.  
impl<T> PartialEq for SemiSecret<T> where T: Copy {
    fn eq(&self, other: &SemiSecret<T>) -> bool {
        ::consistenttime::ct_u8_slice_eq(&self.0, &other.0)
    }
}
impl<T> Eq for Secret<T>  where T: Copy+Default { }



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
