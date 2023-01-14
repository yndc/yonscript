#include "lib/vm/vm.h"
#include "lib/vm/chunk.h"
#include "lib/vm/debug.h"
#include <iostream>

int main(int argc, const char* argv[]) {
    chunk* c = new chunk();
    c->write(OP_RETURN);
    
    disassemble_chunk(c, "test chunk");

    return 0;
}