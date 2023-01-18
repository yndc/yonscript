#pragma once

#include <vector>
#include "lib/vm/source.h"
#include "lib/common.h"
#include "op.h"

struct chunk
{
    std::vector<uint8_t> code;
    std::vector<float> floats;
    std::vector<source_pos> pos; //TODO: compress line using run-length encoding

    /**
     * write a byte into the chunk
     */
    void write_instruction(uint8_t byte, source_pos pos);

    /**
     * write a constant float into the chunk
     */
    int write_constant_float(float value);
};
