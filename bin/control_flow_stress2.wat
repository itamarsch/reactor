(module
  ;; Import WASI's proc_exit function to exit with a code
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Define the main function
  (func $_start
    ;; Declare local variables
    (local $i i32)
    (local $j i32)
    (local $k i32)
    (local $sum i32)
    (local $product i32)
    (local $selector i32)
    (local $temp_f32 f32)
    (local $temp_f64 f64)
    (local $exit_code i32)
    (local $original_i i32)  ;; To store the original value of $i

    ;; Initialize variables
    (local.set $i (i32.const 0))
    (local.set $j (i32.const 0))
    (local.set $k (i32.const 0))
    (local.set $sum (i32.const 0))
    (local.set $product (i32.const 1))
    (local.set $selector (i32.const 2)) ;; Change this to test different branches

    ;; Use br_table for branching based on $selector
    (block $after_br_table
      (block $default_block
        (block $block2
          (block $block1
            (block $block0
              (local.get $selector)
              (br_table $block0 $block1 $block2 $default_block)
            )
            ;; Selector == 0
            (local.set $i (i32.const 2))
            (local.set $j (i32.const 3))
            (br $after_br_table)
          )
          ;; Selector == 1
          (local.set $i (i32.const 4))
          (local.set $j (i32.const 5))
          (br $after_br_table)
        )
        ;; Selector == 2
        (local.set $i (i32.const 6))
        (local.set $j (i32.const 7))
        (br $after_br_table)
      )
      ;; Default case
      (local.set $i (i32.const 8))
      (local.set $j (i32.const 9))
    )
    ;; Continue after branching

    ;; Store the original value of $i for use in the inner loop condition
    (local.set $original_i (local.get $i))

    ;; Nested loops to compute sum of products
    ;; Outer loop: k from 1 to 3
    (local.set $k (i32.const 1))
    (block $outer_loop_exit
      (loop $outer_loop
        ;; Inner loop: i from original_i to original_i + 2
        (local.set $i (local.get $original_i))
        (block $inner_loop_exit
          (loop $inner_loop
            ;; sum += i * j
            (local.set $sum
              (i32.add (local.get $sum)
                (i32.mul (local.get $i) (local.get $j))
              )
            )
            ;; i += 1
            (local.set $i
              (i32.add (local.get $i) (i32.const 1))
            )
            ;; if i >= original_i + 3, break inner loop
            (local.get $i)
            (local.get $original_i)
            (i32.const 3)
            (i32.add)
            (i32.ge_s)
            (br_if $inner_loop_exit)
            ;; Else continue inner loop
            (br $inner_loop)
          )
        )
        ;; k += 1
        (local.set $k
          (i32.add (local.get $k) (i32.const 1))
        )
        ;; if k > 3, break outer loop
        (local.get $k)
        (i32.const 3)
        (i32.gt_s)
        (br_if $outer_loop_exit)
        ;; Else continue outer loop
        (br $outer_loop)
      )
    )

    ;; Some floating-point calculations
    ;; Compute temp_f32 = sqrt(16.0 + 9.0) = sqrt(25.0) = 5.0
    (local.set $temp_f32
      (f32.sqrt
        (f32.add (f32.const 16.0) (f32.const 9.0))
      )
    )

    ;; Since f64.pow is not available, compute 2^10 manually using a loop
    ;; Initialize temp_f64 to 1.0
    (local.set $temp_f64 (f64.const 1.0))
    (local.set $i (i32.const 0)) ;; Reusing $i
    (block $pow_loop_exit
      (loop $pow_loop
        ;; temp_f64 *= 2.0
        (local.set $temp_f64
          (f64.mul (local.get $temp_f64) (f64.const 2.0))
        )
        ;; i += 1
        (local.set $i
          (i32.add (local.get $i) (i32.const 1))
        )
        ;; if i >= 10, break loop
        (local.get $i)
        (i32.const 10)
        (i32.ge_s)
        (br_if $pow_loop_exit)
        ;; Else continue loop
        (br $pow_loop)
      )
    )

    ;; Convert floating-point results to integers
    (local.set $i
      (i32.trunc_f32_s (local.get $temp_f32))
    )

    (local.set $j
      (i32.wrap_i64
        (i64.trunc_f64_s (local.get $temp_f64))
      )
    )

    ;; Use blocks with results to combine values
    (local.set $product
      (block (result i32)
        (i32.mul (local.get $i) (local.get $j))
      )
    )

    ;; Final computation for exit code
    (local.set $exit_code
      (block (result i32)
        (i32.add
          (local.get $sum)
          (local.get $product)
        )
      )
    )

    ;; Exit with the computed exit code
    (local.get $exit_code)
    (i32.const 126)
    (i32.rem_s)
    (call $__wasi_proc_exit )
  )

  ;; Export the main function
  (export "_start" (func $_start))
)
