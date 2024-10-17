(module
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory 1) ;; Define 1 page of memory (64KiB)

  ;; Export the memory so that we can inspect it if needed
  (export "memory" (memory 0))

  ;; Function to store a 32-bit value to an unaligned address
  (func (export "_start")
    (i32.const 1)    ;; Push the unaligned address (1 is not divisible by 4)
    (i32.const 42)   ;; Push the value to store
    (i32.store offset=0 align=4) ;; Perform an aligned store (expecting 4-byte alignment)

    (i32.const 1)
    (i32.load)

    call $proc_exit    ;; Call proc_exit with the result as the return code
  )

)
