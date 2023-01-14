#pragma once

#include <vector>
#include "common.h"
#include "op.h"

class chunk
{
    std::vector<uint8_t> code;

public:
    /**
     * write a byte into the chunk
     */
    void write(uint8_t byte);
};
