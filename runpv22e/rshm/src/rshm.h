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

/*
@brief: Create one share memory if it doesn't exist.
        If it already exists, it will just open for read and write.
@param name[in]: name for share memory.
@return On success: rshm.
        On error: NULL.
*/
SRshm *rshm_create(const char *name);

/*
@brief: Release rshm control object.
@param rshm[in]: rshm.
@return On success: 0.
        On error: error code below zero is returned.
*/
int rshm_release(SRshm *rshm);

/*
@brief: Write data to share memory.
@param rshm[in]: rshm.
@param buf[in]: Buffer write to share memory.
@param len[in]: Length of buf.
@param timeout[in]: Timeout for write, in seconds.
       < 0 means block forever if share memory is full.
       = 0 means non-block and will return fail immediately if share memory is full.
       > 0 means block timeout seconds if share memory is full.
@return On success: return the number of bytes write.
        On error: error code below zero is returned.
*/
int rshm_write(SRshm *rshm, const char *buf, size_t len, int timeout);

/*
@brief: Read data from share memory.
@param rshm[in]: rshm.
@param buf[out]: Buffer used to store received data.
@param len[in]: Length of buf.
@param timeout[in]: Timeout for read, in seconds.
       < 0 means block forever if share memory is empty.
       = 0 means non-block and will return fail immediately if share memory is empty.
       > 0 means block timeout seconds if share memory is empty.
@return On success: return the number of bytes read.
        On error: error code below zero is returned.
*/
int rshm_read(SRshm *rshm, char *buf, size_t len, int timeout);

#ifdef __cplusplus
}
#endif

#endif
