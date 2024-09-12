def main():
    print("Miles\t\tKilometers")
    print("-------------------------------")
    
    #Use while loop
    mile = 1
    while mile <= 10:
            print(mile, "\t\t", mile * 1.609)
            mile += 1

if __name__ == "__main__":
    main()