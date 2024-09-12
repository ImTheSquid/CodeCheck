# import the Scanner class
import java.util.Scanner;

# define the Level5 class
public class Level5 {

    # define the main method
    public static void main(String[] args) {
        # create a Scanner object to read input from the user
        Scanner sc = new Scanner(System.in); 

        # create an integer array with a size of 10
        int arrAngka[] = new int[10];

        # loop through the array and read input from the user
        for (int x = 0; x < 10; x++) {
            # print a message to the user asking for a number
            System.out.print("Read a number: ");
            # read an integer from the user and store it in the array
            arrAngka[x] = sc.nextInt();
        }

        # loop through the array in reverse order and print each element
        for (int x = 9; x >= 0; x--)
            System.out.println(arrAngka[x]);
    }
}