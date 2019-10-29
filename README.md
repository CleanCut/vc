# vc

`vc` (short for `version control`) is a rewrite of Git in Rust.  This is not useful, complete, 
stable, fancy or anything that you should ever consider actually using. I'm just learning Git
internals by implementing them myself.

## Goals

- Write the Git parts themselves in plain Rust using the standard library
- Lean on external crates as much as possible for non-core-Git functionality

## Non-goals

- Making this work for anyone other than myself

## Starting references

- [Write yourself a Git](https://wyag.thb.lt/)
- [Git pack format](https://github.com/git/git/blob/master/Documentation/technical/pack-format.txt)
