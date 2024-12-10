#include "util.hpp"
#if DAY == 9 && PUZZLE == 2

#include <vector>
#include <limits>
#include <algorithm>

struct File {
    // id < 0 is free space
    int id {};
    int size {};
};
std::vector<File> files;

auto getFirstSpace(int minSize) {
    const auto predicate { [=](File file) { return file.id < 0 && file.size >= minSize; } };
    const auto iterator { std::find_if(files.begin(), files.end(), predicate) };
    return static_cast<uint>(std::distance(files.begin(), iterator));
}

auto getLastFile(const std::size_t& lastFileToConsider) {
    // use reverse iterators to find the last file
    const auto predicate { [](File file) { return file.id >= 0; } };
    // only search until (including) the element at index lastFileToConsider
    const auto offset { static_cast<std::ptrdiff_t>(files.size() - 1 - lastFileToConsider) };
    const auto iterator { std::find_if(files.rbegin() + offset, files.rend(), predicate) };
    return static_cast<uint>(std::distance(files.begin(), iterator.base()) - 1);
}

void fillSpaceWithFile(uint space, uint file) {
    // if same size: simple swap
    if (files[space].size == files[file].size) {
        std::swap(files[space], files[file]);
        return;
    }

    // copy file
    File tempFile { files[file] };
    // free space of file
    files[file].id = -1;
    // reduce size of space
    files[space].size -= tempFile.size;
    // insert copied file before space
    files.insert(files.begin() + space, tempFile);
}

void solution(std::string input) {
    int id { 0 };
    for (auto i { 0uz }; i < input.size(); i++) {
        const bool isFile { i % 2 == 0 };
        const auto size { static_cast<int>(input.at(i) - '0') };
        
        // add file of given size
        if (isFile) {
            files.push_back(File { .id { id++ }, .size { size }});
        // add free space of given size
        } else {
            files.push_back(File { .id { -1 }, .size { size }});
        }
    }

    // move the the last fitting file into the first space
    // until there are no more fillable spaces between blocks.
    // use this variable to continually ignore files at the end that
    // can't be moved because there is no space big enough for them.
    std::size_t lastFileToConsider { files.size() - 1 };
    while (true) {
        /*for (auto x : files) {
            for (auto i { 0 }; i < x.size; i++) {
                if (x.id < 0)
                    std::cout << ".";
                else
                    std::cout << x.id;
            }
        }
        std::cout << std::endl;*/

        const auto lastFile { getLastFile(lastFileToConsider) };
        const auto firstSpace { getFirstSpace(files[lastFile].size) };

        if (firstSpace > lastFile) {
            if (lastFile == 0) {
                break;
            }

            lastFileToConsider = lastFile - 1;
            continue;
        }

        fillSpaceWithFile(firstSpace, lastFile);
    }

    ulong checksum { 0 };
    ulong blockIndex { 0 };
    for (ulong file { 0 }; file < files.size(); file++) {
        // skip empty spaces
        if (files[file].id < 0) {
            blockIndex += static_cast<ulong>(files[file].size);
            continue;
        }

        for (auto i { 0 }; i < files[file].size; i++) {
            checksum += blockIndex * static_cast<ulong>(files[file].id);
            blockIndex++;
        }
    }

    std::cout << checksum << std::endl;
}

#endif