#pragma once 

#include "chunk.h"

/**
 * Disassembles the given chunk
 */
void disassemble_chunk(chunk* c, const char* name);

/**
 * Disassembles the given instruction
 */
void disassemble_instruction(chunk* c, int offset);