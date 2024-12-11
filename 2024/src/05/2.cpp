#include "util.hpp"

#include <regex>
#include <vector>
#include <sstream>
#include <algorithm>

template <typename T>
auto find(const std::vector<T>& vector, T element) {
    return std::find(vector.begin(), vector.end(), element);
}

template <typename T>
bool contains(const std::vector<T>& vector, T element) {
    return find(vector, element) != vector.end();
}

struct UpdateRule {
    int before {};
    int after {};

    bool isCompliantWith(const std::vector<int>& update) const {
        const auto beforePosition { find(update, this->before) };
        const auto  afterPosition { find(update, this->after ) };
        // compliant if at least one number is not in the update
        if (beforePosition == update.end() || afterPosition == update.end()) {
            return true;
        }
        return afterPosition > beforePosition;
    }

    bool isRelevantFor(const std::vector<int>& update) const {
        return contains(update, this->before) && contains(update, this->after);
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

std::vector<int> getSortedUpdate(const std::vector<int>& update, const std::vector<UpdateRule>& updateRules) {
    std::vector<UpdateRule> relevantUpdateRules {};
    for (auto& updateRule : updateRules) {
        if (updateRule.isRelevantFor(update)) {
            relevantUpdateRules.push_back(updateRule);
        }
    }

    std::vector<int> sortedUpdate {
        relevantUpdateRules[0].before,
        relevantUpdateRules[0].after
    };
    for (auto i { 1uz }; i < relevantUpdateRules.size(); i++) {
        const auto updateRule { relevantUpdateRules[i] };
        const auto containsBefore { contains(sortedUpdate, updateRule.before) };
        const auto containsAfter  { contains(sortedUpdate, updateRule.after ) };

        // if both numbers are already in the update
        if (containsBefore && containsAfter) {
            // swap them if they are in the wrong order
            const auto beforePosition { std::find(sortedUpdate.begin(), sortedUpdate.end(), updateRule.before) };
            const auto  afterPosition { std::find(sortedUpdate.begin(), sortedUpdate.end(), updateRule.after ) };
            if (beforePosition > afterPosition) {
                std::iter_swap(beforePosition, afterPosition);
            }
        // if at least one number is not in the update yet
        } else {
            // insert updateRule.before at start if not in update
            if (!containsBefore) {
                sortedUpdate.insert(sortedUpdate.begin(), updateRule.before);
            }
            // insert updateRule.after at end if not in update
            if (!containsAfter) {
                sortedUpdate.push_back(updateRule.after);
            }
        }
    }

    // swap numbers of not compliant update rules until compliant
    while (!isCompliant(sortedUpdate, relevantUpdateRules)) {
        for (auto& updateRule : relevantUpdateRules) {
            if (!updateRule.isCompliantWith(sortedUpdate)) {
                const auto beforePosition { std::find(sortedUpdate.begin(), sortedUpdate.end(), updateRule.before) };
                const auto  afterPosition { std::find(sortedUpdate.begin(), sortedUpdate.end(), updateRule.after ) };
                std::iter_swap(beforePosition, afterPosition);
            }
        }
    }

    return sortedUpdate;
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
        if (!isCompliant(update, updateRules)) {
            auto sortedUpdate { getSortedUpdate(update, updateRules) };
            middleNumberSum += getMiddleNumber(sortedUpdate);
        }
    }

    std::cout << middleNumberSum << std::endl;
}