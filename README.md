# vc

`vc` (short for `version control`) is a rewrite of Git in Rust.

# Status: Pre-alpha
`vc` is not yet stable or complete.  In fact, it's not even _useful_ quite yet. You shouldn't consider using it in this state unless you are interested in participating in the development of it.

## Goals

- Learn Git internals by implementing them myself in Rust
- Write the Git parts themselves in plain Rust using the standard library
- Lean on external crates as much as possible for non-core-Git functionality
- Mirror the internal naming and location of logic from C Git where it makes sense

## Non-goals

- Making this work for anyone other than myself
- 100% bug-for-bug compatibility

## Roadmap

- [ ] `init` (in progress)
  - [x] MVP project skeleton
  - [ ] Config subsystem? (could stub this out and come back to it later...) 
- [ ] `hash-object`
- [ ] `cat-file`
- [ ] `log`

## Useful Development References

- [Write yourself a Git](https://wyag.thb.lt/)
- [Git pack format](https://github.com/git/git/blob/master/Documentation/technical/pack-format.txt)
