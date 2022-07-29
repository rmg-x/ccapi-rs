use thiserror::Error;

// https://www.psdevwiki.com/ps3/Error_Codes#Generic_errors
#[derive(Error, Debug)]
pub enum ConsoleError {
    #[error("The resource is temporarily unavailable")]
    EAGAIN,
    #[error("Invalid argument or flag")]
    EINVAL,
    #[error("The feature is not yet implemented")]
    ENOSYS,
    #[error("Memory allocation failed")]
    ENOMEM,
    #[error("The resource with the specified identifier does not exist")]
    ESRCH,
    #[error("The file does not exist")]
    ENOENT,
    #[error("The file is in unrecognized format")]
    ENOEXEC,
    #[error("Resource deadlock is avoided")]
    EDEADLK,
    #[error("Operation not permitted")]
    EPERM,
    #[error("The device or resource is busy")]
    EBUSY,
    #[error("The operation is timed out")]
    ETIMEDOUT,
    #[error("The operation is aborted")]
    EABORT,
    #[error("Invalid memory access")]
    EFAULT,
    #[error("Try to access a non existing child process")]
    ECHILD,
    #[error("State of the target thread is invalid")]
    ESTAT,
    #[error("Alignment is invalid")]
    EALIGN,
    #[error("Shortage of the kernel resources")]
    EKRESOURCE,
    #[error("The file is a directory")]
    EISDIR,
    #[error("Operation cancelled")]
    ECANCELED,
    #[error("Entry already exists")]
    EEXIST,
    #[error("Port is already connected")]
    EISCONN,
    #[error("Port is not connected")]
    ENOTCONN,
    #[error("Failure in authorizing SELF")]
    EAUTHFAIL,
    #[error("The file is not MSELF")]
    ENOTMSELF,
    #[error("System version error")]
    ESYSVER,
    #[error("Fatal system error occurred while authorizing SELF SELF auth failure")]
    EAUTHFATAL,
    #[error("Math domain violation")]
    EDOM,
    #[error("Math range violation")]
    ERANGE,
    #[error("Illegal multi-byte sequence in input")]
    EILSEQ,
    #[error("File position error")]
    EFPOS,
    #[error("Syscall was interrupted")]
    EINTR,
    #[error("File too large")]
    EFBIG,
    #[error("Too many links")]
    EMLINK,
    #[error("File table overflow")]
    ENFILE,
    #[error("No space left on device")]
    ENOSPC,
    #[error("Not a TTY")]
    ENOTTY,
    #[error("Broken pipe")]
    EPIPE,
    #[error("Read-only filesystem")]
    EROFS,
    #[error("Illegal seek")]
    ESPIPE,
    #[error("Arg list too long")]
    E2BIG,
    #[error("Access violation")]
    EACCES,
    #[error("Invalid file descriptor")]
    EBADF,
    #[error("Filesystem mounting failed")]
    EIO,
    #[error("Too many files open")]
    EMFILE,
    #[error("No device")]
    ENODEV,
    #[error("Not a directory")]
    ENOTDIR,
    #[error("No such device or IO")]
    ENXIO,
    #[error("Cross-device link error")]
    EXDEV,
    #[error("Bad Message")]
    EBADMSG,
    #[error("In progress")]
    EINPROGRESS,
    #[error("Message size error")]
    EMSGSIZE,
    #[error("Name too long")]
    ENAMETOOLONG,
    #[error("No lock")]
    ENOLCK,
    #[error("Not empty")]
    ENOTEMPTY,
    #[error("Not supported")]
    EUNSUP,
    #[error("File-system specific error")]
    EFSSPECIFIC,
    #[error("Overflow occured")]
    EOVERFLOW,
    #[error("Filesystem not mounted")]
    ENOTMOUNTED,
    #[error("Not SData")]
    ENOTSDATA,
    #[error("Incorrect version in sys_load_param")]
    ESDKVER,
    #[error("Pointer is null When related to PARAMSFO")]
    ENOLICDISC,
    #[error("Pointer is null When related to DISCSFO (and PARAMSFO)")]
    ENOLICENT,
    #[error("Unknown error")]
    Unknown,
}

