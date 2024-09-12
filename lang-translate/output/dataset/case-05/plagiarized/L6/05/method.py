class method:
    #method reverse
    input = input
    def beautyReverse(self, num):
        
        while num > 0:
            
            print(num % 10, end='')
            num //= 100 * 10
        print()
    def reverse(self):
        n = int(input())
        self.beautyReverse(n)
    
    #prog utama
    def main(self):
        print("Enter an integer: ")
        
        
        #pamggil method
        self.reverse()
        
        

if __name__ == '__main__':
    method().main()