#include "bindings.h"
#include <stdio.h>

int main()
{
  struct MyStruct *s = create_my_struct_box();

  printf("Number from rust: %d\n", *s->number);

  *s->number = 1000;
  drop_my_struct(s);

  printf("after drop Number from rust: %d\n", *s->number);

  uint16_t *ptr_from_rust = ffi_test();
  printf("Number from rust: %d\n", *ptr_from_rust);
  printf("Number from rust: %d\n", *(ptr_from_rust + 1));
  printf("Number from rust: %d\n", *(ptr_from_rust + 2));
  printf("Number from rust: %d\n", *(ptr_from_rust + 3));
  ffi_test_2(ptr_from_rust);

  printf("after free: Number from rust: %d\n", *ptr_from_rust);
  printf("after free: Number from rust: %d\n", *(ptr_from_rust + 1));
  printf("after free: Number from rust: %d\n", *(ptr_from_rust + 2));
  printf("after free: Number from rust: %d\n", *(ptr_from_rust + 3));

  struct MyStruct *ptr;
  int len;
  create_my_struct_array(&ptr, &len);
  printf("len: %d\n", len);
  for (int i = 0; i < len; i++)
  {
    printf("Number from rust: %d\n", *(ptr + i)->number);
  }

  drop_my_struct_array(ptr);

  return 0;
}
