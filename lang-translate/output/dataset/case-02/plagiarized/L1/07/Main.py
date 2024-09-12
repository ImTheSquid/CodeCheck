import math

# Masukan radius silinder
radius = float(input("Enter the radius of the cylinder: "))
length = float(input("Enter the length of the cylinder: "))

# Menghitung luas dan volume
area = math.pi * radius**2
volume = area * length

# Mencetak luas dan volume
print(f"The area is {area:.2f}")
print(f"The volume of the cylinder is {volume:.2f}")