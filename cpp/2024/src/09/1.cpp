#include "util.hpp"

#include <vector>
#include <algorithm>

// x <  0 is free space
// x >= 0 is id of file
std::vector<int> blocks;

auto getFirstSpace() {
    const auto iterator { std::find(blocks.begin(), blocks.end(), -1) };
    return static_cast<uint>(std::distance(blocks.begin(), iterator));
}

auto getLastBlock() {
    // use reverse iterators to find the last block
    const auto predicate { [](int id) { return id >= 0; } };
    const auto iterator { std::find_if(blocks.rbegin(), blocks.rend(), predicate) };
    return static_cast<uint>(std::distance(blocks.begin(), iterator.base()) - 1);
}

void solution(std::string input) {
    int id { 0 };
    for (auto i { 0uz }; i < input.size(); i++) {
        const bool isFile { i % 2 == 0 };
        const auto size { static_cast<std::size_t>(input.at(i) - '0') };
        
        // add file of given size
        if (isFile) {
            blocks.insert(blocks.end(), size, id++);
        // add free space of given size
        } else {
            blocks.insert(blocks.end(), size, -1);
        }
    }

    // swap the first space with the last block
    // until there are no more spaces between blocks
    auto firstSpace { getFirstSpace() };
    auto lastBlock { getLastBlock() };
    while (firstSpace < lastBlock) {
        std::swap(blocks[firstSpace], blocks[lastBlock]);
        firstSpace = getFirstSpace();
        lastBlock = getLastBlock();
    }

    ulong checksum { 0 };
    for (auto i { 0uz }; i < blocks.size(); i++) {
        if (blocks[i] < 0) {
            break;
        }

        checksum += i * static_cast<ulong>(blocks[i]);
    }

    std::cout << checksum << std::endl;
}