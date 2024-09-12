import math

def area(r):
    """Calculates the area of a circle given its radius.

    Args:
        r (float): The radius of the circle.

    Returns:
        float: The area of the circle.
    """
    area = r * r * math.pi
    return area

def volume(len, a):
    """Calculates the volume of a cylinder given its length and base area.

    Args:
        len (float): The length of the cylinder.
        a (float): The area of the base of the cylinder.

    Returns:
        float: The volume of the cylinder.
    """
    volume = len * a
    return volume

# Get input from the user
r = float(input("Enter the radius of the cylinder: "))
len = float(input("Enter the length of the cylinder: "))

# Calculate the area of the base
a = area(r)

# Calculate the volume of the cylinder
volume_cylinder = volume(len, a)

# Print the results
print("The area of the cylinder is:", a)
print("The volume of the cylinder is:", volume_cylinder)