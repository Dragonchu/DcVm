/**
 * 简单的打印测试程序
 * 用于测试System.out.println native方法调用
 */
public class SimplePrintTest {
    public static void main(String[] args) {
        System.out.println("Hello, JVM!");
        System.out.println(42);
        System.out.println("测试中文输出");
        System.out.println(3.14);
        System.out.println(true);
        System.out.println('A');
    }
} 