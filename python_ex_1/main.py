from typing import NamedTuple


class Vector(NamedTuple):
    x: int
    y: int

    def __add__(self, other: 'Vector') -> 'Vector':
        return Vector(self.x + other.x,
                      self.y + other.y)


def main():
    first = Vector(1, 2)
    second = Vector(2, 3)
    print(first)
    print(second)
    print(first + second)


if __name__ == '__main__':
    main()
