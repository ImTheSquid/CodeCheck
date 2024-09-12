def reverse_integer(a):
  """Reverses the digits of an integer.

  Args:
    a: The integer to reverse.

  Returns:
    The reversed integer.
  """
  b = str(a)
  reversed_string = ""
  for i in range(len(b)):
    reversed_string += b[len(b) - 1 - i]
  return int(reversed_string)

a = int(input("Enter an integer: "))
reversed_integer = reverse_integer(a)
print(reversed_integer)