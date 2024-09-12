import math

# masukan silinder
rad = float(input("Enter the radius of a cylinder: "))
panjang = float(input("Enter the length of a cylinder: "))

#perhitungan
a = rad * rad * math.pi
vol = a * panjang

# cetak hasil
print("The area is " + str(a))
print("The volume of the cylinder is " + str(vol))