#include "free.h"
#include <cstdlib>
#include <iostream>
#include <string>

Free::Free() { this->str = new std::string("hi"); }
Free::~Free() {
    free(this->str);
    std::cout << "Freeing\n";
}

void Free::Print() { std::cout << *(this->str) << "\n"; }

void pass(Free f) { f.Print(); }

int main(void) {
    Free f;
    f.Print();
    pass(f);
}
