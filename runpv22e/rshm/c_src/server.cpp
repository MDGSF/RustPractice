#include <stdio.h>
#include <signal.h>
#include "rshm.h"

static volatile int g_iRunning = 1;

void sigterm(int iSigno) { g_iRunning = 0; }

int main() {
  printf("rshm server\n");

  signal(SIGTERM, sigterm);
  signal(SIGHUP, sigterm);
  signal(SIGINT, sigterm);

  SRshm *rshm = rshm_create("rshmtest");
  if (NULL == rshm) {
    return 1;
  }

  while (g_iRunning) {
    size_t len = 10240;
    char acBuf[10240] = {0};
    int iRet = rshm_read(rshm, acBuf, len, 5);
    if (iRet <= 0) {
      printf("iRet = %d\n", iRet);
      continue;
    }
    printf("read %d bytes\n", iRet);
    // int iCrCount = 1;
    // for (int i = 0; i < iRet; i++) {
    //   printf("%05d ", (unsigned char)acBuf[i]);
    //   if (iCrCount++ % 10 == 0) {
    //     printf("\n");
    //   }
    // }
    // printf("\n\n\n");
  }

  // rshm_release(rshm);
  return 0;
}
