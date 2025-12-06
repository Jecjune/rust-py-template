# 构建 pip 包指南

本项目使用 `maturin` 作为构建后端，有多种方法可以构建 pip 包（wheel 和 sdist）。

## 方法 1: 使用 maturin（推荐）

### 安装 maturin

```bash
pip install maturin
```

或者使用 pipx（推荐，避免污染环境）：

```bash
pipx install maturin
```

### 构建 wheel 包

```bash
# 在项目根目录（template/）运行
maturin build
```

这会构建：
- **wheel 文件**（.whl）：位于 `target/wheels/` 目录
- **源码分发包**（.tar.gz）：位于 `target/wheels/` 目录

### 构建选项

```bash
# 使用 release 模式构建（优化版本，构建时间较长）
maturin build --release

# 只构建 wheel，不构建 sdist
maturin build --no-sdist

# 只构建 sdist，不构建 wheel
maturin build --sdist-only

# 指定 Python 版本
maturin build --python 3.10

# 为特定平台构建（交叉编译）
maturin build --target x86_64-unknown-linux-gnu
```

### 构建结果位置

构建完成后，wheel 文件位于：
```
target/wheels/pyo3_extension-0.1.0-*.whl
```

## 方法 2: 使用 python -m build（标准方法）

这是 Python 标准的构建方法，底层会调用 maturin。

### 安装 build 工具

```bash
pip install build
```

### 构建包

```bash
# 构建 wheel 和 sdist
python -m build

# 只构建 wheel
python -m build --wheel

# 只构建 sdist
python -m build --sdist
```

### 构建结果位置

构建完成后，文件位于：
```
dist/pyo3_extension-0.1.0-*.whl
dist/pyo3_extension-0.1.0.tar.gz
```

## 方法 3: 使用 pip install（开发模式）

这不是构建包，而是直接安装到当前 Python 环境，适合开发使用：

```bash
# 开发模式安装（debug 模式，构建快）
maturin develop

# 使用 release 模式安装（优化版本）
maturin develop --release
```

## 安装构建的包

构建完成后，可以使用 pip 安装：

```bash
# 安装 wheel 文件
pip install target/wheels/pyo3_extension-0.1.0-*.whl

# 或者从 dist 目录安装（如果使用 python -m build）
pip install dist/pyo3_extension-0.1.0-*.whl

# 从本地目录安装（pip 会自动构建）
pip install .

# 从本地目录安装（可编辑模式，开发用）
pip install -e .
```

## 验证构建的包

```bash
# 检查 wheel 文件
pip show pyo3_extension

# 或者使用 twine 检查
pip install twine
twine check dist/*

# 测试安装
python -c "import pyo3_extension; print(pyo3_extension.sum_as_string(2, 3))"
```

## 发布到 PyPI

### 1. 构建包

```bash
maturin build --release
# 或
python -m build
```

### 2. 检查包

```bash
pip install twine
twine check dist/*
```

### 3. 上传到 TestPyPI（测试）

```bash
twine upload --repository testpypi dist/*
```

### 4. 上传到 PyPI（正式）

```bash
twine upload dist/*
```

**注意**：上传到 PyPI 前，确保：
- 已更新版本号（在 `pyproject.toml` 和 `Cargo.toml` 中）
- 已配置 PyPI 认证（使用 API token 或 trusted publisher）
- 已测试包的功能

## 多平台构建

### 使用 cibuildwheel（CI/CD）

项目包含 GitHub Actions 工作流，可以自动为多个平台构建 wheel。

### 本地交叉编译

```bash
# 安装交叉编译工具链
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin

# 构建特定目标
maturin build --target x86_64-pc-windows-gnu
```

## 常见问题

### 问题：构建失败，提示找不到 Python

**解决**：确保已安装 Python 开发头文件：

```bash
# Ubuntu/Debian
sudo apt-get install python3-dev

# Fedora
sudo dnf install python3-devel

# macOS
# 通常已包含在 Xcode Command Line Tools 中
```

### 问题：构建速度慢

**解决**：开发时使用 debug 模式，发布时使用 release 模式：

```bash
# 开发（快速）
maturin develop

# 发布（优化）
maturin build --release
```

### 问题：wheel 文件太大

**解决**：使用 release 模式构建，并确保启用了优化：

```bash
maturin build --release
```

## 快速参考

```bash
# 开发模式安装（最快）
maturin develop

# 构建 wheel 包
maturin build

# 构建优化版本
maturin build --release

# 使用标准工具构建
python -m build

# 安装构建的包
pip install target/wheels/pyo3_extension-*.whl
```

