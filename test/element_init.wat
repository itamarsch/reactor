
(module
  ;; Import fd_write from WASI to write to stdout
  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write (param i32 i32 i32 i32) (result i32)))

  ;; Declare a memory with at least one page
  (memory $memory 1)
  (export "memory" (memory $memory))

  ;; Declare a table for indirect function calls
  (table $table 1 funcref)

  ;; Define function types
  (type $type0 (func))

  ;; Passive data segments for the strings
  (data $hello_data "Hello, World!\n")
  (data $bye_data "Goodbye, World!\n")

  ;; Globals for memory offsets where strings will be stored
  (global $hello_offset i32 (i32.const 1024))
  (global $bye_offset i32 (i32.const 1040))  ;; Adjusted offset for alignment

  ;; Function to write a string to stdout using fd_write
  (func $write_string (param $str_offset i32) (param $str_len i32)
    (local $iovec_offset i32)
    ;; Set iovec struct at memory offset 0
    (local.set $iovec_offset (i32.const 0))
    ;; Set iov_base to string offset
    (i32.store (local.get $iovec_offset) (local.get $str_offset))
    ;; Set iov_len to string length
    (i32.store (i32.add (local.get $iovec_offset) (i32.const 4)) (local.get $str_len))
    ;; Call fd_write: fd=1 (stdout), iovs=iovec_offset, iovs_len=1, nwritten at memory offset 8
    (call $fd_write
      (i32.const 1)
      (local.get $iovec_offset)
      (i32.const 1)
      (i32.const 8))
    ;; Ignore fd_write result and nwritten
    drop
  )

  ;; Hello World function
  (func $hello (type $type0)
    (call $write_string (global.get $hello_offset) (i32.const 14))
  )

  ;; Goodbye World function
  (func $bye (type $type0)
    (call $write_string (global.get $bye_offset) (i32.const 16))
  )

  ;; Active element segment initializing table index 0 with $hello
  (elem (i32.const 0) $hello)

  ;; Passive element segment containing $bye
  (elem $bye_elem funcref (ref.func $bye))

  ;; Start function to initialize memory with data segments
  (start $start)
  (func $start
    ;; Initialize memory with "Hello, World!\n"
    (memory.init $hello_data (global.get $hello_offset) (i32.const 0) (i32.const 14))
    ;; Initialize memory with "Goodbye, World!\n"
    (memory.init $bye_data (global.get $bye_offset) (i32.const 0) (i32.const 16))
    ;; Drop data segments after initialization
    (data.drop $hello_data)
    (data.drop $bye_data)
  )

  ;; Main function
  (func $main (export "_start")
    ;; Call the function at table index 0 (should print "Hello, World!")
    (call_indirect (type $type0) (i32.const 0))
    ;; Initialize table index 0 with $bye using the passive element
    (i32.const 0)
    (i32.const 0)
    (i32.const 1)
    (table.init 0 $bye_elem)
    ;; Drop the passive element segment
    (elem.drop $bye_elem)
    ;; Call the function at table index 0 again (should print "Goodbye, World!")
    (call_indirect (type $type0) (i32.const 0))
  )
)
