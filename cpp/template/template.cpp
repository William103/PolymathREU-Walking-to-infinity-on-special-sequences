/* include little library I made */
#include "../common/util.h"
#include "../common/tree.h"

auto next(ull x) -> std::vector<ull> {
    std::vector<ull> new_xs;
    return new_xs;
}

auto main() -> int {
    /* starts off with a tree with value 0 and no children and searches 20
     * iterations, feel free to tweak to whatever you want
     */
    Tree tree(0, {});
    for (int i = 0; i < 20; i++) {
        tree.step(next);
        print_vec(tree.longest_path());
    }
}
