#include "util.hpp"

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

    // sort lists
    std::sort(list1.begin(), list1.end());
    std::sort(list2.begin(), list2.end());

    // loop through lists and calculate total distance
    auto total_distance { 0 };
    for (auto i { 0uz }; i < list1.size(); i++) {
        total_distance += std::abs(list1[i] - list2[i]);
    }

    std::cout << total_distance << std::endl;
}