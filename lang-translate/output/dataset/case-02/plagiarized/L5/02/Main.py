import math

def area(r):
    """Calculates the area of a circle.

    Args:
        r: The radius of the circle.

    Returns:
        The area of the circle.
    """
    return math.pi * r * r

def volume(len, a):
    """Calculates the volume of a cylinder.

    Args:
        len: The length of the cylinder.
        a: The area of the base of the cylinder.

    Returns:
        The volume of the cylinder.
    """
    return len * a

# Enter radius and length of the cylinder
r = float(input("Enter the radius of the cylinder: "))
len = float(input("Enter the length of the cylinder: "))

a = area(r)

# Output
print("The area is", area(r))
print("The volume of the cylinder is", volume(len, a))