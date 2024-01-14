# Compute bounds for prefix string queries for BTreeSet/BTreeMap::range

[ [crates.io] ] [ [lib.rs] ] [ [docs.rs] ]

If you have a BTreeSet or BTreeMap with string keys and want to find all entries
with a given prefix, the standard library (as of Rust 1.75) doesn't offer any
built in methods to do so.

You could use something like the following:

```rust
let iterator = mymap.range(Bounds::Included("myprefix"),
                           Bounds::Excluded("myprefiy"));
```

This issue is finding the upper bound `myprefiy`. You have to deal with UTF-8
encoding, invalid code points etc. That is what the code in this library solves.

The code is taken from [a blog post by Jimmy Hartzell](https://www.thecodedmessage.com/posts/prefix-ranges/),
and slightly tweaked:

* To work in no-std (still needs alloc though)
* To work with BTreeMaps (not just BTreeSets).

A huge thanks to Jimmy Hartzell for solving the problem already (though they
never published it as a crate reusable by others).

See the [crate documentation][docs.rs] for usage examples.

## MSRV

[crates.io]: https://crates.io/crates/prefix_range
[docs.rs]: https://docs.rs/prefix_range
[lib.rs]: https://lib.rs/crates/prefix_range
