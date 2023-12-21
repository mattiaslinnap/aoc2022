#!/usr/bin/env python3

nodes = {}


class Tree:
    def __init__(self, name, val_or_op, left=None, right=None):
        self.name = name
        if isinstance(val_or_op, int):
            self.val = val_or_op
            self.op = None
        else:
            self.val = None
            self.op = val_or_op
        self.left = left
        self.right = right

    def print(self, ident=0):
        print(f'{self.name}: {self.val if self.val else self.op}')
        if self.val is None:
            self.left.print(ident=ident + 2)
            self.right.print(ident=ident + 2)

    def build_references(self):
        if isinstance(self.left, str):
            self.left = nodes[self.left]
        if isinstance(self.right, str):
            self.right = nodes[self.right]


def main():
    with open('input/day21/test.txt') as f:
        for line in f:
            line = line.strip()
            if line:
                name, rest = line.split(': ')
                if name == 'humn':
                    nodes[name] = Tree(name, '?')
                else:
                    try:
                        nodes[name] = Tree(name, int(rest))
                    except ValueError:
                        left, op, right = rest.split(' ')
                        nodes[name] = Tree(name, op, left, right)

    # Link up references
    root = nodes['root']
    root.build_references()

    root.print()
    # print(f"ANSWER {numbers['root']}")


def calc(left, op, right):
    if left == 'humn' or right == 'humn':
        return None

    if op == '+':
        return numbers[left] + numbers[right]
    elif op == '-':
        return numbers[left] - numbers[right]
    elif op == '*':
        return numbers[left] * numbers[right]
    elif op == '/':
        return numbers[left] // numbers[right]
    else:
        raise RuntimeError(f'Unknown operator {op}')


def read_lines(path):
    lines = []

    return lines


if __name__ == '__main__':
    main()
