(module
  ;; Import the WASI 'proc_exit' function to set the exit code
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (param i32)))

  ;; Add a memory and export it (required by WASI)
  (memory (export "memory") 1)

  ;; Function with no return value and no arguments
  (func $func_no_return_no_args
    (local $x f32)
    f32.const 3.14
    local.set $x
    ;; Perform some computation
    local.get $x
    f32.const 2.0
    f32.mul
    drop
  )

  ;; Function with a single return value and no arguments
  (func $func_single_return_no_args (result f64)
    (local $y f64)
    f64.const 2.71828
    local.set $y
    local.get $y
    f64.const 1.61803
    f64.add
  )

  ;; Function with multiple return values and no arguments
  (func $func_multi_return_no_args (result i64 f32)
    i64.const 42
    f32.const 1.41421
  )

  ;; Function with no return value but with arguments
  (func $func_no_return_with_args (param $a f32) (param $b f32)
    (local $z f32)
    local.get $a
    local.get $b
    f32.sub
    local.set $z
  )

  ;; Function with a single return value and arguments
  (func $func_single_return_with_args (param $a i64) (param $b i64) (result i64)
    local.get $a
    local.get $b
    i64.mul
  )

  ;; Function with multiple return values and arguments
  (func $func_multi_return_with_args (param $a f64) (param $b f64) (result f64 f64)
    local.get $a
    local.get $b
    f64.div
    local.get $a
    local.get $b
    f64.sub
  )

  ;; Main function to compute the final result
  (func $main (export "_start")
    (local $result i32)
    (local $temp_f64_1 f64)
    (local $temp_f64_2 f64)
    (local $temp_i64 i64)
    (local $temp_f32 f32)
    (local $a i32)
    (local $b i32)
    ;; Call functions without arguments
    call $func_no_return_no_args

    call $func_single_return_no_args
    local.set $temp_f64_1

    call $func_multi_return_no_args
    local.set $temp_f32      ;; Second value (f32)
    local.set $temp_i64      ;; First value (i64)

    ;; Call functions with arguments
    f32.const 9.81
    f32.const 3.14
    call $func_no_return_with_args

    i64.const 7
    i64.const 6
    call $func_single_return_with_args
    local.set $temp_i64

    f64.const 8.0
    f64.const 2.0
    call $func_multi_return_with_args
    local.set $temp_f64_2    ;; Second value (f64)
    local.set $temp_f64_1    ;; First value (f64)

    ;; Compute the final result
    ;; Convert values to i32 and sum them
    local.get $temp_i64
    i32.wrap_i64
    local.get $temp_f64_1
    i32.trunc_f64_s
    i32.add
    local.get $temp_f64_2
    i32.trunc_f64_s
    i32.add
    local.get $temp_f32
    i32.trunc_f32_s
    i32.add
    local.set $result

    ;; Exit with the final result as the exit code
    local.get $result
    call $__wasi_proc_exit
  )
)
