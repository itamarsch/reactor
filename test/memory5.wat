
(module
  ;; Import WASI functions to allow interaction with the environment.
  ;; This is necessary for I/O or exit code.
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

  ;; Declare a memory with an initial size of 1 page (64KiB).
  (memory 1)

  ;; Function to grow memory and return the previous size.
  (func $grow_memory (param $pages_to_grow i32) (result i32)
    ;; Grow the memory by the given number of pages.
    (memory.grow (local.get $pages_to_grow))
  )

  ;; Function to store a value at a memory address
  (func $store_value (param $address i32) (param $value i32)
    ;; Store the value at the given memory address
    (i32.store (local.get $address) (local.get $value))
  )

  ;; Function to write data to the newly allocated pages after memory grows.
  (func $write_to_new_memory
    (local $new_memory_start i32)
    (local $previous_size i32)

    ;; Grow memory by 2 pages and get the previous size
    (local.set $previous_size
      (call $grow_memory (i32.const 2)))

    ;; Calculate the starting address of the newly allocated memory.
    ;; Each page is 64KiB, so the new memory starts at previous_size * 65536 (64KiB).
    (local.set $new_memory_start
      (i32.mul (local.get $previous_size) (i32.const 65536)))

    ;; Store the value 42 at the start of the newly allocated memory.
    (call $store_value (local.get $new_memory_start) (i32.const 42))

    ;; Store the value 100 at the next 4-byte offset (just an example).
    (call $store_value (i32.add (local.get $new_memory_start) (i32.const 4)) (i32.const 100))
  )

  ;; Entry point (_start), called by the runtime.
  (func $start
    ;; Write to the newly allocated memory.
    (call $write_to_new_memory)

    ;; Exit the program with code 0 (success).
    (call $proc_exit (i32.const 0))
  )

  ;; Export the start function for WASI to call.
  (export "_start" (func $start))
)
