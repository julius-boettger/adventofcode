#include "day.hpp"
#if DAY == 1

#include <format>
#include <vector>
#include <fstream>
#include <sstream>
#include <iostream>
#include <algorithm>

std::string readInput() {
    std::ifstream inputFile { std::format("input/{:02}.txt", DAY) };
    if (!inputFile.is_open()) {
        throw;
    }

    // read whole file
    std::string fileContents {
        std::istreambuf_iterator<char>(inputFile),
        std::istreambuf_iterator<char>()
    };

    inputFile.close();
    return fileContents;
}


int main() {
    auto input { readInput() };

    // split input into two lists
    std::vector<int> list1, list2;
    std::istringstream inputStream { input };
    std::string line;
    while (std::getline(inputStream, line)) {
        // extract integers from line
        std::istringstream lineStream { line };
        int num1, num2;
        lineStream >> num1 >> num2;
        // add to lists
        list1.push_back(num1);
        list2.push_back(num2);
    }

    // sort lists
    std::sort(list1.begin(), list1.end());
    std::sort(list2.begin(), list2.end());

    // loop through lists and compare

    return 0;
}

#endif