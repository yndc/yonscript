#pragma once

#include "lib/collection/stack.h"
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
  /**
   * Execute the given chunk of code
   * @param c chunk of code
   */
  interpret_result execute(chunk *c);
};
