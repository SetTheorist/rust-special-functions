- f128
- standard complex number type(s)
- hex/octal/binary float literals
- const floating-point ops
- nicer way to specify custom type numeric literals
  - c.f. C++ user-defined literals or Haskell overloaded literals
  - some kind of compile-time proc-macro that takes in literal representation and returns value?
    - integer, float, string

- resolution of Rust issues #86635, #20671 would be extremely helpful
  - left-embedding
  - lifting all types not equivalent
    - 0+(-0) != -0

- trait specialization and negative traits would help immensely

- custom operators (infix) would be great, though highly unlikely
