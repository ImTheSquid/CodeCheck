import math

# Enter radius of the cylinder
radius = float(input("Enter the radius and length of a cylinder: "))
length = float(input())

areas = radius * radius * math.pi
volumes = areas * length

print("The area is", areas)
print("The volume of the cylinder is", volumes)