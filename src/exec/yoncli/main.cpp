#include "common.hpp"
#include "vm.hpp"
#include "chunk.hpp"
#include <iostream>

int main(int argc, const char* argv[]) {
    chunk* c = new chunk();
    c->write(OP_RETURN);
    std::cout << "omg" << std::endl;
    return 0;
}