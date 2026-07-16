import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.RandomAccessFile;

public class FileErrors {
    public static void main(String[] args) {
        try {
            new FileInputStream(new File("definitely-missing-file"));
            System.out.println("no-ex");
        } catch (FileNotFoundException e) {
            System.out.println("fnf-in");
        }
        try {
            new FileOutputStream(new File("definitely-missing-dir/x"));
            System.out.println("no-ex");
        } catch (FileNotFoundException e) {
            System.out.println("fnf-out");
        }
        try {
            new RandomAccessFile("definitely-missing-file", "r");
            System.out.println("no-ex");
        } catch (FileNotFoundException e) {
            System.out.println("fnf-raf");
        }
    }
}
