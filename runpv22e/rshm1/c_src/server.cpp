#include <stdio.h>
#include <signal.h>
#include "rshm1.h"

static volatile int g_iRunning = 1;

void sigterm(int iSigno) { g_iRunning = 0; }

int main(int argc, char *argv[]) {
  printf("rshm server\n");

  signal(SIGTERM, sigterm);
  signal(SIGHUP, sigterm);
  signal(SIGINT, sigterm);

  if (argc != 2) {
    printf("usage: server readername");
    return 0;
  }

  SRshm *rshm = rshm_create("rshmtest", argv[1]);
  if (NULL == rshm) {
    return 1;
  }

  int iCount = 0;
  while (g_iRunning) {
    size_t len = 8;
    char acBuf[8] = {0};
    int iRet = rshm_read(rshm, acBuf, len, 5);
    if (iRet <= 0) {
      printf("iRet = %d\n", iRet);
      continue;
    }
    printf("[%d] read %d bytes, [ ", iCount++, iRet);
    for (int i = 0; i < iRet; i++) {
      printf("%02d ", (unsigned char)acBuf[i]);
    }
    printf("]\n");
  }

  rshm_release(rshm);
  return 0;
}
