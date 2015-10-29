
#include <stdio.h>
#include <stdlib.h>

typedef struct {
    char* file;
    char* kind;
    char* name;
    unsigned int row;
    unsigned int col;
} c_mentry;

typedef struct {
    c_mentry* entries;
    size_t len;
} c_mentries;

extern c_mentries get_entries();

int main() {
  c_mentries res = get_entries();
  int i = 0;
  for (i = 0; i < res.len; i++) {
    printf("%s %s %s %d %d\n", res.entries[i].name,res.entries[i].kind,res.entries[i].file,res.entries[i].row,res.entries[i].col);
  }
  return 0;
}
