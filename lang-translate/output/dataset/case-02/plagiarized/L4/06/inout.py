import math

def hitungAr(rad):
    return rad * rad * math.pi

def hitungVol(ar, l):
    return ar * l

rad = float(input("Enter the radius and length of a cylinder : "))
l = float(input())
ar = hitungAr(rad)
vol = hitungVol(ar, l)
print("The area is " + str(ar))
print("The volume of the cylinder is " + str(vol))