#include <stdio.h>
#include <stdlib.h>
int main() {
  int *m = malloc(sizeof(int));
  *m = 12;
  printf("%d\n", *m);

  free(m);
  free(m);

  printf("hi\n");
}
