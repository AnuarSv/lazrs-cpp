#include <iostream>
#include <chrono>
#include "fastlaz.h"

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cerr << "usage: " << argv[0] << " <file.las|file.laz>" << std::endl;
        return 1;
    }

    uint64_t counts[256] = {0};
    uint64_t total = 0;

    auto t0 = std::chrono::steady_clock::now();
    int32_t rc = fastlaz_read_and_count(argv[1], counts, &total);
    auto t1 = std::chrono::steady_clock::now();

    if (rc != 0) {
        std::cerr << "failed to read file" << std::endl;
        return 1;
    }

    std::cout << "total points: " << total << std::endl;
    for (int c = 0; c < 256; ++c) {
        if (counts[c] > 0) {
            std::cout << "class " << c << ": " << counts[c] << " points" << std::endl;
        }
    }

    std::chrono::duration<double> elapsed = t1 - t0;
    std::cerr << "elapsed: " << elapsed.count() << " s" << std::endl;

    return 0;
}
