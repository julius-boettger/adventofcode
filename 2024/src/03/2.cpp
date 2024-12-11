#include "util.hpp"

#include <regex>

void solution(std::string input) {
    int sum { 0 };
    // switch when encountering a do() or don't()
    bool active { true };

    // match all valid instructions
    const std::regex pattern { "do\\(\\)|don't\\(\\)|mul\\((\\d{1,3}),(\\d{1,3})\\)" };
    const auto begin { std::sregex_iterator(input.begin(), input.end(), pattern) };
    const auto end { std::sregex_iterator() };
    for (std::sregex_iterator i { begin }; i != end; i++) {
        if (active) {
            if (i->str() == "don't()") {
                active = false;
            } else if (i->str() != "do()") {
                // extract numbers from capture groups
                auto num1 { std::stoi((*i)[1]) };
                auto num2 { std::stoi((*i)[2]) };
                sum += num1 * num2;
            }
        } else if (i->str() == "do()") {
            active = true;
        }
    }

    std::cout << sum << std::endl;
}