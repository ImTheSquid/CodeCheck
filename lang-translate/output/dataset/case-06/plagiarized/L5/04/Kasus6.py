def simpanArray(arr, n):
    for i in range(n):
        arr[i] = int(input("Read a number: "))

def tampilArray(arr, i):
    print(arr[i])

n = 10
arr = [0] * n
simpanArray(arr, n)
for i in range(n-1, -1, -1):
    tampilArray(arr, i)