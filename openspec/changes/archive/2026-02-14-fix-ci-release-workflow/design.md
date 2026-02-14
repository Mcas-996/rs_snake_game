## Context

当前 CI 发布工作流在 cargo-dist 检查阶段失败，发现两个主要问题：
1. `.github/workflows/release.yml` 中的 Homebrew 仓库名称错误（`Mcas9-99996/homebrew-tap` 应为 `Mcas-996/homebrew-tap`）
2. `Cargo.toml` 缺少包描述，导致 Homebrew 发布警告

这些配置不匹配阻止了 v0.1.0 版本的自动发布流程。

## Goals / Non-Goals

**Goals:**
- 修复 GitHub Actions 工作流中的 Homebrew 仓库名称错误
- 添加包描述以消除 cargo-dist 警告
- 确保 cargo-dist 配置文件与实际设置保持一致
- 恢复自动发布流程的正常运行

**Non-Goals:**
- 修改发布流程的核心逻辑
- 更改目标平台或安装程序配置
- 修改版本号或发布策略

## Decisions

- **使用 `dist init` 重新生成配置**: 这是最安全的方法，确保所有配置文件与当前 cargo-dist 版本兼容
- **手动修复关键错误**: 对于简单的配置错误，直接修复比完全重新生成更高效
- **保持现有配置**: 不修改目标平台、安装程序类型等核心设置，只修复导致失败的配置项

## Risks / Trade-offs

- **配置覆盖风险**: 使用 `dist init` 可能覆盖现有自定义配置 → 先备份现有配置，仅应用必要更改
- **版本兼容性**: cargo-dist 版本更新可能引入不兼容的配置变更 → 验证生成的配置与项目需求匹配
- **发布延迟**: 修复配置需要重新触发 CI → 修复后立即推送新标签以重新启动发布流程
