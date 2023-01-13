#ifndef vm_h
#define vm_h

#include "chunk.h"

typedef struct {
  Chunk* chunk;
} VM;

/**
 * Initialize the virtual machine
*/
void vm_init();

/**
 * Free the virtual machine
*/
void vm_free();

#endif