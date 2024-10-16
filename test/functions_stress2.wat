
(module
  ;; Import the WASI 'proc_exit' function to exit with a code
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Function that returns multiple values
  (func $return_multiple_values (result i32 i32 i64 f32)
    (i32.const 10)
    (i32.const 20)
    (i64.const 30)
    (f32.const 40.0)
  )

  ;; Function that takes multiple arguments and returns a value
  (func $compute_complex_sum (param $a i32) (param $b i32) (param $c i64) (param $d f32) (result i64)

    (i64.add
      (i64.extend_i32_s (local.get $a))
      (i64.add
        (i64.extend_i32_s (local.get $b))
        (i64.add
          (local.get $c)
          (i64.trunc_f32_s (local.get $d))
        )
      )
    )
  )

  ;; Function that uses locals of multiple types and performs operations
  (func $use_various_locals
    (local $i i32)
    (local $j i32)
    (local $k i64)
    (local $f f32)
    (local $d f64)
    ;; Initialize locals
    (local.set $i (i32.const 5))
    (local.set $j (i32.const 15))
    (local.set $k (i64.const 25))
    (local.set $f (f32.const 2.5))
    (local.set $d (f64.const 3.5))
    ;; Perform operations
    (local.set $i (i32.add (local.get $i) (i32.const 7)))
    (local.set $j (i32.mul (local.get $j) (i32.const 3)))
    (local.set $k (i64.sub (local.get $k) (i64.const 10)))
    (local.set $f (f32.mul (local.get $f) (f32.const 1.5)))
    (local.set $d (f64.div (local.get $d) (f64.const 0.5)))
  )

  ;; Function that calls other functions and processes multiple return values
  (func $process_values (result i32)
    ;; Call $return_multiple_values and capture results
    (local $x i32)
    (local $y i32)
    (local $z i64)
    (local $w f32)
    (call $return_multiple_values)
    (local.set $w) ;; $w = 40.0
    (local.set $z) ;; $z = 30
    (local.set $y) ;; $y = 20
    (local.set $x) ;; $x = 10
    ;; Call $compute_complex_sum with these values
    (call $compute_complex_sum
      (local.get $x)
      (local.get $y)
      (local.get $z)
      (local.get $w)
    )
    ;; Result is i64, wrap to i32
    (i32.wrap_i64)
  )

  ;; Function with no arguments and no return value
  (func $perform_no_action)

  ;; Additional function that manipulates locals
  (func $manipulate_locals
    (local $a i32)
    (local $b i64)
    (local $c f32)
    (local.set $a (i32.const 100))
    (local.set $b (i64.extend_i32_s (local.get $a)))
    (local.set $c (f32.convert_i32_s (local.get $a)))
    ;; Perform more operations
    (local.set $a (i32.rem_s (local.get $a) (i32.const 7)))
    (local.set $b (i64.shl (local.get $b) (i64.const 2)))
    (local.set $c (f32.sqrt (local.get $c)))
  )

  ;; Main function
  (func (export "_start")
    ;; Call functions
    (call $perform_no_action)
    (call $use_various_locals)
    (call $manipulate_locals)
    ;; Call $process_values and get result
    (call $process_values)
    ;; Exit with the result as the code
    (call $proc_exit)
  )
)
