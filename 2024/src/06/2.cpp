#include "util.hpp"
#if DAY == 6 && PUZZLE == 2

#include <vector>
#include <sstream>

std::vector<std::vector<char>> grid {};
int rows {};
int cols {};

struct Direction {
    int row {};
    int col {};
    void rotateRight();
    bool operator==(const Direction&) const = default;
};

constexpr Direction UP    { .row { -1 }, .col {  0 } };
constexpr Direction RIGHT { .row {  0 }, .col {  1 } };
constexpr Direction DOWN  { .row {  1 }, .col {  0 } };
constexpr Direction LEFT  { .row {  0 }, .col { -1 } };

void Direction::rotateRight() {
         if (*this == UP   ) *this = RIGHT;
    else if (*this == RIGHT) *this = DOWN ;
    else if (*this == DOWN ) *this = LEFT ;
    else if (*this == LEFT ) *this = UP   ;
}

struct Position {
    int row {};
    int col {};

    bool isInBounds() const {
        return
            row >= 0 &&
            col >= 0 &&
            row < rows &&  
            col < cols;
    }

    char getChar() const {
        return grid[static_cast<uint>(row)][static_cast<uint>(col)];
    }

    void setChar(char c) const {
        grid[static_cast<uint>(row)][static_cast<uint>(col)] = c;
    }

    Position operator+(const Direction& direction) const {
        return {
            .row { this->row + direction.row },
            .col { this->col + direction.col }
        };
    }

    Position& operator+=(const Direction& direction) {
        this->row += direction.row;
        this->col += direction.col;
        return *this;
    }

    static Position getStart() {
        for (auto row { 0 }; row < rows; row++) {
            for (auto col { 0 }; col < cols; col++) {
                if (grid[static_cast<uint>(row)][static_cast<uint>(col)] == '^') {
                    return {
                        .row { row },
                        .col { col }
                    };
                }
            }
        }
        return {};
    }
};

bool loopObstructionPossible(Position pos, Direction direction) {
    // look to right
    direction.rotateRight();
    // search line of sight for for 'X' followed by '#'
    Position nextPos;
    while ((nextPos = pos + direction).isInBounds()) {
        if (pos.getChar() == 'X' && nextPos.getChar() == '#') {
            return true;
        }
        pos += direction;
    }
    return false;
}

void solution(std::string input) {
    // build grid from input
    std::istringstream inputStream { input };
    std::string line;
    while (std::getline(inputStream, line)) {
        grid.push_back(std::vector<char> { line.begin(), line.end() });
    }
    rows = static_cast<int>(grid.size());
    cols = static_cast<int>(grid[0].size());

    // number of possible positions for obstructions
    // that would cause the guard to walk in a loop
    auto loopObstructions { 0 };

    Position pos { Position::getStart() };
    Direction direction { UP };
    while (true) {
        pos.setChar('X'); // "i have been here"

        auto nextPos { pos + direction };
        if (!nextPos.isInBounds()) {
            break;
        }

        if (loopObstructionPossible(pos, direction)) {
            loopObstructions++;
        }

        if (nextPos.getChar() == '#') {
            direction.rotateRight();
        }

        pos += direction;
    }

    std::cout << loopObstructions << std::endl;
}

#endif