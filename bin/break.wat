
(module
  ;; Import the WASI "wasi_snapshot_preview1" (necessary for WASI modules)
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory (export "memory") 1)

  ;; Define the start function
  (func (export "_start")
    (block $b (result i32) 
           (block 
             i32.const 0
             
             (block 
               
                (i32.const 3) 
                (i32.const 4) 
                (i32.const 5) 
                (i32.const 7) 
                (br $b)
                drop
                drop
                drop
              )

             drop
             ) 

           i32.const 1
           i32.const 2
           i32.add
           call $proc_exit
           i32.const 2
        )
    

    call $proc_exit
  )
)
