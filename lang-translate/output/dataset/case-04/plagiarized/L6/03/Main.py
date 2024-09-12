mile = 1
print("Miles\t\tKilometers")
print("-------------------------------")

def loops():
    #Use while loop
    for i in range(10, 0, -1):
        print(mile, "\t\t", mile * 1.609)
        mile += 1

loops()