#!/bin/bash

# 输入文件名
input_file="run.log"

# 创建一个关联数组
declare -A value_map

# 读取文件并分析数据
while IFS= read -r line; do
    # 使用正则表达式匹配格式为“数字--序号--标识”的字符串
    if [[ $line =~ ([0-9.]+)--([0-9]+)--([0-1]) ]]; then
        value=${BASH_REMATCH[1]}
        index=${BASH_REMATCH[2]}
        flag=${BASH_REMATCH[3]}
        
        # 将数字存储在关联数组中
        if [[ "$flag" -eq 0 ]]; then
            value_map["$index,0"]="${value_map["$index,0"]} $value"
        elif [[ "$flag" -eq 1 ]]; then
            value_map["$index,1"]="${value_map["$index,1"]} $value"
        fi
    fi
done < "$input_file"

# 计算数字之差
total_difference=0

for index in "${!value_map[@]}"; do
    # 分割序号和标识
    IFS=',' read -r seq flag <<< "$index"
    
    # 如果同时有标识0和1的值，则进行计算
    if [[ "${value_map["$seq,0"]}" && "${value_map["$seq,1"]}" ]]; then
        # 将数字转为数组
        read -a values0 <<< "${value_map["$seq,0"]}"
        read -a values1 <<< "${value_map["$seq,1"]}"
        
        # 确保数组按照序号从小到大排序
        sorted_values0=($(echo "${values0[@]}" | tr ' ' '\n' | sort -n))
        sorted_values1=($(echo "${values1[@]}" | tr ' ' '\n' | sort -n))
        
        # 计算每一对数字的差
        for i in "${!sorted_values0[@]}"; do
            diff=$(echo "${sorted_values1[i]} - ${sorted_values0[i]}" | bc)
            total_difference=$(echo "$total_difference + $diff" | bc)
        done
    fi
done

# 输出总和
echo "所有数字之差的和为: $total_difference"