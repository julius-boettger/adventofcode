#include "util.hpp"
#if DAY == 3 && PUZZLE == 1

#include <regex>

void solution(std::string input) {
    int sum { 0 };

    // match all valid instructions
    const std::regex pattern { "mul\\((\\d{1,3}),(\\d{1,3})\\)" };
    const auto begin { std::sregex_iterator(input.begin(), input.end(), pattern) };
    const auto end { std::sregex_iterator() };
    for (std::sregex_iterator i { begin }; i != end; i++) {
        // extract numbers from capture groups
        auto num1 { std::stoi((*i)[1]) };
        auto num2 { std::stoi((*i)[2]) };
        sum += num1 * num2;
    }

    std::cout << sum << std::endl;
}

#endif