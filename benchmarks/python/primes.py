#!/usr/bin/env python3
# Primes benchmark - Python equivalent

def is_prime(n):
    if n <= 1:
        return False
    if n <= 3:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    
    i = 5
    while i * i <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i = i + 6
    
    return True

def main():
    limit = 100000
    count = 0
    n = 2
    
    while n < limit:
        if is_prime(n):
            count = count + 1
        n = n + 1
    
    print("Primes found:")
    print(count)
    
    return 0

if __name__ == "__main__":
    exit(main())
