import math

class inputoutput:
    rad = 0.0
    panjang = 0.0

    def input(self):
        rad = float(input("Enter the radius: "))
        panjang = float(input("Enter the length: "))

    def hitArea(self):
        return math.pi * self.rad * self.rad

if __name__ == "__main__":
    cyl = inputoutput()
    cyl.input()
    a = cyl.hitArea()
    vol = a * cyl.panjang
    print(f"The area is {a:.2f}")
    print(f"The volume of the cylinder is {vol:.2f}")