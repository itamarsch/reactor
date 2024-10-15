
(module
  ;; Import the WASI "wasi_snapshot_preview1" (necessary for WASI modules)
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

  (memory (export "memory") 1)

  ;; Define the start function
  (func (export "_start")
    ;; Declare local variables
    (local $a i32)
    (local $b i32)
    (local $c i32)
    (local $d i32)
    (local $e i32)
    (local $result i32)

    ;; Block 1
    (block $block1
      ;; Compute $a = 5 + 3
      i32.const 5
      i32.const 3
      i32.add
      local.set $a

      ;; Block 2
      (block $block2
        ;; Compute $b = $a * 2
        local.get $a
        i32.const 2
        i32.mul
        local.set $b

        ;; Block 3
        (block $block3
          ;; Compute $c = $b - 4
          local.get $b
          i32.const 4
          i32.sub
          local.set $c

          ;; Block 4
          (block $block4
            ;; Compute $d = $c / 2
            local.get $c
            i32.const 2
            i32.div_s
            local.set $d

            ;; Block 5
            (block $block5
              ;; Compute $e = $d mod 3
              local.get $d
              i32.const 3
              i32.rem_s
              local.set $e

              ;; Compute final result $result = $e + 42
              local.get $e
              i32.const 42
              i32.add
              local.set $result
            )
          )
        )
      )
    )

    ;; Call proc_exit with $result
    local.get $result
    call $proc_exit
  )
)
