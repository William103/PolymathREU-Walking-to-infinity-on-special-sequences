import math

# some utility functions, same as above
def is_square(x):
    return int(math.sqrt(x)) == math.sqrt(x)

primes = {}

def is_prime(x):
    try:
        return primes[x]
    except KeyError:
        if x < 2:
            primes[x] = False
            return False
        i = 2
        while i * i <= x:
            if x % i == 0:
                primes[x] = False
                return False
            i += 1
        primes[x] = True
        return True
