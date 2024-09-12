import math

rad = float(input("Enter the radius and length of a cylinder : "))
l = float(input())
ar = rad * rad * math.pi
vol = ar * l
print("The area is " + str(ar))
print("The volume of the cylinder is " + str(vol))