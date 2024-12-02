#include "util.hpp"
#if DAY == 1 && PUZZLE == 2

#include <cmath>
#include <vector>
#include <sstream>
#include <algorithm>

void solution(std::string input) {
    // split input into two lists
    std::vector<int> list1, list2;
    std::istringstream inputStream { input };
    std::string line;
    while (std::getline(inputStream, line)) {
        // extract integers from line
        std::istringstream lineStream { line };
        int num1, num2;
        lineStream >> num1 >> num2;
        // add to lists
        list1.push_back(num1);
        list2.push_back(num2);
    }

    // loop through list1 and calculate total similarity
    auto total_similarity { 0z };
    for (auto& num : list1) {
        auto occurences { std::count(list2.begin(), list2.end(), num) };
        total_similarity += num * occurences;
    }

    std::cout << total_similarity << std::endl;
}

#endif