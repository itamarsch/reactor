
(module
  ;; Import proc_exit
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

  ;; Declare memory
  (memory $mem 1)
  (export "memory" (memory 0))
  ;; Start function
  (func $_start
    (local $sum i32)
    (local $i32_val i32)
    (local $i64_val i64)
    (local $f32_val f32)
    (local $f64_val f64)

    ;; Initialize sum to 0
    i32.const 0
    local.set $sum

    ;; Initialize memory with some data via stores
    ;; Write values to memory using store instructions
    ;; Use store instructions that we need to test

    ;; I32Store: store 123 at memory[0]
    i32.const 0     ;; address
    i32.const 123   ;; value
    i32.store       ;; store i32 at memory[0]

    ;; I64Store: store 456 at memory[4]
    i32.const 4
    i64.const 456
    i64.store

    ;; F32Store: store 789.0 at memory[12]
    i32.const 12
    f32.const 789.0
    f32.store

    ;; F64Store: store 101112.0 at memory[16]
    i32.const 16
    f64.const 101112.0
    f64.store

    ;; I32Store8: store 255 at memory[24]
    i32.const 24
    i32.const 255
    i32.store8

    ;; I32Store16: store 65535 at memory[25]
    i32.const 25
    i32.const 65535
    i32.store16

    ;; I64Store8: store 127 at memory[27]
    i32.const 27
    i64.const 127
    i64.store8

    ;; I64Store16: store 32767 at memory[28]
    i32.const 28
    i64.const 32767
    i64.store16

    ;; I64Store32: store 2147483647 at memory[30]
    i32.const 30
    i64.const 2147483647
    i64.store32

    ;; Now, perform loads to read back the stored values and include them in the sum

    ;; I32Load from memory[0]
    i32.const 0
    i32.load
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I64Load from memory[4]
    i32.const 4
    i64.load
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; F32Load from memory[12]
    i32.const 12
    f32.load
    local.set $f32_val

    ;; sum += i32.trunc_s/f32($f32_val)
    local.get $sum
    local.get $f32_val
    i32.trunc_f32_s
    i32.add
    local.set $sum

    ;; F64Load from memory[16]
    i32.const 16
    f64.load
    local.set $f64_val

    ;; sum += i32.trunc_s/f64($f64_val)
    local.get $sum
    local.get $f64_val
    i32.trunc_f64_s
    i32.add
    local.set $sum

    ;; I32Load8S from memory[24]
    i32.const 24
    i32.load8_s
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I32Load8U from memory[24]
    i32.const 24
    i32.load8_u
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I32Load16S from memory[25]
    i32.const 25
    i32.load16_s
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I32Load16U from memory[25]
    i32.const 25
    i32.load16_u
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I64Load8S from memory[27]
    i32.const 27
    i64.load8_s
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load8U from memory[27]
    i32.const 27
    i64.load8_u
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load16S from memory[28]
    i32.const 28
    i64.load16_s
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load16U from memory[28]
    i32.const 28
    i64.load16_u
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load32S from memory[30]
    i32.const 30
    i64.load32_s
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load32U from memory[30]
    i32.const 30
    i64.load32_u
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; Now, to test the store instructions further, let's perform additional operations
    ;; We'll modify the stored values, store them again, and read back to include in sum

    ;; Modify sum (e.g., increment by 1)
    local.get $sum
    i32.const 1
    i32.add
    local.set $sum

    ;; I32Store8: store sum at memory[35]
    i32.const 35
    local.get $sum
    i32.store8

    ;; Read back with I32Load8U
    i32.const 35
    i32.load8_u
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I32Store16: store sum at memory[36]
    i32.const 36
    local.get $sum
    i32.store16

    ;; Read back with I32Load16U
    i32.const 36
    i32.load16_u
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I64Store8: store sum as i64 at memory[38]
    i32.const 38
    local.get $sum
    i64.extend_i32_s
    i64.store8

    ;; Read back with I64Load8U
    i32.const 38
    i64.load8_u
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Store16: store sum as i64 at memory[39]
    i32.const 39
    local.get $sum
    i64.extend_i32_s
    i64.store16

    ;; Read back with I64Load16U
    i32.const 39
    i64.load16_u
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Store32: store sum as i64 at memory[41]
    i32.const 41
    local.get $sum
    i64.extend_i32_s
    i64.store32

    ;; Read back with I64Load32U
    i32.const 41
    i64.load32_u
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; Call proc_exit with sum as exit code
    local.get $sum
    i32.const 126
    i32.rem_s
    call $proc_exit
  )

  ;; Export _start
  (export "_start" (func $_start))
)
