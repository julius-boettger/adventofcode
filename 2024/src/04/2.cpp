#include "util.hpp"
#if DAY == 4 && PUZZLE == 2

#include <regex>
#include <vector>
#include <sstream>

std::vector<std::vector<char>> grid {};

char get(int row, int col) {
    return grid[static_cast<uint>(row)][static_cast<uint>(col)];
}

bool SorM(int row, int col) {
    return get(row, col) == 'S' || get(row, col) == 'M';
}

// assumes SorM(row1, col1) is true
bool opposite(int row1, int col1, int row2, int col2) {
    return get(row2, col2) == (
        get(row1, col1) == 'S' ? 'M' : 'S'
    );
}

void solution(std::string input) {
    auto wordsCounted { 0 };

    // loop through input lines
    std::istringstream inputStream { input };
    std::string inputLine;
    while (std::getline(inputStream, inputLine)) {
        grid.push_back(std::vector<char> { inputLine.begin(), inputLine.end() });
    }

    const auto ROWS { static_cast<int>(grid.size()) };
    const auto COLS { static_cast<int>(grid[0].size()) };

    for (auto row { 1 }; row < ROWS - 1; row++) {
        for (auto col { 1 }; col < COLS - 1; col++) {
            if (get(row, col) == 'A' &&
                SorM(row - 1, col - 1) &&
                SorM(row - 1, col + 1) &&
                opposite(row - 1, col - 1, row + 1, col + 1) &&
                opposite(row - 1, col + 1, row + 1, col - 1))
            {
                wordsCounted++;
            }
        }
    }

    std::cout << wordsCounted << std::endl;
}

#endif