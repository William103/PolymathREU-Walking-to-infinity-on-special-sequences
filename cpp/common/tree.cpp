#include "tree.h"

/* all tree stuff, don't really need to change it unless you have some ideas for
 * optimizations 
 */
Tree::Tree(ull value, std::vector<ull> tree_children) : value(value) {
    for (ull child : tree_children) {
        children.push_back(Tree(child, {}));
    }
}

void Tree::step(std::vector<ull> (*next)(ull)) {
    if (children.size() == 0) {
        auto xs = next(value);
        for (auto val : xs) {
            children.push_back(Tree(val, {}));
        }
    } else {
        for (auto &child : children) {
            child.step(next);
        }
    }
}

auto Tree::longest_path() -> std::vector<ull> {
    if (children.size() == 0) {
        return { value };
    }
    std::vector<ull> max_path;
    int max_length = 0;
    for (auto &child : children) {
        auto temp = child.longest_path();
        if (temp.size() > max_length) {
            max_length = temp.size();
            max_path = temp;
        }
    }
    std::vector<ull> retval = { value };
    retval.insert(retval.end(), max_path.begin(), max_path.end());
    return retval;
}
