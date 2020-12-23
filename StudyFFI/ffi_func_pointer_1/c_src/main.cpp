#include <stdio.h>
#include "ffi_func_pointer_1.h"

void onReadData(Opaque* ctx, int data) { printf("data = %d\n", data); }

int main() {
  Opaque* ctx = create();
  on_read(ctx, onReadData);
  enter_event_loop(ctx);
  return 0;
}
