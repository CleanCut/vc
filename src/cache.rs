//! Compare to `cache.h`

use crate::usage::die;

pub const GIT_DIR_ENVIRONMENT: &str = "GIT_DIR";
pub const GIT_COMMON_DIR_ENVIRONMENT: &str = "GIT_COMMON_DIR";
pub const GIT_NAMESPACE_ENVIRONMENT: &str = "GIT_NAMESPACE";
pub const GIT_WORK_TREE_ENVIRONMENT: &str = "GIT_WORK_TREE";
pub const GIT_PREFIX_ENVIRONMENT: &str = "GIT_PREFIX";
pub const GIT_SUPER_PREFIX_ENVIRONMENT: &str = "GIT_INTERNAL_SUPER_PREFIX";
pub const DEFAULT_GIT_DIR_ENVIRONMENT: &str = ".git";
pub const DB_ENVIRONMENT: &str = "GIT_OBJECT_DIRECTORY";
pub const INDEX_ENVIRONMENT: &str = "GIT_INDEX_FILE";
pub const GRAFT_ENVIRONMENT: &str = "GIT_GRAFT_FILE";
pub const GIT_SHALLOW_FILE_ENVIRONMENT: &str = "GIT_SHALLOW_FILE";
pub const TEMPLATE_DIR_ENVIRONMENT: &str = "GIT_TEMPLATE_DIR";
pub const CONFIG_ENVIRONMENT: &str = "GIT_CONFIG";
pub const CONFIG_DATA_ENVIRONMENT: &str = "GIT_CONFIG_PARAMETERS";
pub const EXEC_PATH_ENVIRONMENT: &str = "GIT_EXEC_PATH";
pub const CEILING_DIRECTORIES_ENVIRONMENT: &str = "GIT_CEILING_DIRECTORIES";
pub const NO_REPLACE_OBJECTS_ENVIRONMENT: &str = "GIT_NO_REPLACE_OBJECTS";
pub const GIT_REPLACE_REF_BASE_ENVIRONMENT: &str = "GIT_REPLACE_REF_BASE";
pub const GITATTRIBUTES_FILE: &str = ".gitattributes";
pub const INFOATTRIBUTES_FILE: &str = "info/attributes";
pub const ATTRIBUTE_MACRO_PREFIX: &str = "[attr]";
pub const GITMODULES_FILE: &str = ".gitmodules";
pub const GITMODULES_INDEX: &str = ":.gitmodules";
pub const GITMODULES_HEAD: &str = "HEAD:.gitmodules";
pub const GIT_NOTES_REF_ENVIRONMENT: &str = "GIT_NOTES_REF";
pub const GIT_NOTES_DEFAULT_REF: &str = "refs/notes/commits";
pub const GIT_NOTES_DISPLAY_REF_ENVIRONMENT: &str = "GIT_NOTES_DISPLAY_REF";
pub const GIT_NOTES_REWRITE_REF_ENVIRONMENT: &str = "GIT_NOTES_REWRITE_REF";
pub const GIT_NOTES_REWRITE_MODE_ENVIRONMENT: &str = "GIT_NOTES_REWRITE_MODE";
pub const GIT_LITERAL_PATHSPECS_ENVIRONMENT: &str = "GIT_LITERAL_PATHSPECS";
pub const GIT_GLOB_PATHSPECS_ENVIRONMENT: &str = "GIT_GLOB_PATHSPECS";
pub const GIT_NOGLOB_PATHSPECS_ENVIRONMENT: &str = "GIT_NOGLOB_PATHSPECS";
pub const GIT_ICASE_PATHSPECS_ENVIRONMENT: &str = "GIT_ICASE_PATHSPECS";
pub const GIT_QUARANTINE_ENVIRONMENT: &str = "GIT_QUARANTINE_PATH";
pub const GIT_OPTIONAL_LOCKS_ENVIRONMENT: &str = "GIT_OPTIONAL_LOCKS";
pub const GIT_TEXT_DOMAIN_DIR_ENVIRONMENT: &str = "GIT_TEXTDOMAINDIR";

/// Sort of like cache.h's "enum sharedrepo"
#[derive(Copy, Clone)]
pub enum SharedRepo {
    PermUmask,
    PermGroup,
    PermEverybody,
    PermMask(u32),
}

#[allow(clippy::if_same_then_else)] // remove this attribute once the "todo (config values)" is done.
impl SharedRepo {
    /// Constructor based on string from the cli. Option::None means no cli override, so get it from
    /// the config.
    pub fn from(val: Option<&str>) -> Self {
        /*
         * Treat values 0, 1 and 2 as compatibility cases.
         */
        match val {
            None => {
                // todo (config values): get the value from the config file(s)
                if false {
                    // `value` in C is the pointer to store the retrieved value into.
                    // if (!git_config_get_value("core.sharedrepository", value)
                    //     the_shared_repository = git_config_perm("core.sharedrepository", value)
                    SharedRepo::PermGroup // delete this line when the above is implemented ^
                                          // oh, and delete the #[allow(clippy::if_same_then_else)] line too.
                } else {
                    SharedRepo::PermGroup
                }
            }
            Some("umask") | Some("false") | Some("no") | Some("off") | Some("0") => {
                SharedRepo::PermUmask
            }
            Some("group") | Some("true") | Some("yes") | Some("on") | Some("1") => {
                SharedRepo::PermGroup
            }
            Some("all") | Some("world") | Some("everybody") | Some("2") => {
                SharedRepo::PermEverybody
            }
            Some(s) => {
                // Parse octal numbers
                if let Ok(octal) = s.parse::<u32>() {
                    // A filemode value was given: 0xxx
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
                } else {
                    SharedRepo::PermGroup
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
