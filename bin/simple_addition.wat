
(module
  ;; Import the WASI functions needed for output
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

  ;; Memory for WASI
  (memory 1)
  (export "memory" (memory 0))

  ;; Exported function to add two numbers
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add)

  ;; Export the add function
  (export "add" (func $add))

  ;; Entry point for the WASI application
  (func $main
    ;; Call the add function with 3 and 5
    i32.const 3
    i32.const 5
    call $add
    drop ;; Drop the result since we're not doing anything with it in this example

    ;; Exit the program with code 0
    i32.const 0
    call $proc_exit
  )

  ;; Export the main function
  (export "_start" (func $main))
)
