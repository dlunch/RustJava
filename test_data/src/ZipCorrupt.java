import java.io.File;
import java.util.zip.ZipException;
import java.util.zip.ZipFile;

public class ZipCorrupt {
    public static void main(String[] args) throws Exception {
        try {
            new ZipFile(new File("test_data/FileLength.txt"));
            System.out.println("no-ex");
        } catch (ZipException e) {
            System.out.println("zipex");
        }
    }
}
