def inputArr(arr):
    x = 0
    while x < 10:
        arr[x] = int(input("Read a number: "))
        x += 1
    return arr

def printArr(arr):
    x = 9
    while x >= 0:
        print(arr[x])
        x -= 1

arr = [0] * 10  # Initialize array with 10 elements
arr = inputArr(arr)
printArr(arr)
