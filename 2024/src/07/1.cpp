#include "util.hpp"
#if DAY == 7 && PUZZLE == 1

#include <cmath>
#include <vector>
#include <sstream>
#include <algorithm>

ulong calculate(const std::vector<uint>& operands, uint operators) {
    ulong result { operands[0] };
    for (auto i { 1uz }; i < operands.size(); i++) {
        const auto& operand { operands[i] };
        // there is one operator less than there are operands
        const auto operatorIndex { i - 1 };
        // if bit operatorIndex of operators is 1: use multiplication
        if ((operators & (1 << operatorIndex)) != 0) {
            result *= operand;
        // otherwise: use addition
        } else {
            result += operand;
        }
    }
    return result;
}

void solution(std::string input) {
    ulong sumOfExpectedValues { 0 }; 

    std::istringstream inputStream { input };
    std::string line;
    while (std::getline(inputStream, line)) {
        // remove ':' to simplify integer extraction with string streams
        line.erase(std::remove(line.begin(), line.end(), ':'), line.end());
        std::istringstream lineStream { line };

        ulong expectedResult;
        lineStream >> expectedResult;

        std::vector<uint> operands;
        uint tempOperand;
        while (lineStream >> tempOperand) {
            operands.push_back(tempOperand);
        }

        // operators to be used between operands.
        // if bit n of this variable is 1, the n'th operator
        // should be multiplication, otherwise it should be addition.
        uint operators { 0 };
        // maximum is operatorCount 1's (multiplication for everything)
        const uint operatorCount { static_cast<uint>(operands.size() - 1) };
        const uint maxOperators { static_cast<uint>(std::pow(2, operatorCount) - 1) };

        // try every possible combination of operators
        for (; operators <= maxOperators; operators++) {
            const ulong result { calculate(operands, operators) };
            if (result == expectedResult) {
                sumOfExpectedValues += result;
                break;
            }
        }
    }

    std::cout << sumOfExpectedValues << std::endl;
}

#endif