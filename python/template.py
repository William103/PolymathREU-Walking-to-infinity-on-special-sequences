import math

# tree stuff, I wouldn't recommend tweaking unless you have serious ideas for
# optimizations
class Tree:
    def __init__(self, value, children):
        self.value = value
        self.children = []
        for child in children:
            self.children.append(Tree(child, []))

    def step(self):
        if len(self.children) == 0:
            for val in next(self.value):
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

# TODO: tweak this to do whatever you want as I described in the Overleaf
def next(x):
    return []


# here we create a tree with value 0 and no children and check it for 20
# iterations, but feel free to change that up
def main():
    tree = Tree(0, [])
    for i in range(20):
        print(tree.longest_path())
        tree.step()
    print(tree.longest_path())


if __name__ == '__main__':
    main()
