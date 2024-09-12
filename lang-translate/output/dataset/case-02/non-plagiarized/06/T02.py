import math

radius = float(input("Enter the radius: "))
length = float(input("Enter the length: "))

area = math.pi * radius * radius
volume = math.pi * radius * radius * length

print("The area is", area)
print("The volume is", volume)