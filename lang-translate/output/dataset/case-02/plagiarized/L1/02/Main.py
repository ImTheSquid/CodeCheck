import math

radius = float(input("Enter the radius: "))
length = float(input("Enter the length: "))

area = math.pi * radius**2
volume = area * length

print(f"The area is {area:.2f}")
print(f"The volume of the cylinder is {volume:.2f}")