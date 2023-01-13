#include "chunk.hpp"

void chunk::write(uint8_t byte)
{
  this->code.push_back(byte);
}

// void chunk_write(Chunk* chunk, uint8_t byte) {
//   if (chunk->capacity < chunk->count + 1) {
//     int oldCapacity = chunk->capacity;
//     chunk->capacity = GROW_CAPACITY(oldCapacity);
//     chunk->code = GROW_ARRAY(uint8_t, chunk->code,
//         oldCapacity, chunk->capacity);
//   }

//   chunk->code[chunk->count] = byte;
//   chunk->count++;
// }