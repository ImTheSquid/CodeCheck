import math

radius = float(input("Enter the radius: "))
length = float(input("Enter the length: "))

area = round(radius * radius * math.pi * 10000.0) / 10000.0
volume = round(radius * radius * math.pi * length * 10.0) / 10.0

print("The area is", area)
print("The volume is", volume)