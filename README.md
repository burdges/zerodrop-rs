

## Nolonger maintained.  Used [ClearOnDrop](https://github.com/cesarb/clear_on_drop/) instead.




A thin wrapper for `Box` that zeros its data when dropped

[![build status](https://api.travis-ci.org/burdges/zerodrop-rs.png)](https://travis-ci.org/burdges/zerodrop-rs)
[![documenation](https://docs.rs/zerodrop/badge.svg)](https://docs.rs/zerodrop/)
[![crates.io link](https://img.shields.io/crates/v/zerodrop.svg)](https://crates.io/crates/zerodrop)


### Documentation

There are many types of data that should be erased when nolonger needed, with cryptographic key material being an extreme example.  This crate provides simple wrapper types that zero their contents when dropped.  See the [documentation](https://docs.rs/zerodrop/).

We cannot recommend this crate for all cryptographic applications because it lacks support for `mlock`.  There is no way to support `mlock` with less than a full fledged allocator because if several `mlock` calls lock the same page then the first `munlock` call will unlock that page completely.

There are two crates [secrets](https://github.com/stouset/secrets) and [tars](https://github.com/seb-m/tars/) that provides such an allocator, which you should use if you want real protection.  These crates predate the recently [added](https://github.com/rust-lang/rfcs/pull/1398) allocator [traits](https://github.com/rust-lang/rust/issues/32838) however, so things remain in flux for now.

We believe this crate provides an API similar enough to an allocator wrapping `mlock` that code developed using it and later ported to a full fledged allocator.  In particular, we operate only upon `Box`ed data and provide no methods that return data to the stack where [it could not be erased reliably](https://github.com/rust-lang/rfcs/issues/1850).



### Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/zerodrop).  Add it to your `Cargo.toml` with:

```toml
[dependencies]
zerodrop = "^0.1"
```

Use the crate like:

```rust
extern crate zerodrop;

...
```
