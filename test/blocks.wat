
(module
  ;; Import the WASI "wasi_snapshot_preview1" (necessary for WASI modules)
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Define the start function
  (func (export "_start")
    ;; Declare a local to store the block's result
    (local $result i32)

    ;; Start a block that will return the result of the addition
    (block (result i32) ;; Declare that the block will return an i32 value
      ;; Add two constants together inside the block
      i32.const 10  ;; Push the constant 10 onto the stack
      i32.const 20  ;; Push the constant 20 onto the stack
      i32.add       ;; Add the two constants (result: 30 on the stack)
    )

    ;; Store the result of the block in a local variable
    local.set $result

    ;; (Optionally) do something with $result, for now, we'll just call proc_exit
    ;; Exit the process, passing the result as the exit code
    local.get $result  ;; Load the result from the local variable
    call $proc_exit    ;; Call proc_exit with the result as the return code
  )
)
