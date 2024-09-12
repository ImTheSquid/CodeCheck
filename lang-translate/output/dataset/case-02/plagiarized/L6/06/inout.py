import math

def hitungAr(rad):
    ar = rad * rad * math.pi
    print(f"The area is {ar}")
    return ar

def hitungVol(ar, l):
    vol = ar * l
    print(f"The volume of the cylinder is {vol}")

# Enter radius of the cylinder
rad = float(input("Enter the radius and length of a cylinder : "))
l = float(input())
hitungVol(hitungAr(rad), l)