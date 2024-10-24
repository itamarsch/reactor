(module
  ;; Import the WASI 'proc_exit' function to exit with a code.
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

  (memory 1)
  (export "memory" (memory 0))
  ;; Define the function type for functions that return an i32.
  (type $funcType (func (result i32)))

  ;; Declare a table of 3 function references.
  (table $table0 3 funcref)  ;; 'anyfunc' is used in MVP instead of 'funcref'.

  ;; Define functions that will be stored in the table.
  (func $func1 (result i32)
    ;; Return 10
    i32.const 10
  )

  (func $func2 (result i32)
    ;; Return 20
    i32.const 20
  )

  (func $func3 (result i32)
    ;; Return 30
    i32.const 30
  )

  ;; Declare an element segment to make functions available for 'ref.func'.
  ;; This element segment initializes the table starting at index 0.
  ;; We'll overwrite these entries using 'table.set' later.
(elem declare funcref (ref.func $func1) (ref.func $func2) (ref.func $func3))

  ;; The '_start' function required by WASI.
  (func $_start
    (local $sum i32)  ;; Declare a local variable to hold the sum.

    ;; Overwrite the table entries using 'table.set'.
    ;; Set table[0] = $func3
    i32.const 0          ;; Index 0
    ref.func $func3      ;; Reference to $func3
    table.set $table0    ;; Set table[0][0] = $func3

    ;; Set table[1] = $func1
    i32.const 1
    ref.func $func1
    table.set $table0    ;; Set table[0][1] = $func1

    ;; Set table[2] = $func2
    i32.const 2
    ref.func $func2
    table.set $table0    ;; Set table[0][2] = $func2

    ;; Initialize sum to 0.
    i32.const 0
    local.set $sum

    ;; Call function at table[0] and add its result to sum.
    i32.const 0          ;; Table index 0
    call_indirect (type $funcType)
    local.get $sum
    i32.add
    local.set $sum

    ;; Call function at table[1] and add its result to sum.
    i32.const 1          ;; Table index 1
    call_indirect (type $funcType)
    local.get $sum
    i32.add
    local.set $sum

    ;; Call function at table[2] and add its result to sum.
    i32.const 2          ;; Table index 2
    call_indirect (type $funcType)
    local.get $sum
    i32.add
    local.set $sum

    ;; Exit with the sum as the exit code.
    local.get $sum
    call $proc_exit
  )

  ;; Specify the start function.
  (export "_start" (func $_start))
)
