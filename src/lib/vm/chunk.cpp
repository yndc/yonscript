#include "chunk.h"

void chunk::write_instruction(uint8_t byte, source_pos pos)
{
  this->code.push_back(byte);
  this->pos.push_back(pos);
}

int chunk::write_constant_float(float value)
{
  this->floats.push_back(value);
  return (int)this->floats.size();
}