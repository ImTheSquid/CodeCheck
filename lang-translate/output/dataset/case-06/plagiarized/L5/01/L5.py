def print_array(arr):
    i = 9
    while i >= 0:
        print(arr[i])
        i -= 1

def main():
    angka = [0] * 10
    i = 0
    while i != 10:
        angka[i] = int(input("Read a number: "))
        i += 1

    print_array(angka)

if __name__ == "__main__":
    main()