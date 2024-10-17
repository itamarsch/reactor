(module
  ;; Import proc_exit
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

  ;; Declare memory
  (memory 1) ;; Define 1 page of memory (64KiB)

  ;; Export the memory so that we can inspect it if needed
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
    i32.const 0     ;; address
    i32.const 123   ;; value
    i32.store       ;; store i32 at memory[0]

    i32.const 4
    i64.const 456
    i64.store

    i32.const 12
    f32.const 789.0
    f32.store

    i32.const 16
    f64.const 101112.0
    f64.store

    ;; I32Load
    i32.const 0
    i32.load
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I64Load
    i32.const 4
    i64.load
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; F32Load
    i32.const 12
    f32.load
    local.set $f32_val

    ;; sum += i32.trunc_s/f32($f32_val)
    local.get $sum
    local.get $f32_val
    i32.trunc_f32_s
    i32.add
    local.set $sum

    ;; F64Load
    i32.const 16
    f64.load
    local.set $f64_val

    ;; sum += i32.trunc_s/f64($f64_val)
    local.get $sum
    local.get $f64_val
    i32.trunc_f64_s
    i32.add
    local.set $sum


    ;; I32Load8S
    i32.const 0
    i32.load8_s
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I32Load8U
    i32.const 0
    i32.load8_u
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I32Load16S
    i32.const 0
    i32.load16_s
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I32Load16U
    i32.const 0
    i32.load16_u
    local.set $i32_val

    ;; sum += $i32_val
    local.get $sum
    local.get $i32_val
    i32.add
    local.set $sum

    ;; I64Load8S
    i32.const 4
    i64.load8_s
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load8U
    i32.const 4
    i64.load8_u
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load16S
    i32.const 4
    i64.load16_s
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load16U
    i32.const 4
    i64.load16_u
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load32S
    i32.const 4
    i64.load32_s
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I64Load32U
    i32.const 4
    i64.load32_u
    local.set $i64_val

    ;; sum += i32.wrap_i64($i64_val)
    local.get $sum
    local.get $i64_val
    i32.wrap_i64
    i32.add
    local.set $sum

    ;; I32Store
    i32.const 24
    local.get $sum
    i32.store

    ;; I64Store
    i32.const 28
    local.get $sum
    i64.extend_i32_s
    i64.store

    ;; F32Store
    i32.const 36
    local.get $sum
    f32.convert_i32_s
    f32.store

    ;; F64Store
    i32.const 40
    local.get $sum
    f64.convert_i32_s
    f64.store

    ;; I32Store8
    i32.const 48
    local.get $sum
    i32.store8

    ;; I32Store16
    i32.const 49
    local.get $sum
    i32.store16

    ;; I64Store8
    i32.const 51
    local.get $sum
    i64.extend_i32_s
    i64.store8

    ;; I64Store16
    i32.const 52
    local.get $sum
    i64.extend_i32_s
    i64.store16

    ;; I64Store32
    i32.const 54
    local.get $sum
    i64.extend_i32_s
    i64.store32

    ;; Call proc_exit with sum as exit code
    local.get $sum
    i32.const 126
    i32.rem_s
    call $proc_exit
  )

  ;; Export _start
  (export "_start" (func $_start))
)
