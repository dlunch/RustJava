import java.io.Closeable;
import java.io.Flushable;
import java.io.IOException;
import java.io.PrintWriter;
import java.io.StringWriter;
import java.util.Formattable;
import java.util.FormattableFlags;
import java.util.Formatter;

public class FormatterIntegration implements Formattable {
    private static class SnapshotSequence implements CharSequence {
        public int length() {
            return 1;
        }

        public char charAt(int index) {
            if (index != 0) {
                throw new IndexOutOfBoundsException();
            }
            return 'x';
        }

        public CharSequence subSequence(int start, int end) {
            if (start != 0 || end != 1) {
                throw new IndexOutOfBoundsException();
            }
            return "slice";
        }

        public String toString() {
            return "snapshot";
        }
    }

    private static class FailingAppendable implements Appendable, Flushable, Closeable {
        private final StringBuilder output = new StringBuilder();
        private int calls;
        private int flushes;
        private int closes;

        public Appendable append(CharSequence value) throws IOException {
            calls++;
            if (calls == 2 || calls == 4) {
                throw new IOException("io-" + calls);
            }
            output.append(value);
            return this;
        }

        public Appendable append(CharSequence value, int start, int end) throws IOException {
            return append(value.subSequence(start, end));
        }

        public Appendable append(char value) throws IOException {
            return append(String.valueOf(value));
        }

        public void flush() throws IOException {
            flushes++;
            throw new IOException("flush");
        }

        public void close() throws IOException {
            closes++;
            throw new IOException("close");
        }

        public String toString() {
            return output.toString();
        }
    }

    public void formatTo(Formatter formatter, int flags, int width, int precision) {
        String value = (flags & FormattableFlags.UPPERCASE) != 0 ? "FORM" : "form";
        formatter.format("%s:%d:%d:%d", value, flags, width, precision);
    }

    public static void main(String[] args) {
        System.out.println(String.format("%#-12.3S|%04x|%.2f", new FormatterIntegration(), 255, 1.5));

        FailingAppendable output = new FailingAppendable();
        Formatter formatter = new Formatter(output);
        formatter.format("A%sB%sC", "x", "y");
        System.out.println(output + "|" + output.calls + "|" + formatter.ioException().getMessage());
        formatter.flush();
        formatter.close();
        formatter.close();
        System.out.println(output.flushes + "|" + output.closes + "|" + formatter.ioException().getMessage());

        SnapshotSequence sequence = new SnapshotSequence();
        System.out.append(sequence).append('|').append(sequence, 0, 1).println();
        StringWriter destination = new StringWriter();
        new PrintWriter(destination).append(sequence).append('|').append(sequence, 0, 1).flush();
        System.out.println(destination);
    }
}
