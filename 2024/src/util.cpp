#include "util.hpp"

#include <format>
#include <fstream>

static std::string readInput() {
    // INPUT_PATH is set by meson.build
    std::ifstream inputFile { INPUT_PATH };
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
    // solution to run is managed by meson.build
    solution(readInput());
    return 0;
}