
(module
  ;; Import the WASI "wasi_snapshot_preview1" (necessary for WASI modules)
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Define the start function
  (func (export "_start")
    ;; Declare a local to store the block's result
    (local $result i32)
    (local.set $result (i32.const 30))

    (block $loop_guard 
    (loop $loop
      (local.get $result)

      (br_if $loop_guard  (i32.eqz))
      
      (local.get $result)
      (i32.const 1)
      (i32.sub)
      (local.set $result)

      (br $loop)
 
      
      )
    )
  
    local.get $result
    call $proc_exit
  )
)
