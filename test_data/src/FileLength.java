import java.io.File;

public class FileLength {
    public static void main(String[] args) {
        File missing = new File("definitely-missing-file");
        System.out.println(missing.exists());
        System.out.println(missing.length());
    }
}
