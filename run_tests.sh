#!/bin/bash

# JVM测试运行脚本
# 支持选择测试文件、自动编译、执行

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置
TEST_DIR="test"
BUILD_DIR="test"
JAVA_SRC_DIR="test"

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 显示帮助信息
show_help() {
    echo "JVM测试运行脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  -f, --file <文件名>     指定要运行的Java文件（不含.java扩展名）"
    echo "  -l, --list             列出所有可用的测试文件"
    echo "  -c, --compile          强制重新编译Java文件"
    echo "  -q, --quiet            静默模式，不输出JVM调试日志"
    echo "  -h, --help             显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  $0 -f TestProgram      运行TestProgram.java"
    echo "  $0 -f TestProgram -c   强制重新编译并运行TestProgram.java"
    echo "  $0 -f TestProgram -q   静默模式运行TestProgram.java"
    echo "  $0 -l                  列出所有测试文件"
}

# 列出所有可用的测试文件
list_test_files() {
    print_info "可用的测试文件:"
    echo ""
    
    if [ ! -d "$JAVA_SRC_DIR" ]; then
        print_error "测试目录不存在: $JAVA_SRC_DIR"
        exit 1
    fi
    
    java_files=($(find "$JAVA_SRC_DIR" -name "*.java" -type f | sort))
    
    if [ ${#java_files[@]} -eq 0 ]; then
        print_warning "没有找到Java文件"
        return
    fi
    
    for file in "${java_files[@]}"; do
        filename=$(basename "$file" .java)
        class_file="$BUILD_DIR/$filename.class"
        
        if [ -f "$class_file" ]; then
            echo -e "  ${GREEN}✓${NC} $filename (已编译)"
        else
            echo -e "  ${YELLOW}○${NC} $filename (未编译)"
        fi
    done
    echo ""
}

# 编译Java文件
compile_java_file() {
    local java_file="$1"
    local class_file="$2"
    local force_compile="$3"
    
    # 检查是否需要编译
    if [ "$force_compile" != "true" ] && [ -f "$class_file" ]; then
        print_info "Class文件已存在，跳过编译: $class_file"
        return 0
    fi
    
    print_info "编译Java文件: $java_file"
    
    # 检查Java文件是否存在
    if [ ! -f "$java_file" ]; then
        print_error "Java文件不存在: $java_file"
        return 1
    fi
    
    # 编译Java文件
    if javac -d "$BUILD_DIR" "$java_file"; then
        print_success "编译成功: $java_file -> $class_file"
        return 0
    else
        print_error "编译失败: $java_file"
        return 1
    fi
}

# 检测JDK rt.jar路径
detect_rt_jar_path() {
    # 方法1: 检查JAVA_HOME环境变量
    if [ -n "$JAVA_HOME" ] && [ -f "$JAVA_HOME/jre/lib/rt.jar" ]; then
        rt_jar_path="$JAVA_HOME/jre/lib/rt.jar"
        print_info "检测到rt.jar: $rt_jar_path"
        return 0
    fi
    
    # 方法2: 检查常见的JDK安装路径
    local common_paths=(
        "/Library/Java/JavaVirtualMachines/jdk1.8.0_*/Contents/Home/jre/lib/rt.jar"
        "/System/Library/Java/JavaVirtualMachines/1.8.0.jdk/Contents/Home/jre/lib/rt.jar"
        "/usr/lib/jvm/java-8-openjdk*/jre/lib/rt.jar"
        "/usr/lib/jvm/java-8-oracle/jre/lib/rt.jar"
        "/opt/java/jdk1.8.0_*/jre/lib/rt.jar"
    )
    
    for pattern in "${common_paths[@]}"; do
        for path in $pattern; do
            if [ -f "$path" ]; then
                rt_jar_path="$path"
                print_info "检测到rt.jar: $rt_jar_path"
                return 0
            fi
        done
    done
    
    # 方法3: 使用java命令查找
    if command -v java &> /dev/null; then
        local java_path=$(which java)
        if [[ "$java_path" == *"/bin/java" ]]; then
            local jdk_home=$(dirname "$(dirname "$java_path")")
            if [ -f "$jdk_home/jre/lib/rt.jar" ]; then
                rt_jar_path="$jdk_home/jre/lib/rt.jar"
                print_info "检测到rt.jar: $rt_jar_path"
                return 0
            fi
        fi
    fi
    
    print_error "未找到rt.jar，请确保已安装Java 8 JDK"
    print_error "请设置JAVA_HOME环境变量或确保JDK正确安装"
    return 1
}

# 运行JVM程序
run_jvm() {
    local class_name="$1"
    local quiet_mode="$2"
    
    print_info "启动JVM执行: $class_name"
    
    # 检测rt.jar路径
    if ! detect_rt_jar_path; then
        return 1
    fi
    
    # 构建classpath: test目录 + rt.jar
    local test_abs_path="$(pwd)/test"
    local classpath="$test_abs_path:$rt_jar_path"
    print_info "使用classpath: $classpath"
    
    # 切换到vm目录
    cd vm
    
    # 构建JVM命令参数
    local jvm_args="../test/$class_name.class"
    jvm_args="$jvm_args $classpath"
    
    # 如果启用静默模式，添加--quiet参数
    if [ "$quiet_mode" = true ]; then
        jvm_args="$jvm_args --quiet"
    fi
    
    # 运行JVM程序
    if cargo run --bin vm -- $jvm_args; then
        print_success "JVM执行完成"
    else
        print_error "JVM执行失败"
        return 1
    fi
}

# 主函数
main() {
    local selected_file=""
    local force_compile=false
    local list_files=false
    local quiet_mode=false
    
    # 解析命令行参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            -f|--file)
                selected_file="$2"
                shift 2
                ;;
            -c|--compile)
                force_compile=true
                shift
                ;;
            -l|--list)
                list_files=true
                shift
                ;;
            -q|--quiet)
                quiet_mode=true
                shift
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                print_error "未知选项: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    # 如果只是列出文件
    if [ "$list_files" = true ]; then
        list_test_files
        exit 0
    fi
    
    # 如果没有指定文件，显示交互式选择
    if [ -z "$selected_file" ]; then
        print_info "请选择要运行的测试文件:"
        echo ""
        list_test_files
        
        # 获取所有Java文件
        java_files=($(find "$JAVA_SRC_DIR" -name "*.java" -type f | sort))
        
        if [ ${#java_files[@]} -eq 0 ]; then
            print_error "没有找到Java文件"
            exit 1
        fi
        
        # 显示选择菜单
        echo "请选择文件编号 (1-${#java_files[@]}):"
        for i in "${!java_files[@]}"; do
            filename=$(basename "${java_files[$i]}" .java)
            echo "  $((i+1)). $filename"
        done
        
        read -p "请输入编号: " choice
        
        # 验证输入
        if ! [[ "$choice" =~ ^[0-9]+$ ]] || [ "$choice" -lt 1 ] || [ "$choice" -gt ${#java_files[@]} ]; then
            print_error "无效的选择"
            exit 1
        fi
        
        selected_file=$(basename "${java_files[$((choice-1))]}" .java)
    fi
    
    # 设置文件路径
    java_file="$JAVA_SRC_DIR/$selected_file.java"
    class_file="$BUILD_DIR/$selected_file.class"
    
    print_info "选择的文件: $selected_file"
    
    # 编译Java文件
    if ! compile_java_file "$java_file" "$class_file" "$force_compile"; then
        exit 1
    fi
    
    # 检查class文件是否存在
    if [ ! -f "$class_file" ]; then
        print_error "Class文件不存在: $class_file"
        exit 1
    fi
    
    # 运行JVM程序
    if ! run_jvm "$selected_file" "$quiet_mode"; then
        exit 1
    fi
    
    print_success "测试完成!"
}

# 检查必要的工具
check_dependencies() {
    if ! command -v javac &> /dev/null; then
        print_error "未找到javac，请确保已安装Java JDK"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        print_error "未找到cargo，请确保已安装Rust"
        exit 1
    fi
}

# 检查依赖并运行主函数
check_dependencies
main "$@" 