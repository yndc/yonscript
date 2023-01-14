#pragma once

#include "chunk.h"

/**
 * Interpretation result returned by a VM instance after running a chunk
 */
typedef enum
{
  OK,
  COMPILE_ERROR,
  RUNTIME_ERROR
} interpret_result;

/**
 * The virtual machine
 */
class vm
{
public:
  interpret_result interpret(chunk *c);
};
