import math

radius = float(input("Enter the radius and length of a cylinder: "))
length = float(input())

luas = radius * radius * math.pi
volume = luas * length

print("The area is", luas)
print("The volume of the cylinder is", volume)