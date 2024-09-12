# Delarasi
arrNum = [0] * 10

# for untuk 10 kali input
for i in range(10):
    # Read a number
    arrNum[i] = int(input("Read a number: "))

# Display the array
for i in range(9, -1, -1):
    #Print angka dalam array
    print(arrNum[i])