# borealis-rs
rust reimplement of https://github.com/natinusala/borealis


## mingw64

```shell
pacman -S --needed base-devel mingw-w64-x86_64-toolchain
pacman -S mingw-w64-x86_64-glfw
```

```shell
export CC=gcc  
export CXX=g++
```

## clang64

```shell
pacman -S --needed base-devel mingw-w64-clang-x86_64-toolchain
pacman -S mingw-w64-clang-x86_64-rust
#pacman -S mingw-w64-x86_64-libc++
```

```shell
export CC=clang  
export CXX=clang++
```