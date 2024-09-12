def ShowTable(jumlahData):
    for i in range(jumlahData, 0, -1):
        temp = jumlahData + 1 - i
        print(f'{temp}\t\t{ConvertMilesToKilo(temp)}')

def ConvertMilesToKilo(miles):
    return miles * 1.609

print("Miles\t\tKilometers")
print("-------------------------------")
ShowTable(10)