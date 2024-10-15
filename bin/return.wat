
(module
  ;; Import the WASI 'proc_exit' function to set the exit code
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (param i32)))

  ;; Add a memory and export it (required by WASI)
  (memory (export "memory") 1)

  (func $factorial (param $n i64) (result i64)
    ;; If n == 0, return 1 immediately
    (if (i64.eqz (local.get $n))
      (then
        (return (i64.const 1))
      )
    )
    ;; Else, continue with the recursion
    (i64.mul
      (local.get $n)
      (call $factorial (i64.sub (local.get $n) (i64.const 1)))
    )
  )

  (func $main (export "_start")
    (i64.const 5)
    call $factorial
    i32.wrap_i64
    call $__wasi_proc_exit
  )
)
