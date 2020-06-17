#include <stdio.h>
#include <sys/time.h>
#include "rshm.h"

/* get system time */
static inline void itimeofday(long *sec, long *usec)
{
	struct timeval time;
	gettimeofday(&time, NULL);
	if (sec) *sec = time.tv_sec;
	if (usec) *usec = time.tv_usec;
}

/* get clock in millisecond 64 */
static inline int64_t iclock64(void)
{
	long s, u;
	int64_t value;
	itimeofday(&s, &u);
	value = ((int64_t)s) * 1000 + (u / 1000);
	return value;
}

int main() {
  printf("rshm client\n");

  SRshm *rshm = rshm_create("rshmtest");
  size_t len = 10240;

  char acBuf1[10240] = {0};
  for (int i = 0; i < 10240; i++) {
    acBuf1[i] = i % 256;
  }

  char acBuf2[10240] = {0};
  for (int i = 0; i < 10240; i++) {
    acBuf2[i] = 256 - i % 256;
  }

  char acBuf3[10240] = {0};
  for (int i = 0; i < 10240; i++) {
    acBuf3[i] = 18;
  }

  int iBufIndex = 0;
  char* apcBuf[3] = {0};
  apcBuf[0] = acBuf1;
  apcBuf[1] = acBuf2;
  apcBuf[2] = acBuf3;

  int64_t start = iclock64();

  for (int i = 0; i < 10000; i++) {
    iBufIndex = iBufIndex % 3;
    int iRet = rshm_write(rshm, apcBuf[iBufIndex++], len, 5);
    if (iRet <= 0) {
      printf("iRet = %d\n", iRet);
      continue;
    }
    printf("write %d bytes\n", iRet);
  }

  int64_t end = iclock64();
  printf("elpsed = %ld\n", end - start);
}
