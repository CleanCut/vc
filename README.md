# vc

`vc` (short for `version control`) is a rewrite of Git in Rust.

# Status: Pre-alpha
`vc` is not yet stable or complete.  In fact, it's not even _useful_ quite yet. You shouldn't consider using it in this state unless you are interested in participating in the development of it.

## Goals

- Learn Git internals by implementing them myself in Rust.
- Write the Git parts themselves in plain Rust using the standard library.
- Lean on external crates as much as possible for non-core-Git functionality.
- _Start_ by mirroring the internal naming and location of logic from C Git where it makes sense.

## Non-goals

- Making this work for anyone other than myself
- 100% bug-for-bug compatibility

## Roadmap

- [x] Command-line argument handling skeleton
- [ ] `init` (`builtin/init-db.c`) _in progress_
- [ ] `config`
  - [ ] Then go back and finish parts of `init` that interact with `config`
- [ ] Testing framework (`t/`)
  - Script that downloads `t/` from `git/git`, and patches it to work
  - `t/0001-init.sh` can run and some of the tests pass
- [ ] `hash-object`
- [ ] `cat-file`
- [ ] `log`
- [ ] `ls-tree`

## Notes

Block comments (using `/* */`) are copied-and-pasted from C Git. For example:

```
/*
 * GIT_WORK_TREE makes sense only in conjunction with GIT_DIR
 * without --bare.  Catch the error early.
 */
```

Any other style comment is from me. For example:
```
// todo: Do the thing...

/// This Rust struct represents the...

//! This module is for...
```


## Useful Development References

- [Git](https://github.com/git/git)  -- which I'll refer to mostly as `C Git`
- [Write yourself a Git](https://wyag.thb.lt/)
- [Git pack format](https://github.com/git/git/blob/master/Documentation/technical/pack-format.txt)
