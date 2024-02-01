#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct MyStruct {
  int32_t *number;
} MyStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * This function takes a box and does nothing with it.
 *
 * @param with_box The box to be dropped.
 */
void root(const struct MyStruct *with_box);

/**
 * This function takes a box and drops it.
 */
void drop_box(int32_t *x);

void drop_my_struct(struct MyStruct *x);

/**
 * This function takes an optional box and drops it if it is not None.
 */
void drop_box_opt(int32_t *x);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
