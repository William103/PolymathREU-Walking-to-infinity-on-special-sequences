#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>

using ull = unsigned long long;

auto next(ull x) -> std::vector<ull>;
auto is_prime(ull n) -> bool;

/* all tree stuff, don't really need to change it unless you have some ideas for
 * optimizations 
 */
class Tree
{
public:
    ull value;
    std::vector<Tree> children;

    Tree(ull value, std::vector<ull> tree_children) : value(value) {
        for (ull child : tree_children) {
            children.push_back(Tree(child, {}));
        }
    }

    void step() {
        if (children.size() == 0) {
            auto xs = next(value);
            for (auto val : xs) {
                children.push_back(Tree(val, {}));
            }
        } else {
            for (auto &child : children) {
                child.step();
            }
        }
    }

    auto longest_path() -> std::vector<ull> {
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
};

/* TODO Tweak this to change functionality according to explanation on the
 * Overleaf 
 */
auto next(ull x) -> std::vector<ull> {
    std::vector<ull> new_xs;
    return new_xs;
}

/* some helper functions */
auto is_prime(ull n) -> bool {
    static std::unordered_map<ull, bool> primes;
    if (n < 2) { 
        primes[n] = false;
        return false;
    }
    if (primes.count(n) > 0) {
        return primes[n];
    }
    for (int i = 2; i * i <= n; i++) {
        if (n % i == 0) {
            primes[n] = false;
            return false;
        }
    }
    primes[n] = true;
    return true;
}

template <typename T>
auto print_vec(std::vector<T> vec) -> void {
    std::cout << '[';
    for (T &val : vec) {
        std::cout << val << ", ";
    }
    std::cout << "]\n";
}

auto main() -> int {
    /* starts off with a tree with value 0 and no children and searches 20
     * iterations, feel free to tweak to whatever you want
     */
    Tree tree(0, {});
    for (int i = 0; i < 20; i++) {
        tree.step();
        print_vec(tree.longest_path());
    }
}
