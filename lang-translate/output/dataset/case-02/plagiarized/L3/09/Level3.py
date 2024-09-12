import math

radius = float(input("Enter the radius: "))
length = float(input("Enter the length: "))

luasAlas = radius * radius * math.pi
vol = luasAlas * length

print(f"The area is {luasAlas}")
print(f"The volume of the cylinder is {vol}")