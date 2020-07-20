import math

from tree import Tree
import util

def next(x):
    new_xs = []
    for i in range(10):
        if util.is_prime(10 * x + i):
            new_xs.append(10 * x + i)
    return new_xs


def main():
    tree = Tree(0, [])
    for i in range(20):
        print(tree.longest_path())
        tree.step(next)
    print(tree.longest_path())


if __name__ == '__main__':
    main()
