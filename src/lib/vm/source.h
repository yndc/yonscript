#pragma once

struct source_pos
{
    unsigned int line;
    unsigned int col;
    source_pos(unsigned int line, unsigned int col);
};