import math

def hitungArea(jejari):
    area = math.pow(jejari, 2) * 3.14159
    return area

def hitungVolume(panjang, jejari):
    volume = hitungArea(jejari) * panjang
    return volume

jejari = float(input("Enter the radius of the cylinder: "))
panjang = float(input("Enter the length of the cylinder: "))

print("The area is", hitungArea(jejari))
print("The volume of the cylinder is", hitungVolume(panjang, jejari))