#pragma once 

#include <cstdlib>

// /**
//  * Calculate a new bigger capacity from the given array capacity
//  * @param old original capacity as the baseline
//  */
// inline int GrowCapacity(int old);

/**
 * Reallocates a block of memory into a new block with different size
 * @param ptr pointer of the memory block
 * @param newSize designated memory block size
*/
void* Reallocate(void* ptr, size_t newSize);