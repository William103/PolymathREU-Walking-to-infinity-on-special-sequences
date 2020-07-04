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
    str_x = str(x)
    for (i,digit) in enumerate(str_x + '0'):
        for d in range(0,10):
            temp = str_x[:i] + str(d) + str_x[i:]
            if not (i == 0 and d == 0):
                if is_prime(int(temp)):
                    new_xs.append(int(temp))
    return new_xs


def main():
    tree = Tree(0, [2, 3, 5, 7])
    for i in range(20):
        print(tree.longest_path())
        tree.step()
    print(tree.longest_path())


if __name__ == '__main__':
    main()
