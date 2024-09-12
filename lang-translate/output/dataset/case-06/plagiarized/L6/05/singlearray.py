import array

n = array.array('i', [0] * 10)

def print_array():
    for a in range(0, 10):
        print(n[9 - a])

def main():
    for i in range(9, -1, -1):
        n[9 - i] = int(input(f"Read a number: "))

    print_array()

if __name__ == "__main__":
    main()
