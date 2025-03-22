# jmake

**jmake** is a minimalistic, fast Java build tool. It supports incremental compilation, native JNI-based test execution, parallel builds, and a cache system for releases.

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

- `clean`  
  Delete the contents of the configured `bin/` directory.

---

## 📊 Project Structure

Here's a typical file layout jmake works with. 
**Required folders** (you can customize them in the jmakefile) are:
- `src/` (source files)
- `bin/` (compiled output)
- `test/` (test sources)

(All created with `jmake init`)

Everything else is optional:

> [!WARNING]
> jmake MUST be run in the root for it to work properly
```
project-root/
├── src/                 # [REQUIRED] Java source files
│   └── MyApp.java
├── test/                # [REQUIRED] Test source files
│   └── MyAppTests.java
├── bin/                 # [REQUIRED] Compiled classes output
├── lib/                 # [OPTIONAL] External .class or .jar dependencies
│   ├── helper.class
│   └── utils.jar
├── jmakefile            # [OPTIONAL] Configuration file
└── scripts/             # [OPTIONAL] Pre/Post build scripts
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
jmake clean
```

---

## ✨ Features

- ✓ Incremental compilation  
- ✓ Native & multi-threaded JVM execution using the JNI  
- ✓ Auto-expanded classpath with JAR and `.class` support  
- ✓ Lightweight: binaries of **~500kB**  
- ✓ Cross-platform support (Windows, Linux, macOS)  
- ✓ Pre/Post build command hooks  
- ✓ Release system with content-hash caching  

---

## 📝 Configuration (JMakefile)

You can customize jmake with a `jmakefile` in the project root. These are the default assumed if no jmakefile is provided:

```jmakefile
src='src'
test='test'
lib='lib'
bin='bin'
classpath='bin:lib:lib/*'
comp_flags=''
cache='~/.cache/jmake'
jvm_options=''
threads='4'

//Actually threads will calculate the max amount 
//allowed for your system by default

pre={
    // './scripts/precompile.sh',
    // 'echo preparing environment...'
};

post={
    // './scripts/cleanup.sh',
    // 'echo done.'
};
```

---

## 📝 Notes

- If you do not specify a package to `init` or `build`, it will just look for all `*.java` files under `src/`.  
- Java classes in `lib/` without a package cannot be imported — just use them directly.  
- jmake will automatically expand `"lib/*"` to include all `.jar`s and include `"lib/"` for `.class` files.  
- You can configure `classpath`, `threads`, `jvm_options`, and more in a `jmakefile`.  

---

## 💼 License

MIT License. See the [LICENSE](./LICENSE) file.

