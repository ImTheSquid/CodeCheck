import math

def hitArea(rad):
  return rad * rad * math.pi

def main():
  rad = 2.5
  panjang = 5.0
  a = hitArea(rad)
  vol = a * panjang
  print(f"The area is {a:.2f}")
  print(f"The volume of the cylinder is {vol:.2f}")

if __name__ == "__main__":
  main()
