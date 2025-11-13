#!/usr/bin/env python3
# Array operations benchmark - sum large array

def main():
    size = 5000000
    sum_val = 0
    i = 0

    # Create and sum array
    while i < size:
        sum_val += i
        i += 1

    print("Array sum:")
    print(sum_val)

if __name__ == "__main__":
    main()
