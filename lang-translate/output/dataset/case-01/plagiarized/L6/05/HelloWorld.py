class HelloWorld:
    s = ""

    def Cetak():
        for a in range(5, 0, -1):
            print(HelloWorld.s)

    def main(args):
        HelloWorld.s = "Welcome to Java"
        HelloWorld.Cetak()

HelloWorld.main([])