# Changelog

## [Unreleased]
### Changed
- **miniapp**: 按照分包策略抽取独立页面至子工作路径 `subpkg` 下以缩减小程序主包打包体积。主要涉及页面包含 `course, score, exam, elective, plan, dekt`。重构了 `pages.json` 分离配置并修复 `index.vue` 入口路由。
