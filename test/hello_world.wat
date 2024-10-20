(module
  ;; Import the WASI functions for writing to stdout and exiting the program
  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "proc_exit"
    (func $proc_exit (param i32)))

  ;; Declare a memory space for our program
  (memory $memory 1)
  (export "memory" (memory $memory))

  ;; Define the string "Hello, World!\n" in memory starting at offset 0
  (data (i32.const 0) "Hello, World!\n")

  ;; Entry point of the program
  (func $_start (export "_start")
    ;; Local variables for error codes and bytes written
    (local $err i32)
    (local $nwritten i32)

    ;; Set up the iovec structure in memory to point to our string
    ;; iov_base (pointer to the string)
    (i32.store (i32.const 20) (i32.const 0))
    ;; iov_len (length of the string)
    (i32.store (i32.const 24) (i32.const 14))

    ;; Call fd_write to write the string to stdout (file descriptor 1)
    (local.set $err
      (call $fd_write
        (i32.const 1)      ;; file descriptor (stdout)
        (i32.const 20)     ;; iovecs pointer
        (i32.const 1)      ;; iovec count
        (i32.const 28)))   ;; where to store the number of bytes written

    ;; Check if fd_write returned an error
    (if (i32.ne (local.get $err) (i32.const 0))
      (then
        ;; Exit the program with the error code from fd_write
        (call $proc_exit (local.get $err))
      )
    )

    ;; Load the number of bytes written
    (local.set $nwritten (i32.load (i32.const 28)))

    ;; Check if the number of bytes written matches the expected length
    (if (i32.ne (local.get $nwritten) (i32.const 14))
      (then
        ;; Exit with an error code indicating partial write (e.g., 1)
        (call $proc_exit (i32.const 1))
      )
    )

    ;; Exit the program with a return code of 0 (success)
    (call $proc_exit (i32.const 0))
  )
)
