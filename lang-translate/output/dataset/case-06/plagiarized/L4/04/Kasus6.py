def simpanArray(arr, n):
    for i in range(n):
        arr[i] = int(input("Read a number: "))

def tampilArray(arr, n):
    for i in range(n - 1, -1, -1):
        print(arr[i])

if __name__ == "__main__":
    n = 10
    arr = [0] * n
    simpanArray(arr, n)
    tampilArray(arr, n)