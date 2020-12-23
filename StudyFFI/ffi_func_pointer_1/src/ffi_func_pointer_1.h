#ifndef R_FFI_FUNC_POINTER_1_H
#define R_FFI_FUNC_POINTER_1_H

#include <inttypes.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct _Opaque Opaque;

Opaque* create();
void on_read(Opaque* ctx, void (*func)(Opaque* ctx, int data));
void enter_event_loop(Opaque* ctx);

#ifdef __cplusplus
}
#endif

#endif
