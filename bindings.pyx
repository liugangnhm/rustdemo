from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct MyStruct:
    int32_t *number;

  # This function takes a box and does nothing with it.
  #
  # @param with_box The box to be dropped.
  void root(const MyStruct *with_box);

  MyStruct *create_my_struct_box();

  void create_my_struct_ref();

  # This function takes a box and drops it.
  void drop_box(int32_t *x);

  void drop_my_struct(MyStruct *x);

  # This function takes an optional box and drops it if it is not None.
  void drop_box_opt(int32_t *x);

  uint16_t *ffi_test();

  void create_my_struct_array(MyStruct **prt, int32_t *len);

  void drop_my_struct_array(MyStruct *prt);

  void ffi_test_2(uint16_t *p);
