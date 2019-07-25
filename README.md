# vc

`vc` (short for `version control`) is a rewrite of Git in Rust.  This is not useful, complete, 
stable, fancy or anything that you should ever consider actually using. Fit is just an experiment
for fun mostly so I can learn Git internals by implementing them myself.

## Goals

- Write the Git parts themselves in plain Rust using the standard library
- If I ever get to non-Git parts (TLS, for example) lean on external crates as much as possible.
- Throw away or abandon this whole thing if it stops being useful to learn from and/or fun.

## Starting references

- [Write yourself a Git](https://wyag.thb.lt/)
- [Git pack format](https://github.com/git/git/blob/master/Documentation/technical/pack-format.txt)