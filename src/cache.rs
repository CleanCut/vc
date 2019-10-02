pub enum SharedRepo {
    PERM_UMASK = 0,
    OLD_PERM_GROUP = 1,
    OLD_PERM_EVERYBODY = 2,
    PERM_GROUP = 0o0660,
    PERM_EVERYBODY = 0o0664,
}