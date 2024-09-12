import math

radius = float(input("Enter the radius of the cylinder: "))
length = float(input("Enter the length of the cylinder: "))
areas = radius * radius * math.pi
volumes = areas * length

print(f"The area is {areas:.2f}")
print(f"The volume of the cylinder is {volumes:.2f}")