import math

radius = float(input("Enter the radius: "))
panjang = float(input("Enter the length: "))

area = math.pi * radius**2
volume = area * panjang

print(f"The area is {area:.4f}")
print(f"The volume is {volume:.1f}")