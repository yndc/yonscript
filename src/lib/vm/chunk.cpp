#include "chunk.h"

void chunk::write(uint8_t byte)
{
  this->code.push_back(byte);
}
