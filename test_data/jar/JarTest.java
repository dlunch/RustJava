import java.io.InputStream;
import java.io.IOException;

class JarTest {
    public static void main(String[] args) throws IOException {
        InputStream stream = JarTest.class.getResourceAsStream("test.txt");

        byte[] buffer = new byte[1024];
        stream.read(buffer);
        System.out.println(new String(buffer));
    }
}
