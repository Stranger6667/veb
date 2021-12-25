# vEB

A WIP Rust implementation of a van Emde Boas tree.

```rust
fn main() {
    let mut tree = veb::VebTree::new(1024);
    tree.insert(50);
    assert!(tree.contains(50));
    assert!(!tree.contains(49));
}
```

**NOTE**: This is an alpha version and is not ready for production use.

TODO:
- [x] Insert
- [x] Contains
- [ ] Delete one
- [ ] Delete all
- [ ] Store values associated with keys
- [x] Find next
- [ ] Find prev
- [ ] Size
- [ ] Iteration + exact size iteration + fused, double ended iterator
- [ ] Extend from iterator
- [ ] Drain / drain filter / retain / take
- [ ] Default / Debug
- [ ] First / last (with pop)
- [ ] Append
- [ ] Fuzzing
- [ ] Generic over T
- [ ] Clone
- [ ] Eq
- [ ] Some `From` impls (e.g. `&[usize]`)
- [ ] O(n) space
- [ ] Use bit vectors when size is <= word size
- [ ] More tests
- [ ] Detailed comments & docs
- [ ] Arena-based allocation
