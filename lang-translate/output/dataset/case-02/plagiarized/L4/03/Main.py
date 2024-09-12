import math

radius = float(input("Enter the radius of the cylinder: "))
length = float(input("Enter the length of the cylinder: "))
areas = radius * radius * math.pi
volumes = areas * length

print("The area is", areas)
print("The volume of the cylinder is", volumes)