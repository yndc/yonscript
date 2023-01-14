#include "lib/collection/stack.h"

template <typename T, unsigned int N>
unsigned int fixed_stack<T, N>::size()
{
    return this->top;
}

template <typename T, unsigned int N>
unsigned int fixed_stack<T, N>::push(T item)
{
    this->data[this->top] = item;
    this->top++;
    return this->top;
}

template <typename T, unsigned int N>
T fixed_stack<T, N>::pop()
{
    this->top--;
    return this->data[this->top];
}

template <typename T, unsigned int N>
T fixed_stack<T, N>::peek()
{
    return this->data[this->top - 1];
}