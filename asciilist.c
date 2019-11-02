#include <stdio.h>
#include <ctype.h>

int main(int argc, char **argv) {
  if (argc < 3) return 0;
  char c = *argv[1];
  char to = *argv[2];
  if (!islower(c) || !islower(to))
    return 1;
  for (int i = c; i <= to; i++) {
    printf("%c ", i);
  }

  return 0;
}
