import math

# tree stuff, I wouldn't recommend tweaking unless you have serious ideas for
# optimizations
class Tree:
    def __init__(self, value, children):
        self.value = value
        self.children = []
        for child in children:
            self.children.append(Tree(child, []))

    def step(self, next):
        if len(self.children) == 0:
            for val in next(self.value):
                self.children.append(Tree(val, []))
            return
        for child in self.children:
            child.step(next)

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
