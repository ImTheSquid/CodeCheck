import math

# Enter radius of the cylinder
radius = float(input("Enter the radius and length of a cylinder: "))
length = float(input())

# Hitung Area
area = radius * radius * math.pi
# Hitung Volume
volume = area * length

# Print area cylinder
print("The area is " + str(area))

# Print volume cylinder
print("The volume of the cylinder is " + str(volume))
