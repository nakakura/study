#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

void repeat_hello(int32_t i);

int main(int argc, char const *argv[]) {
  if(argc<2) {
    printf("invalid arg.\n");
    exit(1);
  }
  int32_t i = atoi( argv[1] );
  repeat_hello(i);
  return 0;
}
