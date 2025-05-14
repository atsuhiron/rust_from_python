import heavy_functions


def func(value: int) -> int:
    return heavy_functions.compute_heavy(value)


if __name__ == "__main__":
    print(func(12))

