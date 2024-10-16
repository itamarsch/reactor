(module
  ;; Import the WASI 'proc_exit' function to exit the program
  (import "wasi_snapshot_preview1" "proc_exit"
    (func $__wasi_proc_exit (param i32)))

  ;; Declare a memory section (required for WASI modules)
  (memory (export "memory") 1)

  ;; Define the '_start' function which is the entry point for WASI modules
  (func $main (export "_start")
    ;; Declare local variables
    (local $n i32)        ;; The position in the Fibonacci sequence to compute
    (local $counter i32)  ;; Loop counter
    (local $a i32)        ;; Holds F(n-2)
    (local $b i32)        ;; Holds F(n-1)
    (local $temp i32)     ;; Temporary variable for swapping

    ;; Set n to 7
    (local.set $n (i32.const 7))

    ;; Initialize variables
    (local.set $counter (i32.const 0))
    (local.set $a (i32.const 0))    ;; F(0)
    (local.set $b (i32.const 1))    ;; F(1)

    ;; Start the loop
    (loop $loop
      ;; Check if counter equals n

      ;; Compute the next Fibonacci number
      ;; temp = b
      (local.get $b)
      (local.set $temp)

      ;; b = a + b
      (local.get $a)
      (local.get $b)
      (i32.add)
      (local.set $b)

      ;; a = temp
      (local.get $temp)
      (local.set $a)

      ;; Increment the counter
      (local.get $counter)
      (i32.const 1)
      (i32.add)
      (local.set $counter)

      (local.get $counter)
      (local.get $n)
      (i32.ne)
      (br_if $loop)              
    )

    ;; After the loop, $a holds the nth Fibonacci number
    ;; Call the WASI 'proc_exit' function to exit the program with the result code
    (call $__wasi_proc_exit (local.get $a))
  )
)
