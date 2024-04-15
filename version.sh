#!/usr/bin/env bash

# https://crates.io/crates/cargo-release 更新不同级别版本号
# https://crates.io/crates/cargo-edit 更新依赖
# https://github.com/crate-ci/cargo-release/blob/master/docs/reference.md

# 检查 cargo-release 是否存在
command -v cargo-release >/dev/null 2>&1 || {
    echo "cargo-release 命令未找到，请先安装."
    exit 1
}

# 检查 git 是否存在
command -v git >/dev/null 2>&1 || {
    echo "git 命令未找到，请先安装."
    exit 1
}

# 检查是否有未提交的改动
if ! git diff-index --quiet HEAD --; then
    echo "检测到未提交的改动，请先完成 Git 提交."
    exit 1
fi

# 设置默认级别，可以是 major、minor 或 patch
default_level="patch"

# 读取从命令行传递的参数 (如果有的话)
release_level=$1


# 如果没有提供参数，就使用默认级别
if [ -z "$release_level" ]; then
    release_level=$default_level
    echo "未指定发布级别号，将使用默认设置：$default_level."
fi

# 确定提供的级别是否有效
case $release_level in
    major|minor|patch)
        # 使用 cargo-release 来获取即将发布的版本号，但暂不实际发布
        current_version=$(grep '^version = ".*"' Cargo.toml | head -n1 | sed -E 's/version = "(.*)"/\1/')
        echo "Current version: $current_version"

        # 根据发布级别计算下一个版本
        IFS='.' read -r -a version_parts <<< "$current_version"
        major=${version_parts[0]}
        minor=${version_parts[1]}
        patch=${version_parts[2]}

        case $release_level in
          major)
            ((major+=1))
            minor=0
            patch=0
            ;;
          minor)
            ((minor+=1))
            patch=0
            ;;
          patch)
            ((patch+=1))
            ;;
        esac
        new_version="$major.$minor.$patch"
        cargo release $release_level --no-push --no-publish --no-confirm -x --no-tag
        git reset --soft HEAD^
        npm pkg set version=$new_version
        git add Cargo.toml package.json
        git commit -m "chore: release v$new_version"
        git tag -a "$new_version" -m "chore: release v$new_version"
        git push && git push --tags
        ;;
    *)
        # 如果提供了无效的级别，则退出
        echo "错误：无效的发布级别 '$release_level'."
        exit 2
        ;;
esac