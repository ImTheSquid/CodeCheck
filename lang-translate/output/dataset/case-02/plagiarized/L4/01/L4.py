import math

def hitLuas(jari2):
    return jari2 * jari2 * 3.14159

def total(luas, panjang):
    return luas * panjang

jari2 = 2.0
panjang = 3.0
luas = hitLuas(jari2)
total = total(luas, panjang)

print(f"The area is {luas}, ")
print(f"The volume of the cylinder is {total}")