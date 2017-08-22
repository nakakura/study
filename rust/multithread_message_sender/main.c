#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

void* rustobj;
void* create_rust_object(char* address, unsigned short port);
void send_message(void* rustobj, char* message, int len);
unsigned int get_data(char* data);

int main(int argc, char const *argv[]) {
  if(argc != 2) {
    printf("invalid arg.\n");
    return -1;
  }


char data[100];
unsigned int len = get_data(&data);
printf("recv %d, data %s\n", len, data);

  printf("arg0 %d", strlen(argv[1]));
  rustobj = create_rust_object("127.0.0.1", 9000);
  send_message(rustobj, argv[1], strlen(argv[1]));

  for (int i = 1; i <= 10; i++) {
    char str[1500];
    sprintf(str, "loop %d",i);
    send_message(rustobj, str, strlen(str));
    printf("%d \n", strlen(str));
  }


  return 0;
}
