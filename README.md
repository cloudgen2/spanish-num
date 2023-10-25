# spanish-num
A spanish number exercise written in Rust. This project uses the online-downloader script in https://github.com/cloudgen2/online-installer

## Online Installation
```
curl -fsSL https://dl.leolio.page/spanish-num/ | python3
```
For instance, if you are using mac, the download address should be:
```
https://dl.leolio.page/spanish-num/aarch64-clang/0.4/spanish-num.tar.gz
```

## Run the source code
```
cargo run
```

## Build Release
```
cargo build --release
rm -rf ~/.local/bin/spanish-num
cp target/release/spanish-num  ~/.local/bin/
```

## Execution example

```
############################
#
# spanish-num v.0.4.0
# Actualizado el: 2023-10-14
#
############################

¡Escriba 'exit' para salir!

== Primera etapa L1 ==
1) ¿Cuál es el número 3 en español? tres
» ¡Es correcto!
2) ¿Cuál es el número 5 en español? cinco
» ¡Es correcto!
3) ¿Cuál es el número 2 en español? dos
» ¡Es correcto!
4) ¿Cuál es el número 7 en español? siete
» ¡Es correcto!
5) ¿Cuál es el número 7 en español? siete
» ¡Es correcto!
6) ¿Cuál es el número 2 en español? dos
» ¡Es correcto!
7) ¿Cuál es el número 10 en español? diez
» ¡Es correcto!
8) ¿Cuál es el número 1 en español? uno
» ¡Es correcto!
9) ¿Cuál es el número 9 en español? nueve
» ¡Es correcto!
10) ¿Cuál es el número 9 en español? nueve
» ¡Es correcto!
12) ¿Qué es? ( 1🍎 )? Esta es una manzana.
» ¡Es correcto!
```

Happy Programming!
