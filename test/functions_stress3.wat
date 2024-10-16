
(module
  ;; Import the WASI proc_exit function to set the exit code
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Function that returns multiple values
  (func $return_multiple_values (result i32 i32)
    (local $a i32)
    (local $b i32)

    (i32.const 42)
    (local.set $a )
    (local.set $b (i32.const 58))
    (local.get $a)
    (local.get $b)
  )

  ;; Function with arguments and a single return value
  (func $add_numbers (param $x i32) (param $y i32) (result i32)
    (i32.add (local.get $x) (local.get $y))
  )

  ;; Function with no arguments and no return value
  (func $do_nothing)

  ;; Function using locals of multiple types
  (func $use_multiple_locals (result i32)
    (local $i i32)
    (local $l i64)
    (local $f f32)
    (local $d f64)
    (local.set $i (i32.const 10))
    (local.set $l (i64.const 20))
    (local.set $f (f32.const 30.5))
    (local.set $d (f64.const 40.5))
    ;; Return the sum of $i and the wrapped $l
    (i32.add
      (local.get $i)
      (i32.wrap_i64 (local.get $l))
    )
  )

  ;; Main function
  (func $main (export "_start")
    ;; Call functions and compute final result
    (local $res i32)
    ;; Call function with multiple return values
    (local $val1 i32)
    (local $val2 i32)
    (call $return_multiple_values)
    (local.set $val1)
    (local.set $val2)
    ;; Add the two returned values
    (local.set $res
      (call $add_numbers (local.get $val1) (local.get $val2))
    )
    ;; Add result from use_multiple_locals function
    (local.set $res
      (call $add_numbers (local.get $res)
        (call $use_multiple_locals)
      )
    )
    ;; Call a function with no operation
    (call $do_nothing)
    ;; Use the result as the exit code
    (local.get $res)
    (i32.const 126)
    (i32.rem_s)
    (call $proc_exit )
  )
)
