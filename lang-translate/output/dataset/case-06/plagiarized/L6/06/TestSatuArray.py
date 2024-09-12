def inputArr(arr):
  arr2 = [0] * 10
  x = 0
  while x < 10:
    arr[x] = int(input(f"Read a number: "))
    x += 1
  x = 9
  for i in range(10):
    arr2[i] = arr[x]
    x -= 1
  return arr2

def printArr(arr):
  x = 0
  while x < 10:
    print(arr[x])
    x += 1

arr = [0] * 10
printArr(inputArr(arr))