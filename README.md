# jmake

**jmake** is a minimalistic, fast Java build tool.  
It supports incremental compilation, native JNI-based test execution, parallel builds, and a release cache system.

---

## 📦 Installation

1. Install Rust: https://www.rust-lang.org/tools/install

2. Clone and build:

    ```bash
    git clone https://github.com/lvzrr/jmake
    cd jmake
    cargo build --release
    ```

3. Add the compiled binary to your path:

    ```bash
    cp target/release/jmake ~/.local/bin/   # or any folder in your PATH
    ```

---

## 🚀 Usage

```bash
jmake [command] [target] [flags]
```

### Commands

- `init <package>`  
  Initialize a new Java package.

- `build <target>`  
  Compile Java files from `src/`.  
  Use `--release <MainClass>` to create a JAR.

- `run <MainClass> [args...]`  
  Run the given class using JNI from the `bin/` directory.

- `test <target>`  
  Compile and run test classes from `test/`.  
  It will look for classes like `<target>.TestsMain`.

- `test <target> --sandbox`  
  Run tests with restricted JVM options defined under `sandbox`.

- `clean`  
  Delete the contents of the configured `bin/` directory.

---

## 📊 Project Structure

Here’s what your project might look like:

> [!WARNING]
> jmake **must** be run from the root directory for correct behavior

```
project-root/
├── src/                 # [REQUIRED] Java source files
│   └── MyApp.java
├── test/                # [REQUIRED] Test source files
│   └── MyAppTests.java
├── bin/                 # [REQUIRED] Compiled output
├── lib/                 # [OPTIONAL] .class or .jar dependencies
│   ├── Helper.class
│   └── external.jar
├── jmake.toml           # [OPTIONAL] Configuration file
└── scripts/             # [OPTIONAL] Pre/Post build hooks
    ├── precompile.sh
    └── cleanup.sh
```

---

## 💡 Examples

```bash
jmake init mypkg
jmake build mypkg
jmake build mypkg --release mypkg.Main
jmake run mypkg.Main arg1 arg2
jmake test testpkg
jmake test testpkg --sandbox
jmake clean
```

---

## ✨ Features

- ✅ Incremental compilation with smart dependency tracking  
- ✅ Native & multi-threaded JVM execution via JNI  
- ✅ Optional sandbox mode for restricted memory/stack test environments  
- ✅ Classpath expansion: handles `lib/*`, `.jar` and `.class` files  
- ✅ Lightweight: compiled binary is under **1MB**  
- ✅ Cross-platform support: Windows, Linux, macOS  
- ✅ Configurable PRE and POST build steps  
- ✅ Efficient JAR release system using hash-based caching  

---

## 🛠 Configuration: `jmake.toml`

You can configure all options in a `jmake.toml` file placed at the root.

```toml
src = "src"
test = "test"
lib = "lib"
bin = "bin"
cache = "~/.cache/jmake"
classpath = "bin:lib/*"
comp_flags = "-g"
threads = 4
jvm_version = "8"

pre = ["./scripts/precompile.sh", "echo compiling..."]
post = ["./scripts/cleanup.sh", "echo done."]
jvm_options = ["-Xmx512m"]
run_args = ["arg1", "arg2"]

# Will override jvm_options when running tests with --sandbox
sandbox = ["-Xmx64m", "-Xss256k"]
```

You may also write multiline arrays:

```toml
pre = [
  "echo hello",
  "sh runme.sh"
]
```

Quotes (`"` or `'`) around strings are optional and trimmed automatically.

---

## 📎 Notes

- If no target is specified for `build`, all `*.java` in `src/` are compiled.  
- Files in `lib/` that are not part of a package cannot be imported; use them as raw dependencies.  
- Classpath entries like `"lib/*"` are auto-expanded during both compile and run.  
- When running with `--sandbox`, `jvm_options` is replaced by the `sandbox` config for tighter resource control.  

---

## 💼 License

MIT License — see the [LICENSE](./LICENSE) file.
