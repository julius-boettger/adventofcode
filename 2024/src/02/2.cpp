#include "util.hpp"
#if DAY == 2 && PUZZLE == 2

#include <cmath>
#include <vector>
#include <sstream>

// return number of problems in levels
static int checkLevels(const std::vector<int>& levels) {
    auto problems { 0 };

    int previousLevel { levels[0] };
    int level { levels[1] };
    bool ascending { level > previousLevel };

    auto diff { std::abs(level - previousLevel) };
    if (diff > 3 || diff < 1) {
        problems++;
    }

    previousLevel = level;

    for (auto i { 2uz }; i < levels.size(); i++) {
        level = levels[i];
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
        // extract levels from line
        std::vector<int> levels {};
        std::istringstream lineStream { line };
        int level;
        while (lineStream >> level) {
            levels.push_back(level);
        }

        auto problems { checkLevels(levels) };
        if (problems == 0) {
            total_safe_reports++;
        } else if (problems <= 2) {
            // try if it's safe when removing an element
            // (not optimal, the number of loop iterations could be reduced)
            for (auto i { 0uz }; i < levels.size(); i++) {
                auto modified_levels { levels };
                modified_levels.erase(modified_levels.begin() + static_cast<int>(i));
                problems = checkLevels(modified_levels);
                if (problems == 0) {
                    total_safe_reports++;
                    break;
                }
            }
        }
    }

    std::cout << total_safe_reports << std::endl;
}

#endif