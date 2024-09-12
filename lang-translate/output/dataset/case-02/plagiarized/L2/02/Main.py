import math

# Enter radius of the cylinder
r = float(input("Enter the radius of the cylinder: "))
len = float(input("Enter the length of the cylinder: "))

a = r * r * math.pi
volume = a * len

# Output
print(f"The area is {a}")
print(f"The volume of the cylinder is {volume}")