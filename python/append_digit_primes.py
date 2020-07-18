import math

class Tree:
    def __init__(self, value, children):
        self.value = value
        self.children = []
        for child in children:
            self.children.append(Tree(child, []))

    def step(self):
        if len(self.children) == 0:
            for val in step(self.value):
                self.children.append(Tree(val, []))
            return
        for child in self.children:
            child.step()

    def __str__(self):
        retval = '[' + str(self.value) + ': '
        for child in self.children:
            retval += str(child)
        retval += ']'
        return retval

    def __repr__(self):
        return str(self)

    def longest_path(self):
        if len(self.children) == 0:
            return [self.value]
        max_path = []
        max_length = 0
        for child in self.children:
            temp = child.longest_path()
            if len(temp) > max_length:
                max_path = temp
                max_length = len(temp)
        return [self.value] + max_path

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

def step(x):
    new_xs = []
    for i in range(10):
        if is_prime(10 * x + i):
            new_xs.append(10 * x + i)
    return new_xs


def main():
    tree = Tree(0, [])
    for i in range(9):
        print(tree.longest_path())
        tree.step()
    print(tree.longest_path())


if __name__ == '__main__':
    main()
