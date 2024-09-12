import math

class inputoutput:
    rad = 0.0
    panjang = 0.0
    
    def input(self):
        self.rad = float(input("Enter the radius: "))
        self.panjang = float(input("Enter the length: "))
        
    def hitArea(self):
        return math.pow(self.rad, 2) * 3.14159

if __name__ == "__main__":
    cylinder = inputoutput()
    
    print("Enter the radius and length of a cylinder: ")
    cylinder.input()
    
    a = cylinder.hitArea()
    vol = a * cylinder.panjang
    
    print("The area is " + str(a))
    print("The volume of the cylinder is " + str(vol)) 