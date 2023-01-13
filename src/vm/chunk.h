#ifndef chunk_h
#define chunk_h

#include "common.h"
#include "op.h"

typedef struct
{
    int count;
    int capacity;
    uint8_t *code;
} Chunk;

void chunk_init(Chunk *chunk);

void chunk_write(Chunk* chunk, uint8_t byte);

#endif