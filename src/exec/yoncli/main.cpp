#include "lib/vm/vm.h"
#include "lib/vm/chunk.h"
#include "lib/vm/debug.h"
#include "lib/vm/source.h"
#include <iostream>
#include <sstream>

int main(int argc, const char *argv[])
{
    chunk *c = new chunk();

    c->write_instruction(OP_CONSTANT, source_pos(1, 23));
    c->write_instruction(c->write_constant_float(1.2), source_pos(1, 23));
    c->write_instruction(OP_CONSTANT, source_pos(1, 23));
    c->write_instruction(c->write_constant_float(1.2), source_pos(1, 23));
    c->write_instruction(OP_CONSTANT, source_pos(1, 23));
    c->write_instruction(c->write_constant_float(1.2), source_pos(1, 23));
    c->write_instruction(OP_CONSTANT, source_pos(2, 23));
    c->write_instruction(c->write_constant_float(1.2), source_pos(2, 23));
    c->write_instruction(OP_NEGATE, source_pos(2, 23));
    c->write_instruction(OP_CONSTANT, source_pos(2, 23));
    c->write_instruction(c->write_constant_float(3), source_pos(2, 23));
    c->write_instruction(OP_ADD, source_pos(10, 10));
    c->write_instruction(OP_MULTIPLY, source_pos(10, 10));
    c->write_instruction(OP_DIVIDE, source_pos(10, 10));
    c->write_instruction(OP_SUBTRACT, source_pos(10, 10));
    c->write_instruction(OP_RETURN, source_pos(5, 23));

    vm v;
    interpret_result r = v.execute(c);

    std::cout << r;

    // std::stringstream *buffer = disassemble_chunk(c, "test chunk");
    // std::cout << buffer->str();

    return 0;
}