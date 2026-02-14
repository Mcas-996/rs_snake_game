## Why

CI 发布工作流失败，因为 GitHub Actions 配置文件中的 Homebrew 仓库名称不正确，阻止了 v0.1.0 版本的自动发布。

## What Changes

- 修复 `.github/workflows/release.yml` 中的 Homebrew 仓库名称错误
- 在 `Cargo.toml` 中添加包描述以消除警告
- 重新生成 cargo-dist 配置以确保一致性

## Capabilities

### New Capabilities
- `ci-release-fix`: 修复 CI 发布流程中的配置错误和警告

### Modified Capabilities
- 无现有功能需求变更，仅修复发布配置

## Impact

- 影响自动发布流程和 Homebrew 包分发
- 修复后 cargo-dist 将能够成功构建和发布所有目标平台
- 解决 GitHub Actions 工作流中的配置不匹配问题
