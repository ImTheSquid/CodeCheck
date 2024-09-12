import math

def hitungLuas(radius):
    return radius * radius * math.pi

def hitungVolume(luas, length):
    return luas * length

# Enter radius of the cylinder
print("Enter the radius and length of a cylinder: ")
radius = float(input())
# Hitung Area
luas = hitungLuas(radius)
#input Length
length = float(input())
# Print area cylinder
print("The area is " + str(luas))
#Hitung Volume
volume = hitungVolume(luas, length)
# Print volume cylinder
print("The volume of the cylinder is " + str(volume))