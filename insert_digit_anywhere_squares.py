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
        #return str(self.value) + ': ' + str([str(child) for child in self.children])

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

def step(x):
    new_xs = []
    str_x = str(x)
    for (i,digit) in enumerate(str_x + '0'):
        for d in range(0,10):
            temp = str_x[:i] + str(d) + str_x[i:]
            if not (i == 0 and d == 0):
                if is_square(int(temp)):
                    new_xs.append(int(temp))
    return new_xs


def main():
    tree = Tree(0, [1,4,9])
    for i in range(200):
        tree.step()
    print(tree)
    print(tree.longest_path())


if __name__ == '__main__':
    main()
