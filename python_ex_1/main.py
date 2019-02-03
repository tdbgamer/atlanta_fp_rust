from typing import NamedTuple


class Point(NamedTuple):
    x: int
    y: int

    def __add__(self, other: 'Point') -> 'Point':
        return Point(self.x + other.x,
                     self.y + other.y)


def main():
    first = Point(1, 2)
    second = Point(2, 3)
    print(first)
    print(second)
    print(first + second)


if __name__ == '__main__':
    main()
