#!/usr/bin/env bash

# set -eo pipefail # https://stackoverflow.com/a/2871034

cd pkg

# PREFIXNAME=@wlintt/
# NAME=$(npm pkg get name | tr -d '"')

# if [[ $NAME == $PREFIXNAME* ]]; then
#     break
#   else
    # npm pkg set name="${PREFIXNAME}${NAME}"
#   fi
# npm pkg set name="${PREFIXNAME}${NAME}"

FILENAME='snippets/'

# FILESARRAY=$(npm pkg get files)
# # 将得到的 JSON 字符串转换为 bash 数组
# # 用 tr 删除引号和逗号，用 grep -v 过滤掉不需要的行
# # 并使用 while read 循环代替 readarray
# IFS=$'\n' # 设置 IFS 为换行符，以保证文件名正确地分割
# FILES=($(echo $FILESARRAY | tr -d '[],"' | grep -v '^$'))
# # 在此issue未解决前，手动处理 https://github.com/rustwasm/wasm-pack/issues/1206
# # 检查数组中的项
# for ITEM in "${FILES[@]}"; do
#   if [[ "$ITEM" == *"$FILENAME"* ]]; then
#     # echo "Found file that starts with '$FILENAME': $item"
#     # 如果找到匹配的项，可以在此处退出或做其他处理
#     # break # 如果只是break，则后续代码还会执行
#     # exit # 如果使用exit，则脚本执行到此结束
#     exit
#   else
#     npm pkg set files[]=$FILENAME
#   fi
# done
npm pkg set files[]=$FILENAME
