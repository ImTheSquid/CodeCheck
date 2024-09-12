import math

def hitungArea(r):
    return r * r * math.pi

# Enter radius of the cylinder
r = float(input("Enter the radius and length of a cylinder: "))
l = float(input())

print("The area is " + str(hitungArea(r)))
print("The volume of the cylinder is " + str(hitungArea(r)*l))