import math

def area(r):
    area = r * r * math.pi
    return area

def volume(len, a):
    volume = len * a
    return volume

# Enter radius of the cylinder    
r = float(input("Enter the radius and length of a cylinder: "))
len = float(input())

a = area(r)
volume = volume(len,a)
#Output
print("The area is " + str(a))
print("The volume of the cylinder is " + str(volume))
