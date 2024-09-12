import math

radius = float(input("Enter the radius of the cylinder: "))
length = int(input("Enter the length of the cylinder: "))

area = radius * radius * math.pi
print("The area is", area)
print("The volume is", area * length)