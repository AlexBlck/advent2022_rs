import numpy as np


def main():
    with open("../data/input.txt") as f:
        for line in f:
            signal = line.strip()
            for n in range(4, len(signal)):
                marker = signal[n - 4 : n]
                marker = [x for x in marker]
                if len(np.unique(marker)) == 4:
                    print(n)
                    break


if __name__ == "__main__":
    main()
