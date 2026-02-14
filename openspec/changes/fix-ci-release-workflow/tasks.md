## 1. Configuration Analysis

- [ ] 1.1 备份当前 GitHub Actions 工作流文件
- [ ] 1.2 验证 Homebrew 仓库名称错误的具体位置
- [ ] 1.3 检查 Cargo.toml 中缺失的包描述

## 2. Fix Configuration Errors

- [ ] 2.1 修复 release.yml 中的 Homebrew 仓库名称（Mcas9-99996 → Mcas-996）
- [ ] 2.2 在 Cargo.toml 中添加包描述字段
- [ ] 2.3 验证 dist-workspace.toml 配置正确性

## 3. Validation and Testing

- [ ] 3.1 运行 `cargo dist plan` 验证配置修复
- [ ] 3.2 检查所有目标平台配置是否正确
- [ ] 3.3 验证 Homebrew 发布配置完整性

## 4. Release Process Recovery

- [ ] 4.1 删除有问题的 v0.1.0 标签
- [ ] 4.2 推送修复后的配置到远程仓库
- [ ] 4.3 重新创建并推送 v0.1.0 标签以触发发布
