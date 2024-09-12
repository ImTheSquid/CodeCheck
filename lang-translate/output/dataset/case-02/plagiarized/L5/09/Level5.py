import math

def hitung(radius, tinggi):
    area = radius * radius * math.pi
    volume = radius * radius * math.pi * tinggi
    print("The area is " + str(area))
    print("The volume of the cylinder is " + str(volume))

radius = float(input("Enter the radius of the cylinder: "))
tinggi = float(input("Enter the length of the cylinder: "))
hitung(radius, tinggi)