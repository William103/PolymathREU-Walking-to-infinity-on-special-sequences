import math

# these are the tree and utility libraries I created
from tree import Tree
import util

# TODO: tweak this to do whatever you want as I described in the Overleaf
def next(x):
    return []


# here we create a tree with value 0 and no children and check it for 20
# iterations, but feel free to change that up
def main():
    tree = Tree(0, [])
    for i in range(20):
        print(tree.longest_path())
        tree.step(next)
    print(tree.longest_path())


if __name__ == '__main__':
    main()
