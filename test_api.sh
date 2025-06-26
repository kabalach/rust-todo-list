#!/bin/bash

# Todo List API 测试脚本
# 使用方法: chmod +x test_api.sh && ./test_api.sh

API_BASE="http://localhost:3000"
echo "🚀 开始测试 Todo List API..."
echo "================================================"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试函数
test_endpoint() {
    local method=$1
    local url=$2
    local data=$3
    local description=$4
    
    echo -e "\n${YELLOW}测试: $description${NC}"
    echo "请求: $method $url"
    
    if [ -n "$data" ]; then
        echo "数据: $data"
        response=$(curl -s -w "\n%{http_code}" -X "$method" "$url" \
            -H "Content-Type: application/json" \
            -H "Accept: application/json" \
            -d "$data")
    else
        response=$(curl -s -w "\n%{http_code}" -X "$method" "$url" \
            -H "Accept: application/json")
    fi
    
    # 分离响应体和状态码
    body=$(echo "$response" | head -n -1)
    status_code=$(echo "$response" | tail -n 1)
    
    if [ "$status_code" -ge 200 ] && [ "$status_code" -lt 300 ]; then
        echo -e "${GREEN}✅ 成功 (HTTP $status_code)${NC}"
        echo "响应: $body" | jq . 2>/dev/null || echo "响应: $body"
    else
        echo -e "${RED}❌ 失败 (HTTP $status_code)${NC}"
        echo "响应: $body"
    fi
}

# 1. 测试根路径
echo -e "\n${YELLOW}1. 测试根路径和健康检查${NC}"
echo "访问: $API_BASE"
curl -s -o /dev/null -w "HTTP状态码: %{http_code}\n" "$API_BASE"

test_endpoint "GET" "$API_BASE/health" "" "健康检查"

# 2. 测试获取所有任务（应该为空）
test_endpoint "GET" "$API_BASE/api/tasks" "" "获取所有任务（初始）"

# 3. 创建第一个任务
task1_data='{"title": "学习 Rust", "description": "完成 Rust 官方教程"}'
test_endpoint "POST" "$API_BASE/api/tasks" "$task1_data" "创建第一个任务"

# 4. 创建第二个任务
task2_data='{"title": "构建 API", "description": "使用 Axum 框架构建 RESTful API"}'
test_endpoint "POST" "$API_BASE/api/tasks" "$task2_data" "创建第二个任务"

# 5. 获取所有任务
echo -e "\n${YELLOW}5. 获取所有任务${NC}"
all_tasks=$(curl -s "$API_BASE/api/tasks")
echo "响应: $all_tasks" | jq . 2>/dev/null || echo "响应: $all_tasks"

# 提取第一个任务的ID用于后续测试
task_id=$(echo "$all_tasks" | jq -r '.data[0].id' 2>/dev/null)

if [ "$task_id" != "null" ] && [ -n "$task_id" ]; then
    echo -e "${GREEN}✅ 找到任务ID: $task_id${NC}"
    
    # 6. 获取特定任务
    test_endpoint "GET" "$API_BASE/api/tasks/$task_id" "" "获取特定任务"
    
    # 7. 更新任务（标记为完成）
    update_data='{"completed": true, "title": "学习 Rust（已更新）"}'
    test_endpoint "PUT" "$API_BASE/api/tasks/$task_id" "$update_data" "更新任务状态"
    
    # 8. 再次获取该任务确认更新
    test_endpoint "GET" "$API_BASE/api/tasks/$task_id" "" "确认任务更新"
    
    # 9. 删除任务（可选，注释掉以保留数据）
    # echo -e "\n${YELLOW}是否删除测试任务？ (y/N)${NC}"
    # read -r delete_confirm
    # if [ "$delete_confirm" = "y" ] || [ "$delete_confirm" = "Y" ]; then
    #     test_endpoint "DELETE" "$API_BASE/api/tasks/$task_id" "" "删除任务"
    # fi
else
    echo -e "${RED}❌ 无法获取任务ID，跳过单个任务测试${NC}"
fi

# 10. 最终获取所有任务
test_endpoint "GET" "$API_BASE/api/tasks" "" "最终任务列表"

echo -e "\n================================================"
echo -e "${GREEN}🎉 API 测试完成！${NC}"
echo -e "\n${YELLOW}💡 提示：${NC}"
echo "• 访问 $API_BASE 查看欢迎页面"
echo "• 访问 $API_BASE/health 查看健康状态"
echo "• 访问 $API_BASE/api/tasks 查看所有任务"
echo "• 使用上面的测试页面进行交互式测试"

echo -e "\n${YELLOW}📊 快速统计：${NC}"
curl -s "$API_BASE/api/tasks" | jq -r '.data | length' 2>/dev/null | xargs -I {} echo "总任务数: {}"
curl -s "$API_BASE/api/tasks" | jq -r '.data | map(select(.completed == true)) | length' 2>/dev/null | xargs -I {} echo "已完成: {}"
curl -s "$API_BASE/api/tasks" | jq -r '.data | map(select(.completed == false)) | length' 2>/dev/null | xargs -I {} echo "待完成: {}"