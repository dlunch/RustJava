public class SurrogateChars {
    public static void main(String[] args) {
        StringBuffer sb = new StringBuffer();
        sb.append('\uD83D').append('\uDE00');
        System.out.println(sb.length());
        System.out.println(sb.toString());

        StringBuffer lone = new StringBuffer();
        lone.append('\uD800');
        System.out.println(lone.length());

        System.out.println(String.valueOf('\uD800').length());
        System.out.println('\uD800');

        char[] arr = { '\uD83D', '\uDE00' };
        StringBuffer sb2 = new StringBuffer();
        sb2.append(arr, 0, 2);
        System.out.println(sb2.length());
    }
}
