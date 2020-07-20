import math

from tree import Tree
import util

def next(x):
    new_xs = []
    str_x = str(x)
    for (i,digit) in enumerate(str_x + '0'):
        for d in range(0,10):
            temp = str_x[:i] + str(d) + str_x[i:]
            if not (i == 0 and d == 0):
                if util.is_square(int(temp)):
                    new_xs.append(int(temp))
    return new_xs


def main():
    tree = Tree(0, [1,4,9])
    for i in range(20):
        tree.step(next)
    print(tree)
    print(tree.longest_path())


if __name__ == '__main__':
    main()
