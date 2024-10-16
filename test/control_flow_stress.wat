(module
  ;; Import WASI's proc_exit function to exit with a code
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Define the main function
  (func $_start
    ;; Declare local variables
    (local $i i32)
    (local $sum i32)
    (local $j i64)
    (local $factorial_result i64)
    (local $selector i32)
    (local $exit_code i32)

    ;; Initialize variables
    (local.set $i (i32.const 0))
    (local.set $sum (i32.const 0))
    (local.set $j (i64.const 5))    ;; For factorial calculation
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
            (local.set $i (i32.const 0))
            (br $after_br_table)
          )
          ;; Selector == 1
          (local.set $i (i32.const 10))
          (br $after_br_table)
        )
        ;; Selector == 2
        (local.set $i (i32.const 20))
        (br $after_br_table)
      )
      ;; Default case
      (local.set $i (i32.const 30))
    )
    ;; Continue after branching

    ;; Loop to calculate sum from $i to $i + 10
    (block $sum_block
      (loop $sum_loop
        (local.get $i)
        (i32.const 30)
        (i32.ge_s)
        (br_if $sum_block)
        ;; sum += i
        (local.set $sum
          (i32.add (local.get $sum) (local.get $i))
        )
        ;; i += 1
        (local.set $i
          (i32.add (local.get $i) (i32.const 1))
        )
        (br $sum_loop)
      )
    )

    ;; Compute factorial of 5 using blocks with result
    (local.set $factorial_result
      (call $factorial (local.get $j))
    )

    ;; Use block with result to compute exit code
    (local.set $exit_code
      (block (result i32)
        (i32.add
          (i32.wrap_i64 (local.get $factorial_result))
          (local.get $sum)
        )
      )
    )

    ;; Exit with the computed exit code
    (local.get $exit_code)
    (i32.const 126)
    (i32.rem_s)
    (call $__wasi_proc_exit )
  )

  ;; Recursive function to compute factorial using if, blocks, and return
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

  ;; Export the main function
  (export "_start" (func $_start))
)
