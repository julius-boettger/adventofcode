#include "util.hpp"
#if DAY == 8 && PUZZLE == 2

#include <vector>
#include <sstream>
#include <algorithm>

std::vector<std::vector<char>> antennaGrid {};
std::vector<std::vector<char>> antinodeGrid {};
uint rows {};
uint cols {};

template <typename T>
bool contains(const std::vector<T>& vector, T element) {
    return std::find(vector.begin(), vector.end(), element) != vector.end();
}

struct Coord {
    int row {};
    int col {};

    char getAntenna() const {
        return antennaGrid[static_cast<uint>(this->row)][static_cast<uint>(this->col)];
    }

    void setAntinode() const {
        antinodeGrid[static_cast<uint>(this->row)][static_cast<uint>(this->col)] = '#';
    }

    bool isInBounds() const {
        return
            this->row >= 0 &&
            this->col >= 0 &&
            row < static_cast<int>(rows) &&  
            col < static_cast<int>(cols);
    }

    Coord operator+(const Coord& other) const {
        return {
            .row { this->row + other.row },
            .col { this->col + other.col }
        };
    }

    Coord operator-(const Coord& other) const {
        return {
            .row { this->row - other.row },
            .col { this->col - other.col }
        };
    }

    friend std::ostream& operator<<(std::ostream& stream, const Coord& coord) {
        stream << "[" << coord.row << "][" << coord.col << "]";
        return stream; // for chaining
    }
};

void markAntinodes(Coord coord1, Coord coord2) {
    Coord antinode;

    coord1.setAntinode();
    antinode = (coord1 - coord2) + coord1; 
    while (antinode.isInBounds()) {
        antinode.setAntinode();
        antinode = (coord1 - coord2) + antinode;
    }

    coord2.setAntinode();
    antinode = (coord2 - coord1) + coord2; 
    while (antinode.isInBounds()) {
        antinode.setAntinode();
        antinode = (coord2 - coord1) + antinode;
    }
}

void solution(std::string input) {
    // build antenna grid from input
    std::istringstream inputStream { input };
    std::string line;
    while (std::getline(inputStream, line)) {
        antennaGrid.push_back(std::vector<char> { line.begin(), line.end() });
    }
    rows = static_cast<uint>(antennaGrid.size());
    cols = static_cast<uint>(antennaGrid[0].size());

    // initialize empty antinode grid for output
    antinodeGrid = std::vector(rows, std::vector(cols, static_cast<char>(0)));

    // determine present antenna types
    std::vector<char> antennaTypes {};
    for (auto& row : antennaGrid) {
        for (auto c : row) {
            if (c != '.' && !contains(antennaTypes, c)) {
                antennaTypes.push_back(c);
            }
        }
    }

    for (auto antennaType : antennaTypes) {
        // determine coordinates of antennas
        std::vector<Coord> coords {};
        for (auto row { 0uz }; row < rows; row++) {
            for (auto col { 0uz }; col < cols; col++) {
                if (antennaGrid[row][col] == antennaType) {
                    coords.push_back({
                        .row { static_cast<int>(row) },
                        .col { static_cast<int>(col) }
                    });
                }
            }
        }

        if (coords.size() < 2) {
            continue;
        }

        // loop through all combinations of antenna coords to mark their antinodes
        for (auto i { 0uz }; i < coords.size(); i++) {
            for (auto j { i + 1 }; j < coords.size(); j++) {
                markAntinodes(coords[i], coords[j]);
            }
        }
    }

    // count marked antinodes
    auto antinodes { 0 };
    for (auto& row : antinodeGrid) {
        for (auto& c : row) {
            if (c == '#') {
                antinodes++;
            }
        }
    }

    std::cout << antinodes << std::endl;
}

#endif