def simpanArray(arr, i):
    sc = input("Read a number: ")
    arr[i] = int(sc)
    if i > 0:
        simpanArray(arr, i - 1)

def tampilArray(arr, i):
    if i >= 0:
        print(arr[i])
        tampilArray(arr, i - 1)

if __name__ == "__main__":
    n = 10
    arr = [0] * n
    simpanArray(arr, n - 1)
    tampilArray(arr, n - 1)