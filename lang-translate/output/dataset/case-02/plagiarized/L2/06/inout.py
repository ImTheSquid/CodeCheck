import math

radius = float(input("Enter the radius and length of a cylinder separated by a space: "))
length = float(input())

area = math.pi * radius**2
volume = area * length

print(f"The area is {area}")
print(f"The volume of the cylinder is {volume}")