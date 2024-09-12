import math

radius = float(input("Enter the radius: "))
length = float(input("Enter the length: "))
area = radius * radius * math.pi
volume = area * length
print("The area is", area)
print("The volume is", volume)