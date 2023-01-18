#include <cstdlib>

// inline int GrowCapacity(int capacity = 10)
// {
//     return (capacity) < 8 ? 8 : (capacity) * 2;
// }

void *Reallocate(void *ptr, size_t newSize)
{
    if (newSize == 0)
    {
        free(ptr);
        return NULL;
    }

    void *result = realloc(ptr, newSize);

    // Out of memory!
    if (result == NULL)
        exit(1);

    return result;
}

