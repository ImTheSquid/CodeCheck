import math

#program menghitung luas alas dan volume
def main():
    
    #enter the radius and length of a cylinder: 
    rad = float(input("Enter the radius and length of a cylinder: "))
    panjang = float(input())
    
    luas = rad * rad * math.pi
    vol = luas * panjang
    print("The area is " + str(luas))
    print("The volume of the cylinder is " + str(vol))

if __name__ == "__main__":
    main()