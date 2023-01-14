#pragma once

#include <sstream>
#include "chunk.h"

/**
 * Disassembles the given chunk
 */
std::stringstream* disassemble_chunk(chunk *c, const char *name);

/**
 * Disassembles the given instruction
 */
int disassemble_instruction(chunk *c, std::stringstream *buffer, int offset);