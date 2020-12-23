#include <signal.h>
#include <stdio.h>
#include <sys/time.h>
#include <unistd.h>
#include "rshm1.h"

/* get system time */
static inline void itimeofday(long *sec, long *usec) {
  struct timeval time;
  gettimeofday(&time, NULL);
  if (sec) *sec = time.tv_sec;
  if (usec) *usec = time.tv_usec;
}

/* get clock in millisecond 64 */
static inline int64_t iclock64(void) {
  long s, u;
  int64_t value;
  itimeofday(&s, &u);
  value = ((int64_t)s) * 1000 + (u / 1000);
  return value;
}

static volatile int g_iRunning = 1;

void sigterm(int iSigno) { g_iRunning = 0; }

int main() {
  printf("rshm client\n");

  signal(SIGTERM, sigterm);
  signal(SIGHUP, sigterm);
  signal(SIGINT, sigterm);

  SRshm *rshm = rshm_create("rshmtest", "testclient");
  size_t len = 8;

  char acBuf1[8] = {0};
  for (int i = 0; i < 8; i++) {
    acBuf1[i] = 1;
  }

  char acBuf2[8] = {0};
  for (int i = 0; i < 8; i++) {
    acBuf2[i] = 2;
  }

  char acBuf3[8] = {0};
  for (int i = 0; i < 8; i++) {
    acBuf3[i] = 3;
  }

  int iBufIndex = 0;
  char *apcBuf[3] = {0};
  apcBuf[0] = acBuf1;
  apcBuf[1] = acBuf2;
  apcBuf[2] = acBuf3;

  int64_t start = iclock64();

  for (int i = 0; g_iRunning && i < 10000; i++) {
    iBufIndex = iBufIndex % 3;
    char *pcCurBuf = apcBuf[iBufIndex++];
    int iRet = rshm_write(rshm, pcCurBuf, len);
    if (iRet <= 0) {
      printf("iRet = %d\n", iRet);
      continue;
    }
    printf("[%d] write %d bytes, [ ", i, iRet);
    for (int i = 0; i < iRet; i++) {
      printf("%02d ", (unsigned char)pcCurBuf[i]);
    }
    printf("]\n");
    usleep(10 * 1000);
  }

  int64_t end = iclock64();
  printf("elpsed = %ld\n", end - start);

  rshm_destory(rshm);
}
