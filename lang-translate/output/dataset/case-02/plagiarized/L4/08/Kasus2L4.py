import math

def hitungLuas(radius):
    return radius * radius * math.pi

def hitungVolume(luas, length):
    return luas * length

# Enter radius of the cylinder
radius = float(input("Enter the radius and length of a cylinder: "))
length = float(input())

# Hitung Area
luas = hitungLuas(radius)
# Print area cylinder
print("The area is " + str(luas))
#Hitung Volume
volume = hitungVolume(luas, length)
# Print volume cylinder
print("The volume of the cylinder is " + str(volume))