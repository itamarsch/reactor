
(module
  (memory (export "memory") 1)

  ;; Define the start function
  (func (export "_start")
    ;; Base-level if statement with a condition
    (if (i32.eqz (i32.const 0))  ;; Condition (always true)
      (then
        ;; Then branch with complex nested and neighboring blocks and breaks
        (block $then_outer
          ;; First neighboring block
          (block $then_block1
            ;; Nested block with break
            (block $then_nested1
              (i32.const 10)
              (br $then_block1)  ;; Break to $then_block1
            )
            ;; This code is skipped due to the break above
            (i32.const 20)
            drop
          )
          ;; Second neighboring block
          (block $then_block2
            ;; Nested block with conditional break
            (block $then_nested2
              (i32.const 30)
              
              (br_if $then_block2 (i32.const 1))  ;; Conditional break to $then_block2
              drop
            )
            ;; This code is skipped if condition is true
            (i32.const 40)
            drop
          )
          ;; Deeply nested blocks with break to outer block
          (block $then_block3
            (block $then_nested3
              (block $then_deep_nested
                (i32.const 50)
                (br $then_outer)  ;; Break to $then_outer
              )
              ;; This code is skipped due to the break above
              (i32.const 60)
              drop
            )
            ;; This code is also skipped due to the break
            (i32.const 70)
            drop
          )
        )
        ;; Neighboring block at the same level as $then_outer
        (block $then_neighbor
          (i32.const 80)
          drop
        )
      )
      (else
        ;; Else branch with complex nested and neighboring blocks and breaks
        (block $else_outer
          ;; First neighboring block
          (block $else_block1
            ;; Nested block with break
            (block $else_nested1
              (i32.const 90)
              (br $else_block1)  ;; Break to $else_block1
            )
            ;; This code is skipped due to the break above
            (i32.const 100)
            drop
          )
          ;; Second neighboring block
          (block $else_block2
            ;; Nested block with conditional break
            (block $else_nested2
              (i32.const 110)
              (br_if $else_block2 (i32.const 1))  ;; Conditional break to $else_block2
              drop
            )
            ;; This code is skipped if condition is true
            (i32.const 120)
            drop
          )
          ;; Deeply nested blocks with break to outer block
          (block $else_block3
            (block $else_nested3
              (block $else_deep_nested
                (i32.const 130)
                (br $else_outer)  ;; Break to $else_outer
              )
              ;; This code is skipped due to the break above
              (i32.const 140)
              drop
            )
            ;; This code is also skipped due to the break
            (i32.const 150)
            drop
          )
        )
        ;; Neighboring block at the same level as $else_outer
        (block $else_neighbor
          (i32.const 160)
          drop
        )
      )
    )
  )
)
