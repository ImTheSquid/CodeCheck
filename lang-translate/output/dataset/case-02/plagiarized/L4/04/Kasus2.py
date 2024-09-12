import math

def hitungArea(r):
    return r * r * math.pi

def hitungVolume(r, l):
    return hitungArea(r) * l

# Enter radius of the cylinder
r = 3
l = 5

print("The area is " + str(hitungArea(r)))
print("The volume of the cylinder is " + str(hitungVolume(r, l)))