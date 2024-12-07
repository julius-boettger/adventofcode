#include "util.hpp"
#if DAY == 7 && PUZZLE == 1

#include <vector>
#include <numeric>
#include <sstream>
#include <algorithm>

ulong calculate(const std::vector<uint>& operands, uint operators) {
    ulong result { operands[0] };
    for (auto i { 1uz }; i < operands.size(); i++) {
        auto& operand { operands[i] };
        // there is one operator less than there are operands
        auto operatorIndex { i - 1 };
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

        uint expectedResult;
        lineStream >> expectedResult;

        std::vector<uint> operands;
        uint tempOperand;
        while (lineStream >> tempOperand) {
            operands.push_back(tempOperand);
        }

        // sum
        uint minResult { std::accumulate(operands.begin(), operands.end(), 0u) };
        if (expectedResult == minResult) {
            sumOfExpectedValues += minResult;
            continue;
        }
        // product
        ulong maxResult { std::accumulate(operands.begin(), operands.end(), 1uz, std::multiplies()) };
        if (expectedResult == maxResult) {
            sumOfExpectedValues += maxResult;
            continue;
        }

        if (expectedResult < minResult || expectedResult > maxResult) {
            continue;
        }

        // operators to be used between operands.
        // if bit n of this variable is 1, the n'th operator
        // should be multiplication, otherwise it should be addition.
        // start at 1, because 0 (meaning only addition) was already checked with minResult.
        uint operators { 1 };

        // try every possible combination of operators
        ulong result;
        while ((result = calculate(operands, operators++)) != maxResult) {
            if (result == expectedResult) {
                sumOfExpectedValues += result;
                break;
            }
        }
    }

    std::cout << sumOfExpectedValues << std::endl;
}

#endif