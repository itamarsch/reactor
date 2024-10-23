
(module
  ;; Import the WASI function for exiting with a code
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

  ;; Define the memory and export it
  (memory $memory 1)  ;; 1 page = 64 KiB
  (export "memory" (memory $memory))

  ;; Define mutable globals of all kinds of types
  (global $g_i32 (mut i32) (i32.const 42))
  (global $g_i64 (mut i64) (i64.const 123456789))
  (global $g_f32 (mut f32) (f32.const 3.14))
  (global $g_f64 (mut f64) (f64.const 2.71828))

  ;; The start function
  (func $_start
    ;; Define local variables to hold global values
    (local $v_i32 i32)
    (local $v_i64 i64)
    (local $v_f32 f32)
    (local $v_f64 f64)
    (local $exit_code i32)

    ;; Load the globals into locals
    (local.set $v_i32 (global.get $g_i32))
    (local.set $v_i64 (global.get $g_i64))
    (local.set $v_f32 (global.get $g_f32))
    (local.set $v_f64 (global.get $g_f64))

    ;; Compute an exit code using the globals
    (local.set $exit_code
      (i32.wrap_i64
        (i64.rem_u
          (i64.add
            (i64.extend_i32_u (local.get $v_i32))
            (i64.add
              (local.get $v_i64)
              (i64.add
                (i64.trunc_f32_u (local.get $v_f32))
                (i64.trunc_f64_u (local.get $v_f64))
              )
            )
          )
          (i64.const 256)
        )
      )
    )

    ;; Call proc_exit with the computed exit code
    (call $proc_exit (local.get $exit_code))
  )

  ;; Export the start function as "_start" for WASI compatibility
  (export "_start" (func $_start))
)
