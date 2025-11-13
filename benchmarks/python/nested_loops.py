#!/usr/bin/env python3
# Nested loops benchmark - test loop performance

def main():
    outer = 5000
    sum_val = 0
    i = 0

    while i < outer:
        j = 0
        while j < 1000:
            sum_val += 1
            j += 1
        i += 1

    print("Nested loops result:")
    print(sum_val)

if __name__ == "__main__":
    main()
