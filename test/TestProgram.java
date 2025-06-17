/**
 * 测试程序 - 用于测试JVM实现
 * 仅使用Java 1.8特性
 */
public class TestProgram {
    // 静态字段测试
    private static int staticInt = 42;
    private static String staticString = "Hello, JVM!";
    private static boolean staticBoolean = true;
    private static long staticLong = 123456789L;
    private static double staticDouble = 3.14159;
    private static float staticFloat = 2.71828f;
    private static char staticChar = 'A';
    private static byte staticByte = 127;
    private static short staticShort = 32767;

    // 实例字段测试
    private int instanceInt;
    private String instanceString;
    private boolean instanceBoolean;
    private long instanceLong;
    private double instanceDouble;
    private float instanceFloat;
    private char instanceChar;
    private byte instanceByte;
    private short instanceShort;

    // 静态初始化块
    static {
        System.out.println("静态初始化块执行");
        staticInt = 100;
    }

    // 构造函数
    public TestProgram() {
        System.out.println("构造函数执行");
        instanceInt = 200;
        instanceString = "Instance String";
    }

    // 静态方法测试
    public static int add(int a, int b) {
        return a + b;
    }

    public static String concat(String a, String b) {
        return a + b;
    }

    public static boolean compare(int a, int b) {
        return a > b;
    }

    // 实例方法测试
    public int multiply(int a, int b) {
        return a * b;
    }

    public String getInstanceString() {
        return instanceString;
    }

    // 数组操作测试
    public static int[] createArray(int size) {
        return new int[size];
    }

    public static void fillArray(int[] arr, int value) {
        for (int i = 0; i < arr.length; i++) {
            arr[i] = value;
        }
    }

    // 控制流测试
    public static int ifElseTest(int value) {
        if (value > 0) {
            return 1;
        } else if (value < 0) {
            return -1;
        } else {
            return 0;
        }
    }

    public static int switchTest(int value) {
        switch (value) {
            case 1:
                return 10;
            case 2:
                return 20;
            case 3:
                return 30;
            default:
                return 0;
        }
    }

    // 循环测试
    public static int sumArray(int[] arr) {
        int sum = 0;
        for (int i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        return sum;
    }

    public static int whileTest(int n) {
        int sum = 0;
        int i = 1;
        while (i <= n) {
            sum += i;
            i++;
        }
        return sum;
    }

    // 异常处理测试
    public static int divide(int a, int b) {
        try {
            return a / b;
        } catch (ArithmeticException e) {
            System.out.println("除零错误");
            return 0;
        }
    }

    // 主方法
    public static void main(String[] args) {
        System.out.println("测试程序开始执行");

        // 测试静态字段
        System.out.println("静态字段值:");
        System.out.println("staticInt: " + staticInt);
        System.out.println("staticString: " + staticString);
        System.out.println("staticBoolean: " + staticBoolean);

        // 测试实例创建和方法调用
        TestProgram test = new TestProgram();
        System.out.println("实例字段值:");
        System.out.println("instanceInt: " + test.instanceInt);
        System.out.println("instanceString: " + test.instanceString);

        // 测试方法调用
        System.out.println("方法调用测试:");
        System.out.println("add(5, 3): " + add(5, 3));
        System.out.println("multiply(4, 6): " + test.multiply(4, 6));

        // 测试数组操作
        int[] arr = createArray(5);
        fillArray(arr, 10);
        System.out.println("数组和: " + sumArray(arr));

        // 测试控制流
        System.out.println("ifElseTest(5): " + ifElseTest(5));
        System.out.println("switchTest(2): " + switchTest(2));

        // 测试循环
        System.out.println("whileTest(5): " + whileTest(5));

        // 测试异常处理
        System.out.println("divide(10, 2): " + divide(10, 2));
        System.out.println("divide(10, 0): " + divide(10, 0));

        System.out.println("测试程序执行完成");
    }
} 