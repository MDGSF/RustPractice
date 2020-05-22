#ifndef CPP_CALL_RUST_H
#define CPP_CALL_RUST_H

#include <inttypes.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

void print_hello_from_rust();

uint32_t sum_of_even(const uint32_t *numbers, size_t length);

uint32_t hm_chars(const char *str);

char *batman_song(uint8_t length);
void free_song(char *);

typedef struct {
  uint32_t x;
  uint32_t y;
} STuple;
STuple flip_things_around(STuple);

typedef struct _SDatabase SDatabase;
SDatabase *database_new();
void database_free(SDatabase *);
void database_insert(SDatabase *);
uint32_t database_query(SDatabase *, const char *zip);

#ifdef __cplusplus
}
#endif

#endif
