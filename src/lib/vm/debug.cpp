#include <string>  
#include <fmt/format.h>  
#include <iostream> 
#include <sstream>   
#include "debug.h"

void disassemble_chunk(chunk* c, const char* name)
{
    std::stringstream buffer;
    buffer << fmt::format("== {} ==", name) << std::endl;
}

void disassemble_instruction(chunk* c, int offset)
{

}