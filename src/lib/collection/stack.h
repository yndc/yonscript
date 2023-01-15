#pragma once

template <typename T, unsigned int N>
class fixed_stack
{
    T data[N];
    unsigned int cap;
    unsigned int top;

public:
    unsigned int size()
    {
        return top;
    }
    unsigned int push(T item)
    {
        this->data[this->top] = item;
        this->top++;
        return this->top;
    }
    T set(unsigned int index, T value)
    {
        T old = this->data[index];
        this->data[index] = value;
        return old;
    }
    T pop()
    {
        this->top--;
        return this->data[this->top];
    }
    T peek()
    {
        return this->data[this->top - 1];
    }
    T operator[](unsigned int index)
    {
        return this->data[index];
    }
    T operator=(unsigned int index)
    {
        return this->data[index];
    }
};
