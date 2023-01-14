#pragma once

template <typename T, unsigned int N>
class fixed_stack
{
    T data[N];
    unsigned int cap;
    unsigned int top;

public:
    unsigned int size();
    unsigned int push(T item);
    T pop();
    T peek();
    T operator[](unsigned int index);
};
