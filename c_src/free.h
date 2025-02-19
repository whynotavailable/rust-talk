#pragma once

#include <string>

class Free {
    std::string *str;

public:
    Free();
    ~Free();

    void Print();
};
