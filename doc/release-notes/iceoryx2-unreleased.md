# iceoryx2 v?.?.?

## [v?.?.?](https://github.com/eclipse-iceoryx/iceoryx2/tree/v?.?.?)

[Full Changelog](https://github.com/eclipse-iceoryx/iceoryx2/compare/v?.?.?...v?.?.?)

### Features

<!--
    NOTE: Add new entries sorted by issue number to minimize the possibility of
    conflicts when merging.
-->

* `iox2 config explain` cli command for config descriptions
  [#832](https://github.com/eclipse-iceoryx/iceoryx2/issues/832)
* Add a C++ string container type with fixed compile-time capacity
  [#938](https://github.com/eclipse-iceoryx/iceoryx2/issues/938)
* Add a C++ vector container type with fixed compile-time capacity
  [#951](https://github.com/eclipse-iceoryx/iceoryx2/issues/951)
* Use `epoll` instead of `select` for the `WaitSet` on Linux
  [#961](https://github.com/eclipse-iceoryx/iceoryx2/issues/961)
* Add a Rust vector type with fixed compile-time capacity which has the same
  memory layout as the C++ vector
  [#1073](https://github.com/eclipse-iceoryx/iceoryx2/issues/1073)
* Add a Rust string type with fixed compile-time capacity which has the same
  memory layout as the C++ vector
  [#1075](https://github.com/eclipse-iceoryx/iceoryx2/issues/1075)

### Bugfixes

<!--
    NOTE: Add new entries sorted by issue number to minimize the possibility of
    conflicts when merging.
-->

* Remove duplicate entries in `iox2` command search path to prevent discovered
  commands from being listed multiple times
    [#1045](https://github.com/eclipse-iceoryx/iceoryx2/issues/1045)
* LocalService in C language binding uses IPC configuration
    [#1059](https://github.com/eclipse-iceoryx/iceoryx2/issues/1059)
* Trait `std::fmt::Debug` is not implemented for `sigset_t` in libc
    [#1087](https://github.com/eclipse-iceoryx/iceoryx2/issues/1087)
*

### Refactoring

<!--
    NOTE: Add new entries sorted by issue number to minimize the possibility of
    conflicts when merging.
-->

* Example text [#1](https://github.com/eclipse-iceoryx/iceoryx2/issues/1)

### Workflow

<!--
    NOTE: Add new entries sorted by issue number to minimize the possibility of
    conflicts when merging.
-->

* Example text [#1](https://github.com/eclipse-iceoryx/iceoryx2/issues/1)

### New API features

<!--
    NOTE: Add new entries sorted by issue number to minimize the possibility of
    conflicts when merging.
-->

* Example text [#1](https://github.com/eclipse-iceoryx/iceoryx2/issues/1)

### API Breaking Changes

1. **Rust:** Replaced the `FixedSizeVec` with the `StaticVec`

   ```rust
   // old
   use iceoryx2_bb_container::vec::FixedSizeVec;
   const VEC_CAPACITY: usize = 1234;
   let my_vec = FixedSizeVec::<MyType, VEC_CAPACITY>::new();

   // new
   use iceoryx2_bb_container::vector::*;
   const VEC_CAPACITY: usize = 1234;
   let my_vec = StaticVec::<MyType, VEC_CAPACITY>::new();
   ```

2. **Rust:** Replaced `Vec` with the `PolymorphicVec`

    ```rust
   // old
   use iceoryx2_bb_container::vec::Vec;
   const VEC_CAPACITY: usize = 1234;
   let my_vec = Vec::<MyType>::new();

   // new
   use iceoryx2_bb_container::vector::*;
   let my_stateful_allocator = acquire_allocator();
   let vec_capacity: usize = 1234;
   let my_vec = PolymorphicVec::<MyType>::new(my_stateful_allocator, vec_capacity)?;
    ```

3. **Rust:** Replaced the `FixedSizeByteString` with the `StaticString`

   ```rust
   // old
   use iceoryx2_bb_container::byte_string::FixedSizeString;
   const CAPACITY: usize = 1234;
   let my_str = FixedSizeByteString::<CAPACITY>::new();

   // new
   use iceoryx2_bb_container::string::*;
   const CAPACITY: usize = 1234;
   let my_str = StaticString::<CAPACITY>::new();
   ```

4. **C++:** Remove `operator*` and `operator->` from `ActiveRequest`,
   `PendingResponse`, `RequestMut`, `RequestMutUninit`, `Response`,
   `ResponseMut`, `Sample`, `SampleMut`, `SampleMutUninit` since it lead
   to easy confusions and bugs when used in combination with `optional` or
   `expected`. See `sample.has_value()` and `sample->has_value()` that work
   on different objects.

   ```cxx
   // old
   auto sample = publisher.loan().expect("");
   sample->some_member = 123;

   // new
   auto sample = publisher.loan().expect("");
   sample.payload_mut().some_member = 123;
   ```

   ```cxx
   // old
   auto sample = publisher.loan().expect("");
   *sample = 123;
   std::cout << *sample << std::endl;

   // new
   auto sample = publisher.loan().expect("");
   sample.payload_mut() = 123;
   std::cout << sample.payload() << std::endl;
   ```
