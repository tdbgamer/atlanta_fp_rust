from typing import NamedTuple


class Person(NamedTuple):
    age: int


def main():
    people = [Person(23),
              Person(19),
              Person(42),
              Person(17),
              Person(17),
              Person(31),
              Person(30)]

    over_30 = [person for person in people if person.age > 30]

    num_over_30 = len(over_30)
    sum_over_30 = sum(person.age for person in over_30)

    avg_over_30 = sum_over_30 / num_over_30

    print(f"The average age of people older than 30 is {avg_over_30}")


if __name__ == '__main__':
    main()
