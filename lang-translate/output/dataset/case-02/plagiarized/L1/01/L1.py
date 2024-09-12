import math

radius = float(input("Enter the radius of the cylinder: "))
length = float(input("Enter the length of the cylinder: "))
area = radius * radius * math.pi
volume = area * length

print("The area is", area, ",")
print("The volume of the cylinder is", volume)