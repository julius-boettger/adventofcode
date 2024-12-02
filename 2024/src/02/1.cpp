#include "util.hpp"
#if DAY == 2 && PUZZLE == 1

#include <cmath>
#include <sstream>

void solution(std::string input) {
    auto total_safe_reports { 0 };

    // loop through lines
    std::istringstream inputStream { input };
    std::string line;
    while (std::getline(inputStream, line)) {
        // extract levels from line
        std::istringstream lineStream { line };

        int previousLevel;
        lineStream >> previousLevel;

        int level;
        lineStream >> level;

        bool ascending { level > previousLevel };

        auto diff { std::abs(level - previousLevel) };
        if (diff > 3 || diff < 1) {
            // this line does not contain a safe report
            // check the next one
            continue; 
        }

        previousLevel = level;

        bool safe_report { true };
        while (lineStream >> level) {
            diff = std::abs(level - previousLevel);
            if (diff > 3 ||
                diff < 1 ||
                ascending != (level > previousLevel))
            {
                safe_report = false;
                break; 
            }

            previousLevel = level;
        }

        if (safe_report) {
            total_safe_reports++;
        }
    }

    std::cout << total_safe_reports << std::endl;
}

#endif