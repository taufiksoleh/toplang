#!/usr/bin/env python3
# Array sum benchmark - Python equivalent

def main():
    size = 100000
    iterations = 100
    
    # Create array
    arr = []
    i = 0
    while i < size:
        arr.append(i)
        i = i + 1
    
    total = 0
    iter_count = 0
    
    while iter_count < iterations:
        sum_val = 0
        idx = 0
        while idx < size:
            sum_val = sum_val + arr[idx]
            idx = idx + 1
        total = total + sum_val
        iter_count = iter_count + 1
    
    print("Total sum:")
    print(total)
    
    return 0

if __name__ == "__main__":
    exit(main())
