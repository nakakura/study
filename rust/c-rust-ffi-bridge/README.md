# c-rust-ffi-bridge
Test repository for calling a shared object written in rust from c.
It is tested in Ubuntu 16.04.

```
$cargo build
$cc main.c -ldl
$./a.out
```

