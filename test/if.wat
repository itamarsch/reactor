

(module
  ;; Import the WASI "wasi_snapshot_preview1" (necessary for WASI modules)
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Define the start function
  (func (export "_start")
    (i32.const 0)
    (if (result i32)  (then (i32.const 4)) (else (i32.const 5)))
    call $proc_exit
  )
)
