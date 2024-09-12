import math

def hitungAr(rad):
    return rad * rad * math.pi

def hitungVol(ar, l):
    return ar * l

# Enter radius of the cylinder
rad = float(input("Enter the radius and length of a cylinder : "))
l = float(input())
ar = hitungAr(rad)
vol = hitungVol(ar, l)
print(f"The area is {ar}")
print(f"The volume of the cylinder is {vol}")