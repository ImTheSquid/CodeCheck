import math

radius = float(input("Enter the radius of the cylinder: "))
length = float(input("Enter the length of the cylinder: "))

area = math.pi * radius**2
volume = area * length

print("The area is", area)
print(f"The volume is {volume:.1f}")