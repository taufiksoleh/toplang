#!/usr/bin/env python3
# Prime numbers benchmark - count primes up to N

def main():
    limit = 50000
    count = 0
    num = 2

    while num < limit:
        is_prime = 1
        i = 2

        # Check if num is prime
        while i * i < num + 1:
            remainder = num - ((num // i) * i)
            if remainder == 0:
                is_prime = 0
                i = num  # Break out of loop
            i += 1

        if is_prime == 1:
            count += 1

        num += 1

    print("Prime count:")
    print(count)

if __name__ == "__main__":
    main()
