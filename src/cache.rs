//! Compare to `cache.h`

use crate::usage::die;

/// Sort of like `cache.h :: enum sharedrepo`
#[derive(Copy, Clone)]
pub enum SharedRepo {
    PermUmask,
    PermGroup,
    PermEverybody,
    PermMask(u32),
}

impl SharedRepo {
    /// Constructor based on string from the cli. Option::None means no cli override, so get it from
    /// the config.
    pub fn from(val: Option<&str>) -> Self {
        /*
         * Treat values 0, 1 and 2 as compatibility cases.
         */
        match val {
            None => {
                // todo: get the value from the config file(s)
                if false {
                    // `value` in C is the pointer to store the retrieved value into.
                    // if (!git_config_get_value("core.sharedrepository", value)
                    //     the_shared_repository = git_config_perm("core.sharedrepository", value)
                    SharedRepo::PermGroup // delete this line when the above is implemented ^
                } else {
                    SharedRepo::PermGroup
                }
            }
            Some("umask") | Some("false") | Some("no") | Some("off") | Some("0") => SharedRepo::PermUmask,
            Some("group") | Some("true") | Some("yes") | Some("on") | Some("1") => SharedRepo::PermGroup,
            Some("all") | Some("world") | Some("everybody") | Some("2") => {
                SharedRepo::PermEverybody
            }
            Some(s) => {
                // Parse octal numbers
                let maybe_octal = s.parse::<u32>();
                if maybe_octal.is_err() {
                    SharedRepo::PermGroup
                } else {
                    // A filemode value was given: 0xxx
                    let octal = maybe_octal.unwrap();
                    if (octal & 0o600) != 0o600 {
                        die(format!(
                            "problem with core.sharedRepository filemode value {:3o}.\n\
                             The owner of files must always have read and write permissions.",
                            octal
                        ));
                    }
                    // setup.c's git_config_perm returns this as a negative value. From what I can
                    // tell, the reasoning was: An arbitrary value passed in by the user could match
                    // one of the (0|1|2) compatibility values and be interpreted vastly differently
                    // since apparently folks were *determined* to stuff the user's mask bits into
                    // the same integer already used by the old compatibility enum values AND the
                    // new built-in mask values. Tsk, tsk. They have to go through an amazing amount
                    // of gymnastics to tease back out masks from the three different sorts of
                    // values stored in the same int. We've got the full power of a Rust enum, so
                    // we'll stick to sane values, thank you very much.
                    SharedRepo::PermMask(octal & 0o666)
                }
            }
        }
    }
    /// Convert Self into the numerical permissions
    pub fn as_permissions(self) -> u32 {
        match self {
            SharedRepo::PermUmask => 0,
            SharedRepo::PermGroup => 0o660,
            SharedRepo::PermEverybody => 0o664,
            SharedRepo::PermMask(i) => i,
        }
    }
}
