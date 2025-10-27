#include "util.hpp"

#include <regex>
#include <vector>
#include <sstream>
#include <algorithm>

struct UpdateRule {
    int before {};
    int after {};

    bool isCompliantWith(const std::vector<int>& update) const {
        auto beforePosition { std::find(update.begin(), update.end(), this->before) };
        auto  afterPosition { std::find(update.begin(), update.end(), this->after ) };
        // compliant if at least one number is not in the update
        if (beforePosition == update.end() || afterPosition == update.end()) {
            return true;
        }
        return afterPosition > beforePosition;
    }
};

int getMiddleNumber(const std::vector<int>& update) {
    return update[(update.size() - 1) / 2];
}

bool isCompliant(const std::vector<int>& update, const std::vector<UpdateRule>& updateRules) {
    for (auto& updateRule : updateRules) {
        if (!updateRule.isCompliantWith(update)) {
            return false;
        }
    }
    return true;
}

void solution(std::string input) {
    auto middleNumberSum { 0 };

    std::vector<std::vector<int>> updates {};
    std::vector<UpdateRule> updateRules {};

    // loop through input lines
    std::istringstream inputStream { input };
    std::string line;
    while (std::getline(inputStream, line)) {
        if (line.find('|') != std::string::npos) {
            const std::regex pattern { "(\\d+)\\|(\\d+)" };
            std::smatch match;
            std::regex_search(line, match, pattern);
            updateRules.push_back(UpdateRule {
                .before { std::stoi(match[1].str()) },
                .after  { std::stoi(match[2].str()) }
            });
        } else if (!line.empty()) {
            std::vector<int> update;
            std::string token;
            std::stringstream lineStream { line };
            while (std::getline(lineStream, token, ',')) {
                update.push_back(std::stoi(token));
            }
            updates.push_back(update);
        }
    }

    for (auto& update : updates) {
        if (isCompliant(update, updateRules)) {
            middleNumberSum += getMiddleNumber(update);
        }
    }

    std::cout << middleNumberSum << std::endl;
}