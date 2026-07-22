import java.io.ByteArrayInputStream;
import java.io.InputStreamReader;
import java.io.UnsupportedEncodingException;

class UnsupportedCharset {
    public static void main(String[] args) throws Exception {
        try {
            "hi".getBytes("UTF-16");
            System.out.println("Should not be executed");
        } catch (UnsupportedEncodingException e) {
            System.out.println("Caught getBytes");
            System.out.println(e.getMessage());
        }

        try {
            new String(new byte[] {104, 105}, "Shift_JIS");
            System.out.println("Should not be executed");
        } catch (UnsupportedEncodingException e) {
            System.out.println("Caught String ctor");
            System.out.println(e.getMessage());
        }

        System.setProperty("file.encoding", "ISO-8859-1");
        ByteArrayInputStream latin1Stream = new ByteArrayInputStream(new byte[] {0x61, (byte) 0xe9, 0x62});
        InputStreamReader latin1Reader = new InputStreamReader(latin1Stream);
        char[] buf = new char[3];
        int read = latin1Reader.read(buf, 0, 3);
        System.out.println(read);
        System.out.println(new String(buf, 0, read));

        System.setProperty("file.encoding", "UTF-16");
        ByteArrayInputStream utf16Stream = new ByteArrayInputStream(new byte[] {104, 105});
        InputStreamReader utf16Reader = new InputStreamReader(utf16Stream);
        try {
            utf16Reader.read(buf, 0, 2);
            System.out.println("Should not be executed");
        } catch (UnsupportedEncodingException e) {
            System.out.println("Caught reader");
            System.out.println(e.getMessage());
        }
    }
}
