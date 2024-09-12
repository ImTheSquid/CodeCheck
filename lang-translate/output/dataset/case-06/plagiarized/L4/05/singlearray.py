class singlearray:
    n = [0] * 10

    def print():
        for a in range(9, -1, -1):
            print(singlearray.n[a])

    def main():
        inp = input
        for i in range(10):
            print("Read a number: ")
            singlearray.n[i] = int(inp())
        singlearray.print()

singlearray.main()