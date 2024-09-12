import math

radius = float(input("Enter the radius: "))
length = float(input("Enter the length: "))

area = math.pi * radius**2
volume = area * length

print("The area is {:.4f}".format(area))
print("The volume is {:.1f}".format(volume))
