
(module
  ;; Import the WASI "wasi_snapshot_preview1" (necessary for WASI modules)
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Define the start function
  (func (export "_start")
    ;; Declare local variables
    (local $i32_result i32)
    (local $i64_result i64)
    (local $f32_result f32)
    (local $f64_result f64)
    (local $final_result i32)

    ;; Block 1: Compute an i32 result with nested blocks
    (block $block1 (result i32)
      (block $block1_inner (result i32)
        (block $block1_innermost (result i32)
          i32.const 7
          i32.const 5
          i32.mul          ;; 7 * 5 = 35
          i32.const 3
          i32.div_s        ;; 35 / 3 = 11 (integer division)
          i32.const 2
          i32.add          ;; 11 + 2 = 13
        )
        i32.const 4
        i32.mul            ;; 13 * 4 = 52
      )
      i32.const 6
      i32.sub              ;; 52 - 6 = 46
    )
    local.set $i32_result  ;; Store the i32 result

    ;; Block 2: Compute an i64 result with nested blocks
    (block $block2 (result i64)
      (block $block2_inner (result i64)
        (block $block2_innermost (result i64)
          i64.const 123456789
          i64.const 987654321
          i64.add          ;; 123456789 + 987654321 = 1111111110
          i64.const 2
          i64.mul          ;; 1111111110 * 2 = 2222222220
        )
        i64.const 100000000
        i64.div_s          ;; 2222222220 / 100000000 = 22
      )
      i64.const 5
      i64.rem_s            ;; 22 % 5 = 2
    )
    local.set $i64_result  ;; Store the i64 result

    ;; Block 3: Compute an f32 result with nested blocks
    (block $block3 (result f32)
      (block $block3_inner (result f32)
        (block $block3_innermost (result f32)
          f32.const 3.14
          f32.const 2.71
          f32.mul          ;; 3.14 * 2.71 ≈ 8.5094
          f32.const 1.41
          f32.add          ;; 8.5094 + 1.41 ≈ 9.9194
        )
        f32.const 2.0
        f32.div           ;; 9.9194 / 2.0 ≈ 4.9597
      )
      f32.const 3.0
      f32.sub             ;; 4.9597 - 3.0 ≈ 1.9597
    )
    local.set $f32_result  ;; Store the f32 result

    ;; Block 4: Compute an f64 result with nested blocks
    (block $block4 (result f64)
      (block $block4_inner (result f64)
        (block $block4_innermost (result f64)
          f64.const 6.02214076e23  ;; Avogadro's number
          f64.const 9.10938356e-31 ;; Electron mass
          f64.mul                  ;; Multiply: result ≈ 5.481799e-07
          f64.const 1.0e10
          f64.div                  ;; Divide: result ≈ 5.481799e-17
        )
        f64.const 2.0
        f64.add                   ;; Add: result ≈ 2.00000000000000005
      )
      f64.const 1.0
      f64.sub                     ;; Subtract: result ≈ 1.00000000000000005
    )
    local.set $f64_result  ;; Store the f64 result

    ;; Block 5: Combine results with type conversions and nested blocks
    (block $block5 (result i32)
      (block $block5_inner (result i32)
        (block $block5_innermost (result i32)
          local.get $i32_result        ;; Get i32 result (46)
          local.get $i64_result
          i32.wrap_i64                 ;; Convert i64 to i32 (2)
          i32.add                      ;; 46 + 2 = 48
        )
        (block $block5_deeper (result i32)
          local.get $f32_result        ;; Get f32 result (~1.9597)
          i32.trunc_f32_s              ;; Truncate to i32 (1)
          local.get $f64_result
          i32.trunc_f64_s              ;; Truncate to i32 (1)
          i32.add                      ;; 1 + 1 = 2
        )
        i32.add                        ;; 48 + 2 = 50
      )
      i32.const 7
      i32.mul                          ;; 50 * 7 = 350
      i32.const 100
      i32.rem_s                        ;; 350 % 100 = 50
    )
    local.set $final_result            ;; Store the final i32 result

    ;; Call proc_exit with $final_result
    local.get $final_result
    call $proc_exit
  )
)
