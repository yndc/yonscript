#include <string>
#include <fmt/format.h>
#include <iostream>
#include <sstream>
#include "debug.h"

static int simple_instruction(chunk *c, const char *name, std::stringstream *buffer, int offset)
{
    *buffer << fmt::format("{}", name) << std::endl;
    return offset + 1;
}

// template <typename T>
static int constant_instruction(chunk *c, const char *name, std::stringstream *buffer, int offset)
{
    uint8_t constantIndex = c->code[offset + 1];
    *buffer << fmt::format("{}\t{}", name, c->floats[constantIndex]) << std::endl;
    return offset + 2;
}

std::stringstream *disassemble_chunk(chunk *c, const char *name)
{
    std::stringstream *buffer = new std::stringstream();
    *buffer << fmt::format("chunk: {}", name) << std::endl;
    *buffer << "------------------------------------------------" << std::endl;
    *buffer << fmt::format("offset\tline\tcol\tinstruction\tvalue") << std::endl;
    for (int offset = 0; offset < (int)c->code.size();)
    {
        offset = disassemble_instruction(c, buffer, offset);
    }

    return buffer;
}

int disassemble_instruction(chunk *c, std::stringstream *buffer, int offset)
{
    *buffer << fmt::format("{:04}\t", offset);
    if (offset > 0 && c->pos[offset].line == c->pos[offset - 1].line)
        *buffer << "|\t";
    else
        *buffer << fmt::format("{:04}\t", c->pos[offset].line);
    *buffer << fmt::format("{:04}\t", c->pos[offset].col);

    uint8_t instruction = c->code[offset];
    switch (instruction)
    {
    case OP_CONSTANT:
        return constant_instruction(c, "OP_CONSTANT", buffer, offset);
    case OP_NEGATE:
        return simple_instruction(c, "OP_NEGATE", buffer, offset);
    case OP_ADD:
        return simple_instruction(c, "OP_ADD", buffer, offset);
    case OP_SUBTRACT:
        return simple_instruction(c, "OP_SUBTRACT", buffer, offset);
    case OP_MULTIPLY:
        return simple_instruction(c, "OP_MULTIPLY", buffer, offset);
    case OP_DIVIDE:
        return simple_instruction(c, "OP_DIVIDE", buffer, offset);
    case OP_RETURN:
        return simple_instruction(c, "OP_RETURN", buffer, offset);
    default:
        printf("Unknown opcode %d\n", instruction);
        return offset + 1;
    }
}
