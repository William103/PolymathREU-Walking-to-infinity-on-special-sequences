#include "../common/util.h"
#include "../common/tree.h"

#include <cstdlib>

auto next(ull x) -> std::vector<ull> {
    std::vector<ull> new_xs;
    ull temp = x * 10;
    for (int i = 0; i < 10; i++) {
        auto temp2 = temp + i;
        if (is_prime(temp2)) {
            new_xs.push_back(temp2);
        }
    }
    return new_xs;
}

auto main(int argc, char *argv[]) -> int {
    if (argc != 2) {
        std::cout << "Usage: append_digit_primes <search_depth>\n";
        return -1;
    }
    Tree tree(0, {});
    for (int i = 0; i < std::atoi(argv[1]); i++) {
        tree.step(next);
        print_vec(tree.longest_path());
    }
}
