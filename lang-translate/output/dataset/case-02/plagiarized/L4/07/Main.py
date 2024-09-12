import math

def hitungArea(jejari):
  area = jejari * jejari * math.pi
  return area

def hitungVolume(panjang, jejari):
  volume = hitungArea(jejari) * panjang
  return volume

# Masukan radius silinder
jejari = float(input("Enter the radius and length of a cylinder: "))
panjang = float(input())

# Mencetak luas dan volume
print("The area is " + str(hitungArea(jejari)))
print("The volume of the cylinder is " + str(hitungVolume(panjang, jejari)))