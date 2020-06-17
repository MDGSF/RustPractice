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

  SRshm *rshm = rshm_open("rshmtest");
  size_t len = 10240;
  char acBuf[10240] = {0};
  for (int i = 0; i < 10240; i++) {
    acBuf[i] = i;
  }

  int64_t start = iclock64();

  for (int i = 0; i < 10000; i++) {
    int iRet = rshm_write(rshm, acBuf, len, 2);
    if (iRet <= 0) {
      printf("iRet = %d\n", iRet);
      continue;
    }
    // printf("write %d bytes\n", iRet);
  }

  int64_t end = iclock64();
  printf("elpsed = %ld\n", end - start);
}
