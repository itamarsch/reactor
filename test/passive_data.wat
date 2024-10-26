
(module
  ;; Import the fd_write function from WASI
  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write (param i32 i32 i32 i32) (result i32)))

  ;; Declare a memory with an initial size of 1 page (64KiB)
  (memory $mem 1)
  (export "memory" (memory 0))

  ;; Active data segment at offset 0 containing "hello world"
  (data (i32.const 0) "hello world\n")

  ;; Passive data segment containing "hello you"
  (data $hello_you "hello you\n")

  ;; Define the main function
  (func $main (export "_start")
    ;; --- Setup for printing "hello world" ---
    ;; Store the address of the string ("hello world") in memory at offset 100
    i32.const 100        ;; Destination address for the iovec.buffer
    i32.const 0          ;; Address of "hello world" in memory
    i32.store            ;; Store buffer address at offset 100

    ;; Store the length of "hello world" (11 bytes) at offset 104
    i32.const 104        ;; Destination address for the iovec.length
    i32.const 12         ;; Length of "hello world"
    i32.store            ;; Store buffer length at offset 104

    ;; Prepare arguments for fd_write to print "hello world"
    i32.const 1          ;; File descriptor (1 for stdout)
    i32.const 100        ;; Pointer to the iovec structure
    i32.const 1          ;; Number of iovec structures (1)
    i32.const 200        ;; Address to store the number of bytes written
    call $fd_write       ;; Call fd_write
    drop                 ;; Ignore the fd_write result (errno)

    ;; --- Modify memory with the passive data segment ---
    ;; Copy "hello you" from the passive data segment into memory at offset 0
    i32.const 0          ;; Destination offset in memory
    i32.const 0          ;; Source offset in data segment
    i32.const 10          ;; Number of bytes to copy ("hello you" is 9 bytes)
    memory.init $hello_you   ;; Initialize memory with the passive data segment
    data.drop $hello_you     ;; Drop the passive data segment

    ;; Update the length in the iovec structure to 9 (length of "hello you")
    i32.const 104        ;; Address of the iovec.length
    i32.const 10          ;; New length of the string
    i32.store            ;; Update the length in the iovec structure

    i32.const 1          ;; File descriptor (1 for stdout)
    i32.const 100        ;; Pointer to the iovec structure
    i32.const 1          ;; Number of iovec structures (1)
    i32.const 200        ;; Address to store the number of bytes written
    call $fd_write       ;; Call fd_write
    drop                 ;; Ignore the fd_write result (errno)
  )

)
