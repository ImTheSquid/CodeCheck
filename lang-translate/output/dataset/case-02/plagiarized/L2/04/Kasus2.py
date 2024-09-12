import math

def hitungArea(r):
    return r * r * math.pi

def hitungVolume(area, l):
    return area * l

# Enter radius of the cylinder
r = float(input("Enter the radius and length of a cylinder: "))
l = float(input())

area = hitungArea(r)
volume = hitungVolume(area, l)
print("The area is " + str(area))
print("The volume of the cylinder is " + str(volume))