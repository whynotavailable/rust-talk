/*
 * This file is a comparison example of a struct with a method in C.
 * To be compared to a similar example in Rust.
 */
#include <stdio.h>

struct example {
  char *data;
};

struct example example_new(char *data) {
  struct example ret;
  ret.data = data;

  return ret;
}
void example_print(struct example *self) { printf("hi %s\n", self->data); }

int main() {
  struct example hi = example_new("dave");

  example_print(&hi);
}
