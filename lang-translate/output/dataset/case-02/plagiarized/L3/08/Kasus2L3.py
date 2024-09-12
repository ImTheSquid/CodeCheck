import math

# Enter radius of the cylinder
radius = float(input("Enter the radius and length of a cylinder: "))
length = float(input())

# Hitung Area
luas = radius * radius * math.pi
# Print area cylinder
print("The area is " + str(luas))

#Hitung Volume
volume = luas * length
# Print volume cylinder
print("The volume of the cylinder is " + str(volume))