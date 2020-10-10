#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

#include "callrust.h"

int main() {
  print_hello_from_rust();

  uint32_t numbers[6] = {1, 2, 3, 4, 5, 6};
  uint32_t sum = sum_of_even(numbers, 6);
  printf("sum = %d\n", sum);

  uint32_t count = hm_chars("The tao of Rust");
  printf("%d\n", count);

  char* song = batman_song(5);
  printf("%s\n", song);
  free_song(song);

  STuple initial = {.x = 10, .y = 20};
  STuple newTuple = flip_things_around(initial);
  printf("(%d,%d)\n", newTuple.x, newTuple.y);

  SDatabase* pstDatabase = database_new();
  database_insert(pstDatabase);
  uint32_t pop1 = database_query(pstDatabase, "10186");
  uint32_t pop2 = database_query(pstDatabase, "10852");
  database_free(pstDatabase);
  printf("%d\n", pop2 - pop1);

  return 0;
}
