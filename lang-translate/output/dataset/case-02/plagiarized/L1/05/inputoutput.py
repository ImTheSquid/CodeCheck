import math

# masukan silinder
radius = float(input("Enter the radius of a cylinder: "))
length = float(input("Enter the length of a cylinder: "))

#perhitungan
area = radius * radius * math.pi
volume = area * length

# cetak hasil
print("The area is", area)
print("The volume of the cylinder is", volume)