impl From<u32> for ConsoleError {
    fn from(arg: u32) -> Self {
        match arg {
            0x80010001 => ConsoleError::EAGAIN,
            0x80010002 => ConsoleError::EINVAL,
            0x80010003 => ConsoleError::ENOSYS,
            0x80010004 => ConsoleError::ENOMEM,
            0x80010005 => ConsoleError::ESRCH,
            0x80010006 => ConsoleError::ENOENT,
            0x80010007 => ConsoleError::ENOEXEC,
            0x80010008 => ConsoleError::EDEADLK,
            0x80010009 => ConsoleError::EPERM,
            0x8001000A => ConsoleError::EBUSY,
            0x8001000B => ConsoleError::ETIMEDOUT,
            0x8001000C => ConsoleError::EABORT,
            0x8001000D => ConsoleError::EFAULT,
            0x8001000E => ConsoleError::ECHILD,
            0x8001000F => ConsoleError::ESTAT,
            0x80010010 => ConsoleError::EALIGN,
            0x80010011 => ConsoleError::EKRESOURCE,
            0x80010012 => ConsoleError::EISDIR,
            0x80010013 => ConsoleError::ECANCELED,
            0x80010014 => ConsoleError::EEXIST,
            0x80010015 => ConsoleError::EISCONN,
            0x80010016 => ConsoleError::ENOTCONN,
            0x80010017 => ConsoleError::EAUTHFAIL,
            0x80010018 => ConsoleError::ENOTMSELF,
            0x80010019 => ConsoleError::ESYSVER,
            0x8001001A => ConsoleError::EAUTHFATAL,
            0x8001001B => ConsoleError::EDOM,
            0x8001001C => ConsoleError::ERANGE,
            0x8001001D => ConsoleError::EILSEQ,
            0x8001001E => ConsoleError::EFPOS,
            0x8001001F => ConsoleError::EINTR,
            0x80010020 => ConsoleError::EFBIG,
            0x80010021 => ConsoleError::EMLINK,
            0x80010022 => ConsoleError::ENFILE,
            0x80010023 => ConsoleError::ENOSPC,
            0x80010024 => ConsoleError::ENOTTY,
            0x80010025 => ConsoleError::EPIPE,
            0x80010026 => ConsoleError::EROFS,
            0x80010027 => ConsoleError::ESPIPE,
            0x80010028 => ConsoleError::E2BIG,
            0x80010029 => ConsoleError::EACCES,
            0x8001002A => ConsoleError::EBADF,
            0x8001002B => ConsoleError::EIO,
            0x8001002C => ConsoleError::EMFILE,
            0x8001002D => ConsoleError::ENODEV,
            0x8001002E => ConsoleError::ENOTDIR,
            0x8001002F => ConsoleError::ENXIO,
            0x80010030 => ConsoleError::EXDEV,
            0x80010031 => ConsoleError::EBADMSG,
            0x80010032 => ConsoleError::EINPROGRESS,
            0x80010033 => ConsoleError::EMSGSIZE,
            0x80010034 => ConsoleError::ENAMETOOLONG,
            0x80010035 => ConsoleError::ENOLCK,
            0x80010036 => ConsoleError::ENOTEMPTY,
            0x80010037 => ConsoleError::EUNSUP,
            0x80010038 => ConsoleError::EFSSPECIFIC,
            0x80010039 => ConsoleError::EOVERFLOW,
            0x8001003A => ConsoleError::ENOTMOUNTED,
            0x8001003B => ConsoleError::ENOTSDATA,
            0x8001003C => ConsoleError::ESDKVER,
            0x8001003D => ConsoleError::ENOLICDISC,
            0x8001003E => ConsoleError::ENOLICENT,
            _ => ConsoleError::Unknown,
        }
    }
}
