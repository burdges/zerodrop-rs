
Zero boxed data when dropped

[![build status](https://api.travis-ci.org/burdges/zerodrop-rs.png)](https://travis-ci.org/burdges/zerodrop-rs)
[![documenation](https://docs.rs/zerodrop/badge.svg)](https://docs.rs/zerodrop/)
[![crates.io link](https://img.shields.io/crates/v/zerodrop.svg)](https://crates.io/crates/zerodrop)


### Documentation

There are many types of data that should be erased when nolonger needed, with cryptographic key material being an extreme example.  This crate provides simple wrapper types that zero their contents when dropped.  See the [documentation](https://docs.rs/zerodrop/).

We cannot recommend this crate for cryptographic applications because it lacks support for `mlock`.  There is no way to support `mlock` with less than a full fledged allocator because if several `mlock` calls lock the same page then the first `munlock` call will unlock the page completely.

There is a crate [tars](https://github.com/seb-m/tars/) that provides such an allocator, but it predates the recently [added](https://github.com/rust-lang/rfcs/pull/1398) allocator [traits](https://github.com/rust-lang/rust/issues/32838).

We believe this crate provides an API similar to what ...

https://github.com/rust-lang/rfcs/issues/1850


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
