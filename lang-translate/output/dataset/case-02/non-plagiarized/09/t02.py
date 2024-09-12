import math

radius = 2.5
length = 5.0

area = radius * radius * math.pi
volume = area * length

area_format = "{:.5f}".format(area)
volume_format = "{:.1f}".format(volume)

print(f"The area is {area_format}")
print(f"The volume is {volume_format}")
