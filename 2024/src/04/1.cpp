#include "util.hpp"

#include <regex>
#include <vector>
#include <sstream>

std::ptrdiff_t wordsCounted { 0 };

void checkLine(const std::string& line) {
    const std::vector<std::regex> patterns {
        std::regex { "XMAS" },
        std::regex { "SAMX" }
    };
    for (auto& pattern : patterns) {
        wordsCounted += std::distance(
            std::sregex_iterator(line.begin(), line.end(), pattern),
            std::sregex_iterator()
        );
    }
}

void solution(std::string input) {
    constexpr auto MIN_WORD_LENGTH { 4 };
    std::vector<std::vector<char>> grid {};

    // loop through input lines
    std::istringstream inputStream { input };
    std::string inputLine;
    while (std::getline(inputStream, inputLine)) {
        grid.push_back(std::vector<char> { inputLine.begin(), inputLine.end() });
        // check rows
        checkLine(inputLine);
    }

    const auto ROWS { grid.size() };
    const auto COLS { grid[0].size() };

    // check columns
    for (auto col { 0uz }; col < COLS; col++) {
        std::string line {};
        for (auto row { 0uz }; row < ROWS; row++) {
            line += grid[row][col];
        }
        checkLine(line);
    }

    //// check diagonals from top left to bottom right
    // starting from top row
    for (auto col { 0uz }; col <= COLS - MIN_WORD_LENGTH; col++) {
        std::string line {};
        for (auto offset { 0uz }; col + offset < COLS; offset++) {
            line += grid[offset][col + offset];
        }
        checkLine(line);
    }
    // starting from left column
    for (auto row { 1uz }; row <= ROWS - MIN_WORD_LENGTH; row++) {
        std::string line {};
        for (auto offset { 0uz }; row + offset < ROWS; offset++) {
            line += grid[row + offset][offset];
        }
        checkLine(line);
    }

    //// check diagonals from top right to bottom left
    // starting from top row
    for (auto col { MIN_WORD_LENGTH - 1 }; col < static_cast<int>(COLS); col++) {
        std::string line {};
        for (auto offset { 0 }; col - offset >= 0; offset++) {
            line += grid[static_cast<uint>(offset)][static_cast<uint>(col - offset)];
        }
        checkLine(line);
    }
    // starting from right column
    for (auto row { 1 }; row <= static_cast<int>(ROWS - MIN_WORD_LENGTH); row++) {
        std::string line {};
        for (auto offset { 0 }; row + offset < static_cast<int>(ROWS); offset++) {
            line += grid[static_cast<uint>(row + offset)][static_cast<uint>(static_cast<int>(COLS) - 1 - offset)];
        }
        checkLine(line);
    }

    std::cout << wordsCounted << std::endl;
}