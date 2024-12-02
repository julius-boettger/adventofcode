#include "util.hpp"
#if DAY == 2 && PUZZLE == 2

#include <cmath>
#include <sstream>

// return number of problems in line
static int checkLine(std::string line) {
    auto problems { 0 };

    // extract levels from line
    std::istringstream lineStream { line };

    int previousLevel;
    lineStream >> previousLevel;

    int level;
    lineStream >> level;

    bool ascending { level > previousLevel };

    auto diff { std::abs(level - previousLevel) };
    if (diff > 3 || diff < 1) {
        problems++;
    }

    previousLevel = level;

    while (lineStream >> level) {
        diff = std::abs(level - previousLevel);
        if (diff > 3 ||
            diff < 1 ||
            ascending != (level > previousLevel))
        {
            problems++;
            break; 
        }

        ascending = level > previousLevel;
        previousLevel = level;
    }

    return problems;
}

void solution(std::string input) {
    auto total_safe_reports { 0 };

    // loop through lines
    std::istringstream inputStream { input };
    std::string line;
    while (std::getline(inputStream, line)) {
        auto problems { checkLine(line) };
        if (problems == 0) {
            total_safe_reports++;
        }
    }

    std::cout << total_safe_reports << std::endl;
}

#endif