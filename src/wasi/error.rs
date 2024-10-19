use std::io;

#[repr(u16)]
#[allow(dead_code)]
pub enum WasiError {
    Success,                         // No error occurred. System call completed successfully.
    ArgumentListTooLong,             // Argument list too long.
    PermissionDenied,                // Permission denied.
    AddressInUse,                    // Address in use.
    AddressNotAvailable,             // Address not available.
    AddressFamilyNotSupported,       // Address family not supported.
    WouldBlock,                      // Resource unavailable, or operation would block.
    AlreadyInProgress,               // Connection already in progress.
    BadFileDescriptor,               // Bad file descriptor.
    BadMessage,                      // Bad message.
    Busy,                            // Device or resource busy.
    Canceled,                        // Operation canceled.
    NoChildProcesses,                // No child processes.
    ConnectionAborted,               // Connection aborted.
    ConnectionRefused,               // Connection refused.
    ConnectionReset,                 // Connection reset.
    Deadlock,                        // Resource deadlock would occur.
    DestinationAddressRequired,      // Destination address required.
    DomainError,                     // Mathematics argument out of domain of function.
    FileExists,                      // File exists.
    BadAddress,                      // Bad address.
    FileTooLarge,                    // File too large.
    HostUnreachable,                 // Host is unreachable.
    IdentifierRemoved,               // Identifier removed.
    IllegalByteSequence,             // Illegal byte sequence.
    InProgress,                      // Operation in progress.
    Interrupted,                     // Interrupted function.
    InvalidArgument,                 // Invalid argument.
    IoError,                         // I/O error.
    SocketConnected,                 // Socket is connected.
    IsDirectory,                     // Is a directory.
    TooManySymbolicLinks,            // Too many levels of symbolic links.
    FileDescriptorTooLarge,          // File descriptor value too large.
    TooManyLinks,                    // Too many links.
    MessageTooLarge,                 // Message too large.
    FileNameTooLong,                 // Filename too long.
    NetworkDown,                     // Network is down.
    NetworkReset,                    // Connection aborted by network.
    NetworkUnreachable,              // Network unreachable.
    TooManyOpenFiles,                // Too many files open in system.
    NoBufferSpace,                   // No buffer space available.
    NoSuchDevice,                    // No such device.
    NoSuchFileOrDirectory,           // No such file or directory.
    ExecutableFormatError,           // Executable file format error.
    NoLocksAvailable,                // No locks available.
    NotEnoughSpace,                  // Not enough space.
    NoMessage,                       // No message of the desired type.
    ProtocolNotAvailable,            // Protocol not available.
    NoSpaceLeft,                     // No space left on device.
    FunctionNotSupported,            // Function not supported.
    SocketNotConnected,              // The socket is not connected.
    NotADirectory,                   // Not a directory.
    DirectoryNotEmpty,               // Directory not empty.
    StateNotRecoverable,             // State not recoverable.
    NotASocket,                      // Not a socket.
    OperationNotSupported,           // Operation not supported on socket.
    InappropriateIoControlOperation, // Inappropriate I/O control operation.
    NoSuchDeviceOrAddress,           // No such device or address.
    ValueTooLarge,                   // Value too large to be stored in data type.
    PreviousOwnerDied,               // Previous owner died.
    OperationNotPermitted,           // Operation not permitted.
    BrokenPipe,                      // Broken pipe.
    ProtocolError,                   // Protocol error.
    ProtocolNotSupported,            // Protocol not supported.
    WrongSocketType,                 // Protocol wrong type for socket.
    ResultTooLarge,                  // Result too large.
    ReadOnlyFileSystem,              // Read-only file system.
    InvalidSeek,                     // Invalid seek.
    NoSuchProcess,                   // No such process.
    ConnectionTimedOut,              // Connection timed out.
    TextFileBusy,                    // Text file busy.
    CrossDeviceLink,                 // Cross-device link.
    CapabilitiesInsufficient,        // Extension: Capabilities insufficient.
}

impl From<io::ErrorKind> for WasiError {
    fn from(error: io::ErrorKind) -> Self {
        match error {
            io::ErrorKind::NotFound => WasiError::NoSuchFileOrDirectory,
            io::ErrorKind::PermissionDenied => WasiError::PermissionDenied,
            io::ErrorKind::ConnectionRefused => WasiError::ConnectionRefused,
            io::ErrorKind::ConnectionReset => WasiError::ConnectionReset,
            io::ErrorKind::ConnectionAborted => WasiError::ConnectionAborted,
            io::ErrorKind::NotConnected => WasiError::SocketNotConnected,
            io::ErrorKind::AddrInUse => WasiError::AddressInUse,
            io::ErrorKind::AddrNotAvailable => WasiError::AddressNotAvailable,
            io::ErrorKind::BrokenPipe => WasiError::BrokenPipe,
            io::ErrorKind::AlreadyExists => WasiError::FileExists,
            io::ErrorKind::WouldBlock => WasiError::WouldBlock,
            io::ErrorKind::InvalidInput => WasiError::InvalidArgument,
            io::ErrorKind::InvalidData => WasiError::BadMessage,
            io::ErrorKind::TimedOut => WasiError::ConnectionTimedOut,
            io::ErrorKind::WriteZero => WasiError::IoError,
            io::ErrorKind::Interrupted => WasiError::Interrupted,
            io::ErrorKind::Other => WasiError::IoError,
            _ => WasiError::IoError,
        }
    }
}
