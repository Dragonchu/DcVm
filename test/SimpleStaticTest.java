/**
 * 简单的静态字段测试程序
 * 用于测试静态初始化块
 */
public class SimpleStaticTest {
    // 静态字段
    private static int staticInt = 42;
    private static String staticString = "Hello, JVM!";
    
    // 静态初始化块
    static {
        System.out.println("静态初始化块执行");
        staticInt = 100;
    }
    
    public static void main(String[] args) {
        System.out.println("main方法开始执行");
        System.out.println("staticInt: " + staticInt);
        System.out.println("staticString: " + staticString);
        System.out.println("程序执行完成");
    }
} 