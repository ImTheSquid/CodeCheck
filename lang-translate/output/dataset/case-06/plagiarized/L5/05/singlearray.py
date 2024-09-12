n = [0] * 10

def print_array():
    for a in range(9, -1, -1):
        print(n[a])

def main():
    i = 0
    while i < 10:
        n[i] = int(input("Read a number: "))
        i += 1
    print_array()

if __name__ == "__main__":
    main()