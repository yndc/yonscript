#pragma once

#include "lib/collection/stack.h"
#include "chunk.h"

#define STACK_MAX 256

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
  fixed_stack<float, STACK_MAX> float_stack;

public:
  /**
   * Execute the given chunk of code
   * @param c chunk of code
   */
  interpret_result execute(chunk *c);
};
