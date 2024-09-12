import math

def count(rad, height):
    print(f"The area is {math.pow(rad, 2) * math.pi}")
    print(f"The volume of the cylinder is {math.pow(rad, 2) * math.pi * height}")

if __name__ == '__main__':
    radius = float(input("Enter the radius: "))
    tinggi = float(input("Enter the length: "))
    count(radius, tinggi)