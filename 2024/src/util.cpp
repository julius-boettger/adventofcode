#include "util.hpp"

#include <format>
#include <fstream>

static std::string readInput() {
    std::ifstream inputFile { std::format("input/{:02}/{}.txt", DAY, PUZZLE) };
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
    solution(readInput());
    return 0;
}