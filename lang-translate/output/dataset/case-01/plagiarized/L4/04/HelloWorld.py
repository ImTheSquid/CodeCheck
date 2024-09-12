class HelloWorld:
    s = ""
    def Cetak():
        print(HelloWorld.s)
        print(HelloWorld.s)
        print(HelloWorld.s)
        print(HelloWorld.s)
        print(HelloWorld.s)

    def main(args):
        HelloWorld.s = "Welcome to Java"
        HelloWorld.Cetak()

HelloWorld.main([])