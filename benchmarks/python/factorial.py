#!/usr/bin/env python3
# Factorial benchmark - calculate multiple factorials with modulo

def main():
    n = 5000
    count = 0
    modval = 1000000007

    while count < n:
        result = 1
        i = 1

        # Calculate factorial with modulo
        while i < count + 1:
            prod = result * i
            result = prod % modval
            i += 1

        count += 1

    print("Factorial iterations:")
    print(count)

if __name__ == "__main__":
    main()
