#include "ReadBuffer.h"

int read_into_buffer(char* pcBuf, int iBufSize) {
  for (int i = 0; i < iBufSize; i++) {
    pcBuf[i] = i;
  }
  return iBufSize;
}
