import math

def hitLuas(jari2):
    return jari2 * jari2 * math.pi

def total(luas, panjang):
    return luas * panjang

jari2 = 0
panjang = 0
print("Enter the radius and length of a cylinder: ")
jari2 = float(input())
panjang = float(input())

luas = hitLuas(jari2)
total = total(luas, panjang)

print("The area is " + str(luas) + " , ")
print("The volume of the cylinder is " + str(total))