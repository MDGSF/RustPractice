#ifndef RUST_SHM_H
#define RUST_SHM_H

#include <inttypes.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
@brief: rshm opaque struct type.
*/
typedef struct _SRshm SRshm;

SRshm *rshm_init(const char *name);
SRshm *rshm_open(const char *name);
int rshm_release(SRshm *);
int rshm_write(SRshm *, const char *buf, size_t len, int timeout);
int rshm_read(SRshm *, char *buf, size_t len, int timeout);

#ifdef __cplusplus
}
#endif

#endif
