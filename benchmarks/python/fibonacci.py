#!/usr/bin/env python3
# Fibonacci benchmark - calculate first N fibonacci numbers with modulo

def main():
    n = 1000000
    a = 0
    b = 1
    count = 0
    mod = 1000000007

    while count < n:
        temp = a
        a = b
        sum_val = temp + b
        b = sum_val % mod
        count += 1

    print("Fibonacci result (mod 1e9+7):")
    print(a)

if __name__ == "__main__":
    main()
