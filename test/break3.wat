
(module
  ;; Import the WASI "wasi_snapshot_preview1" (necessary for WASI modules)
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Define the start function
  (func (export "_start")

    (block $label3
      (block $label4
        (block $label5

          (i32.const 10)
          (br_table $label5 $label4 $label3)

        )
        i32.const 8
        call $proc_exit
      )
      i32.const 9
      call $proc_exit
    )

    i32.const 10
    call $proc_exit
  )
)
