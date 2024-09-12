import math

def hitungLuasAlas(rad):
  luasAlas = rad * rad * math.pi
  return luasAlas

def hitungVolume(luasAlas, tinggi):
  volume = luasAlas * tinggi
  return volume

#masukkan nilai
radius = float(input("Enter the radius: "))
tinggi = float(input("Enter the length: "))

print("The area is", hitungLuasAlas(radius))
print("The volume of the cylinder is", hitungVolume(hitungLuasAlas(radius), tinggi))