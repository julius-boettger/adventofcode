#include "util.hpp"

#include <set>
#include <array>
#include <vector>
#include <sstream>
#include <cstdint>

std::vector<std::vector<int8_t>> heightMap {};
int8_t rows {};
int8_t cols {};

struct Coord {
    int8_t row {};
    int8_t col {};

    int8_t getHeight() const {
        return heightMap[static_cast<size_t>(this->row)][static_cast<size_t>(this->col)];
    }

    bool isInBounds() const {
        return
            this->row >= 0 &&
            this->col >= 0 &&
            row < rows &&  
            col < cols;
    }

    Coord operator+(const Coord& other) const {
        return {
            static_cast<int8_t>(this->row + other.row),
            static_cast<int8_t>(this->col + other.col)
        };
    }

    // required for std::set
    bool operator<(const Coord& other) const {
        return (this->row < other.row) || (this->row == other.row && this->col < other.col);
    }
};

constexpr std::array<Coord, 4> DIRECTIONS {{
    { -1,  0 }, // up
    {  0,  1 }, // right
    {  1,  0 }, // down
    {  0, -1 }, // left
}};

void getTrailEnds(const Coord& coord, std::set<Coord>& trailEnds) {
    const auto height { coord.getHeight() };
    if (height == 9) {
        trailEnds.insert(coord);
        return;
    }

    for (auto& direction : DIRECTIONS) {
        const auto nextCoord { coord + direction };
        if (!nextCoord.isInBounds()) {
            continue;
        }

        const auto nextHeight { nextCoord.getHeight() };
        if (nextHeight == height + 1) {
            // continue searching from that position
            getTrailEnds(nextCoord, trailEnds);
        }
    }
}

int32_t getScore(const Coord& start) {
    std::set<Coord> trailEnds {};
    getTrailEnds(start, trailEnds);
    return static_cast<int32_t>(trailEnds.size());
}

void solution(std::string input) {
    std::istringstream inputStream { input };
    std::string line;
    while (std::getline(inputStream, line)) {
        std::vector<int8_t> row;
        for (auto c : line) {
            row.push_back(c - '0');
        }
        heightMap.push_back(row);
    }
    rows = static_cast<int8_t>(heightMap.size());
    cols = static_cast<int8_t>(heightMap[0].size());

    int32_t totalScore { 0 };
    for (int8_t row { 0 }; row < rows; row++) {
        for (int8_t col { 0 }; col < cols; col++) {
            const auto height { heightMap[static_cast<size_t>(row)][static_cast<size_t>(col)] };
            if (height == 0) {
                totalScore += getScore({ row, col });
            }
        }
    }

    std::cout << totalScore << std::endl;
}