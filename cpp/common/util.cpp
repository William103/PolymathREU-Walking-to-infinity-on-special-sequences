#include "util.h"

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
    for (int i = 2; i * i <= n; i+=2) {
        if (n % i == 0) {
            primes[n] = false;
            return false;
        }
        if (i == 2) i--;
    }
    primes[n] = true;
    return true;
}

auto print_vec(std::vector<ull> vec) -> void {
    std::cout << '[';
    for (ull val : vec) {
        std::cout << val << ", ";
    }
    std::cout << "]\n";
}
