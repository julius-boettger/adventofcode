#include "util.hpp"
#if DAY == 7 && PUZZLE == 2

#include <cmath>
#include <vector>
#include <sstream>
#include <algorithm>

// returns operator (0, 1 or 2) at digit of index of operators in base 3 system
uint getOperator(uint operators, ulong index) {
    // divide by 3^n to isolate digit at index
    const uint divisor { static_cast<uint>(std::pow(3, index)) };
    // extract the digit at index
    return (operators / divisor) % 3;
}

// e.g. 12 + 34 = 1234
void concatenate(ulong& result, const uint& operand) {
    // determine number of digits of operand
    uint operandDigits = (operand == 0) ? 1 : static_cast<uint>(std::log10(operand) + 1);
    // shift result to make room for operand
    result *= static_cast<uint>(std::pow(10, operandDigits));
    // add operand
    result += operand;
}

ulong calculate(const std::vector<uint>& operands, uint operators) {
    ulong result { operands[0] };
    for (auto i { 1uz }; i < operands.size(); i++) {
        const auto& operand { operands[i] };
        // there is one operator less than there are operands
        const auto operatorIndex { i - 1 };

        switch (getOperator(operators, operatorIndex))
        {
        case 0:
            result += operand;
            break;
        case 1:
            result *= operand;
            break;
        case 2:
            concatenate(result, operand);
            break;
        default:
            throw;
            break;
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
        // will be converted to base 3 system, where the three possible
        // options per digit n determine the n'th operator to be used.
        // 0 => addition
        // 1 => multiplication
        // 2 => concatenation
        uint operators { 0 };
        // maximum is operatorCount 2's (concatenation for everything)
        const uint operatorCount { static_cast<uint>(operands.size() - 1) };
        const uint maxOperators { static_cast<uint>(std::pow(3, operatorCount) - 1) };

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