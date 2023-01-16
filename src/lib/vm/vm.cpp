#include <iostream>
#include <sstream>
#include "lib/collection/stack.h"
#include "lib/vm/vm.h"
#include "lib/vm/debug.h"

#define STACK_MAX 256
#define READ_NEXT_BYTE() (*ip++)
#define READ_NEXT_FLOAT() c->floats[READ_NEXT_BYTE()]
#define BINARY_OP(op) float_stack[float_stack_top - 2] = float_stack[float_stack_top - 2] op float_stack[float_stack_top - 1]; float_stack_top--

interpret_result vm::execute(chunk *c)
{
    uint8_t *ip = &c->code[0];
    float float_stack[STACK_MAX];
    unsigned int float_stack_top;

    static const void *ops[] = {
        &&OP_CONSTANT,
        &&OP_NEGATE,
        &&OP_ADD,
        &&OP_SUBTRACT,
        &&OP_MULTIPLY,
        &&OP_DIVIDE,
        &&OP_RETURN,
    };

    while (true)
    {

#ifdef DEBUG_TRACE_EXECUTION
        std::cout << "stack: ";
        for (unsigned int i = 0; i < float_stack_top; i++)
        {
            std::cout << "[" << float_stack[i] << "] ";
        }
        printf("\n");
        std::stringstream *buffer = new std::stringstream();
        disassemble_instruction(c, buffer, (int)(ip - &c->code[0]));
        std::cout << buffer->str();
#endif

        goto *ops[READ_NEXT_BYTE()];

    OP_CONSTANT:
        float value = READ_NEXT_FLOAT();
        float_stack[float_stack_top++] = value;
        continue;
    OP_NEGATE:
        float_stack[float_stack_top - 1] = -float_stack[float_stack_top - 1];
        continue;
    OP_ADD:
        BINARY_OP(+);
        continue;
    OP_SUBTRACT:
        BINARY_OP(-);
        continue;
    OP_MULTIPLY:
        BINARY_OP(*);
        continue;
    OP_DIVIDE:
        BINARY_OP(/);
        continue;
    OP_RETURN:
        return interpret_result::OK;
    }
}

#undef READ_NEXT_BYTE
#undef READ_NEXT_FLOAT
#undef BINARY_OP