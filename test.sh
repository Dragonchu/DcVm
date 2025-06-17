#!/bin/bash

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 检查是否为Mac系统
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${YELLOW}警告: 当前仅支持Mac系统${NC}"
    exit 1
fi

echo -e "${GREEN}检测到Mac系统${NC}"

# 检查本地Java版本
CURRENT_JAVA=$(java -version 2>&1 | awk -F '"' '/version/ {print $2}')
if [[ $CURRENT_JAVA == 1.8* ]]; then
    echo -e "${GREEN}当前已使用Java 8版本: $CURRENT_JAVA${NC}"
else
    echo -e "${YELLOW}当前Java版本为 $CURRENT_JAVA${NC}"
    
    # 检查SDKMAN
    SDKMAN_DIR="$HOME/.sdkman"
    if [ -d "$SDKMAN_DIR" ]; then
        echo -e "${GREEN}检测到SDKMAN安装${NC}"
        # 加载SDKMAN
        export SDKMAN_DIR="$HOME/.sdkman"
        [[ -s "$SDKMAN_DIR/bin/sdkman-init.sh" ]] && source "$SDKMAN_DIR/bin/sdkman-init.sh"
        
        # 检查本地已安装的Java 8版本
        LOCAL_JAVA_8_VERSIONS=$(sdk list java | grep -E "8\.[0-9]+\.[0-9]+" | grep "installed" | awk '{print $NF}')
        if [ -n "$LOCAL_JAVA_8_VERSIONS" ]; then
            echo -e "${BLUE}检测到以下本地Java 8版本:${NC}"
            echo "$LOCAL_JAVA_8_VERSIONS"
            
            echo -e "${BLUE}是否切换到本地Java 8版本? (y/n)${NC}"
            read -p "" -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                # 使用第一个可用的本地Java 8版本
                JAVA_8_VERSION=$(echo "$LOCAL_JAVA_8_VERSIONS" | head -n 1)
                echo -e "${GREEN}正在切换到Java $JAVA_8_VERSION${NC}"
                sdk use java $JAVA_8_VERSION
                if [ $? -eq 0 ]; then
                    echo -e "${GREEN}成功切换到Java $JAVA_8_VERSION${NC}"
                else
                    echo -e "${RED}切换Java版本失败${NC}"
                    exit 1
                fi
            else
                echo -e "${YELLOW}继续使用当前Java版本${NC}"
            fi
        else
            echo -e "${YELLOW}本地未检测到Java 8版本${NC}"
            # 检查远程可用的Java 8版本
            REMOTE_JAVA_8_VERSIONS=$(sdk list java | grep -E "8\.[0-9]+\.[0-9]+" | grep -v "installed" | awk '{print $NF}')
            if [ -n "$REMOTE_JAVA_8_VERSIONS" ]; then
                echo -e "${BLUE}检测到以下可安装的Java 8版本:${NC}"
                echo "$REMOTE_JAVA_8_VERSIONS"
                
                echo -e "${BLUE}是否安装Java 8? (y/n)${NC}"
                read -p "" -n 1 -r
                echo
                if [[ $REPLY =~ ^[Yy]$ ]]; then
                    # 使用第一个可用的远程Java 8版本
                    JAVA_8_VERSION=$(echo "$REMOTE_JAVA_8_VERSIONS" | head -n 1)
                    echo -e "${GREEN}正在安装Java $JAVA_8_VERSION...${NC}"
                    sdk install java $JAVA_8_VERSION
                    if [ $? -eq 0 ]; then
                        echo -e "${GREEN}Java 8安装成功${NC}"
                        sdk use java $JAVA_8_VERSION
                    else
                        echo -e "${RED}Java 8安装失败${NC}"
                        exit 1
                    fi
                else
                    echo -e "${YELLOW}继续使用当前Java版本${NC}"
                fi
            else
                echo -e "${RED}未找到可用的Java 8版本${NC}"
                exit 1
            fi
        fi
    else
        echo -e "${YELLOW}未检测到SDKMAN安装${NC}"
        echo -e "${BLUE}是否安装SDKMAN? (y/n)${NC}"
        read -p "" -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${GREEN}正在安装SDKMAN...${NC}"
            curl -s "https://get.sdkman.io" | bash
            if [ $? -eq 0 ]; then
                echo -e "${GREEN}SDKMAN安装成功${NC}"
                echo -e "${BLUE}请重新运行此脚本${NC}"
                exit 0
            else
                echo -e "${RED}SDKMAN安装失败${NC}"
                exit 1
            fi
        else
            echo -e "${YELLOW}继续使用当前Java版本${NC}"
        fi
    fi
fi

# 检查Java版本
JAVA_VERSION=$(java -version 2>&1 | awk -F '"' '/version/ {print $2}')
if [[ $JAVA_VERSION != 1.8* ]]; then
    echo -e "${YELLOW}警告: 当前Java版本为 $JAVA_VERSION${NC}"
    echo -e "${YELLOW}建议使用Java 1.8版本进行测试${NC}"
    read -p "是否继续使用当前版本? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# 创建测试资源目录
mkdir -p resources/test

# 编译测试程序
echo -e "${GREEN}编译测试程序...${NC}"
javac -d resources/test test/TestProgram.java

# 运行测试
echo -e "${GREEN}运行测试...${NC}"
cargo test

# 检查测试结果
if [ $? -eq 0 ]; then
    echo -e "${GREEN}所有测试通过!${NC}"
else
    echo -e "${RED}测试失败${NC}"
    exit 1
fi 