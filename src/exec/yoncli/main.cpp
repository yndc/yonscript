#include "vm.h"
#include "chunk.h"
#include <iostream>

int main(int argc, const char* argv[]) {
    chunk* c = new chunk();
    c->write(OP_RETURN);
    std::cout << "omg" << std::endl;
    return 0;
}