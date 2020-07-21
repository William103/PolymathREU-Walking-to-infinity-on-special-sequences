/* include little library I made */
#include "../common/util.h"
#include "../common/tree.h"
#include <string>

auto next(ull x) -> std::vector<ull> {
    std::vector<ull> new_xs;
    ull temp;
    std::string str_x = std::to_string(x);
    for (int i = 0; i < str_x.length(); i++) {
        for (int d = 0; d < 10; d++) {
            if (!(i == 0 && d == 0)) {
                str_x = std::to_string(x);
                str_x.insert(i, std::to_string(d));
                /* std::cout << x << ' ' << str_x << ' ' << i << ' ' << d << '\n'; */
                temp = std::stoull(str_x);
                if (is_prime(temp)) {
                    new_xs.push_back(temp);
                }
            }
        }
    }
    return new_xs;
}

auto main() -> int {
    /* starts off with a tree with value 0 and no children and searches 20
     * iterations, feel free to tweak to whatever you want
     */
    Tree tree(0, {2, 3, 5, 7});
    for (int i = 0; i < 20; i++) {
        tree.step(next);
        print_vec(tree.longest_path());
    }
}
