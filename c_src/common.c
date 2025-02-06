/*
 * This file is a comparison example of a struct with a method in C.
 * To be compared to a similar example in Rust.
 */
#include <stdio.h>

typedef struct {
  char *data;
} example;

example example_new(char *data) {
  example ret;
  ret.data = data;
  return ret;
}
void example_print(example *self) { printf("hi %s\n", self->data); }

int main() {
  example hi = example_new("dave");

  example_print(&hi);
}
