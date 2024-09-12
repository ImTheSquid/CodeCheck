def inputArr(arr):
  """
  Reads 10 integers from user input and stores them in the array.

  Args:
      arr: An empty array of integers.

  Returns:
      The array filled with user input.
  """
  for x in range(10):
    arr[x] = int(input(f"Read a number: "))
  return arr

def printArr(arr):
  """
  Prints the elements of the array in reverse order.

  Args:
      arr: The array to be printed.
  """
  for x in range(9, -1, -1):
    print(arr[x])

# Initialize an empty array
arr = [0] * 10
# Fill the array with user input
arr = inputArr(arr)
# Print the array in reverse order
printArr(arr)