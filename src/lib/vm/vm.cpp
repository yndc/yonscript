#include <iostream>
#include <sstream>
#include "lib/collection/stack.h"
#include "lib/vm/vm.h"
#include "lib/vm/debug.h"

#define READ_NEXT_BYTE() (*ip++)
#define READ_NEXT_FLOAT() c->floats[READ_NEXT_BYTE()]

interpret_result vm::execute(chunk *c)
{
    uint8_t *ip = &c->code[0];
    static const void *ops[] = {
        &&OP_RETURN,
        &&OP_CONSTANT,
    };
    while (true)
    {

#ifdef DEBUG_TRACE_EXECUTION
        std::cout << "          ";
        for (unsigned int i = 0; i < this->float_stack.size(); i++)
        {
            std::cout << "[" << this->float_stack[i] << "]" << std::endl;
        }
        printf("\n");
        std::stringstream *buffer = new std::stringstream();
        disassemble_instruction(c, buffer, (int)(ip - &c->code[0]));
        std::cout << buffer->str();
#endif

        goto *ops[READ_NEXT_BYTE()];
    OP_CONSTANT:
        float value = READ_NEXT_FLOAT();
        this->float_stack.push(value);
        continue;
    OP_RETURN:
        return interpret_result::OK;
    }
}

#undef READ_NEXT_BYTE
#undef READ_NEXT_FLOAT