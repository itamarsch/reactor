

(module
  (memory (export "memory") 1)
  
  (func (export "_start")
    ;; Base-level if statement
    (if (i32.const 1)  ;; Condition (always true in this example)
      (then
        ;; Then branch with nested and neighboring blocks
        (block $outer_then
          ;; Neighboring block
          (block $inner_then_1
            (i32.const 10)
            drop
          )
          ;; Another neighboring block
          (block $inner_then_2
            ;; Nested block inside a neighboring block
            (block $deep_inner_then
              (i32.const 20)
              drop
            )
            (i32.const 30)
            drop
          )
        )
        ;; Neighboring block at the same level as $outer_then
        (block $then_neighbor
          (i32.const 40)
          drop
        )
      )
      (else
        ;; Else branch with nested and neighboring blocks
        (block $outer_else
          ;; Neighboring block
          (block $inner_else_1
            (i32.const 50)
            drop
          )
          ;; Another neighboring block
          (block $inner_else_2
            ;; Nested block inside a neighboring block
            (block $deep_inner_else
              (i32.const 60)
              drop
            )
            (i32.const 70)
            drop
          )
        )
        ;; Neighboring block at the same level as $outer_else
        (block $else_neighbor
          (i32.const 80)
          drop
        )
      )
    )
  )
)
