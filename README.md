# 开发指南

本指南将帮助你开始使用这个模板开发 Rust Python 扩展。

## 目录

- [快速开始](#快速开始)
- [项目结构](#项目结构)
- [开发工作流](#开发工作流)
- [PyO3 常用模式](#pyo3-常用模式)
- [类型转换](#类型转换)
- [错误处理](#错误处理)
- [性能优化](#性能优化)
- [调试技巧](#调试技巧)
- [常见问题](#常见问题)

## 快速开始

### 1. 使用模板创建新项目

```bash
# 安装 cargo-generate（如果还没有）
cargo install cargo-generate

# 从模板创建新项目
cargo generate --git https://github.com/your-org/pyo3-extension-template
```

### 2. 开发环境设置

```bash
# 进入项目目录
cd your-project-name

# 安装开发依赖
pip install maturin pytest

# 以开发模式安装（自动重新编译）
maturin develop
```

### 3. 运行测试

```bash
# 运行 Python 测试
pytest tests/

# 或使用 tox
tox -e py
```

## 项目结构

```
your-project/
├── src/
│   └── lib.rs          # Rust 代码和 Python 绑定
├── tests/
│   └── test_*.py       # Python 测试文件
├── Cargo.toml          # Rust 包配置
├── pyproject.toml      # Python 包配置
├── README.md           # 项目说明
└── BUILD.md            # 构建指南
```

## 开发工作流

### 典型的开发循环

1. **修改 Rust 代码** (`src/lib.rs`)
2. **重新编译**：
   ```bash
   maturin develop
   # 或使用 --release 进行优化构建
   maturin develop --release
   ```
3. **运行测试**：
   ```bash
   pytest tests/
   ```
4. **在 Python 中测试**：
   ```python
   import your_module
   # 测试你的代码
   ```

### 热重载开发

对于快速迭代，可以使用 `maturin develop`，它会：
- 自动检测 Rust 代码变化
- 重新编译扩展
- 更新 Python 环境中的模块

## PyO3 常用模式

### 1. 导出函数

```rust
use pyo3::prelude::*;

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pymodule]
fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
```

### 2. 导出类

```rust
#[pyclass]
struct MyClass {
    value: i32,
}

#[pymethods]
impl MyClass {
    #[new]
    fn new(value: i32) -> Self {
        MyClass { value }
    }
    
    fn get_value(&self) -> i32 {
        self.value
    }
    
    fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

#[pymodule]
fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    Ok(())
}
```

### 3. 接受 Python 对象

```rust
use pyo3::types::PyList;

#[pyfunction]
fn process_list(list: &Bound<'_, PyList>) -> PyResult<usize> {
    Ok(list.len())
}
```

### 4. 返回 Python 对象

```rust
#[pyfunction]
fn create_list(py: Python) -> PyResult<Bound<'_, PyList>> {
    let list = PyList::empty(py);
    list.append(1)?;
    list.append(2)?;
    list.append(3)?;
    Ok(list)
}
```

## 类型转换

### Rust 到 Python

| Rust 类型 | Python 类型 |
|-----------|-------------|
| `i32`, `i64` | `int` |
| `f32`, `f64` | `float` |
| `bool` | `bool` |
| `String`, `&str` | `str` |
| `Vec<T>` | `list` |
| `HashMap<K, V>` | `dict` |
| `Option<T>` | `T` 或 `None` |

### Python 到 Rust

使用 `FromPyObject` trait：

```rust
use pyo3::prelude::*;

#[pyfunction]
fn process(value: i32) -> i32 {
    value * 2
}

// 接受多种类型
#[pyfunction]
fn flexible_add(a: i32, b: i32) -> i32 {
    a + b
}
```

## 错误处理

### 抛出 Python 异常

```rust
use pyo3::exceptions::PyValueError;

#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyValueError::new_err("Division by zero"));
    }
    Ok(a / b)
}
```

### 自定义异常

```rust
use pyo3::create_exception;

create_exception!(my_module, MyCustomError, pyo3::exceptions::PyException);

#[pyfunction]
fn my_function() -> PyResult<()> {
    Err(MyCustomError::new_err("Something went wrong"))
}
```

## 性能优化

### 1. 使用 Release 模式

```bash
maturin develop --release
```

### 2. 避免不必要的 Python GIL 操作

```rust
// 不好的做法：频繁获取 GIL
fn slow_function(py: Python) -> PyResult<i32> {
    // 多次 Python 调用
}

// 好的做法：批量处理
fn fast_function() -> i32 {
    // 纯 Rust 计算
    // 只在最后返回结果
}
```

### 3. 使用 `PyO3` 的零成本抽象

```rust
// 使用 Bound 而不是 &PyAny 可以避免运行时检查
fn process<'py>(obj: &Bound<'py, PyAny>) -> PyResult<()> {
    // ...
}
```

## 调试技巧

### 1. 打印调试信息

```rust
#[pyfunction]
fn debug_function(value: i32) -> PyResult<()> {
    println!("Debug: value = {}", value);
    Ok(())
}
```

### 2. 使用 Python 的 logging

```rust
use pyo3::types::PyModule;

#[pyfunction]
fn log_message(py: Python, message: &str) -> PyResult<()> {
    let logging = PyModule::import(py, "logging")?;
    let logger = logging.getattr("getLogger")?.call0()?;
    logger.call_method1("info", (message,))?;
    Ok(())
}
```

### 3. 使用 Rust 调试器

```bash
# 编译调试版本
maturin develop

# 使用 gdb/lldb
gdb python
(gdb) run -c "import my_module; my_module.my_function()"
```

## 常见问题

### Q: 编译错误：找不到 Python

**A:** 安装 Python 开发头文件：
```bash
# Ubuntu/Debian
sudo apt-get install python3-dev

# Fedora
sudo dnf install python3-devel

# macOS
# 通常已包含在 Xcode Command Line Tools 中
```

### Q: 导入模块时出错

**A:** 确保：
1. `Cargo.toml` 中的 `lib.name` 与 `#[pymodule]` 函数名匹配
2. 模块名使用下划线而不是连字符
3. 已正确运行 `maturin develop`

### Q: 类型转换失败

**A:** 检查：
1. Rust 函数签名是否正确
2. Python 传入的类型是否匹配
3. 是否使用了 `PyResult` 处理错误

### Q: 性能不如预期

**A:** 尝试：
1. 使用 `--release` 模式编译
2. 减少 Python GIL 操作
3. 使用 Rust 的迭代器而不是 Python 循环

## 更多资源

- [PyO3 官方文档](https://pyo3.rs/)
- [PyO3 用户指南](https://pyo3.rs/latest/)
- [Maturin 文档](https://maturin.rs/)
- [Rust 官方文档](https://doc.rust-lang.org/)
