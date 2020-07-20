#include <vector>

using ull = unsigned long long;

/* all tree stuff, don't really need to change it unless you have some ideas for
 * optimizations 
 */
class Tree
{
public:
    ull value;
    std::vector<Tree> children;

    Tree(ull value, std::vector<ull> tree_children);
    void step(std::vector<ull> (*next)(ull));
    auto longest_path() -> std::vector<ull>;
};
