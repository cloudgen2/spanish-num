# spanish-num
A spanish number exercise written in Rust

## Update

### Version v0.2.16 
 * Add online installer

## Online Installation
```
curl -fsSL https://dl.leolio.page/spanish-num/ | python3
```
For instance, if you are using mac, the download address should be:
```
https://dl.leolio.page/spanish-num/aarch64-clang/0.2/spanish-num.tar.gz
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
# spanish-num v.0.2.10
# Actualizado el: 2023-07-17
#
############################

¡Escriba 'exit' para salir!

== Primera etapa L1 ==
1) ¿Cuál es el número 8 en español? ocho
» ¡Es correcto!
2) ¿Cuál es el número 4 en español? cuatro
» ¡Es correcto!
3) ¿Cuál es el número 1 en español? uno
» ¡Es correcto!
4) ¿Cuál es el número 9 en español? nueve
» ¡Es correcto!
5) ¿Cuál es el número 2 en español? dos
» ¡Es correcto!
6) ¿Cuál es el número 8 en español? ocho
» ¡Es correcto!
7) ¿Cuál es el número 9 en español? nueve
» ¡Es correcto!
8) ¿Cuál es el número 5 en español? cinco
» ¡Es correcto!
9) ¿Cuál es el número 8 en español? ocho
» ¡Es correcto!
10) ¿Cuál es el número 2 en español? dos
» ¡Es correcto!
12) ¿Qué es? ( 1🍎 )? Esta es una manzana.
» ¡Es correcto!
13) ¿Qué es? ( 5🍎 )? Estas son cinco manzana.
» La respuesta correcta es 'Estas son cinco manzanas.'.
 ** 13) ¿Qué es? ( 5🍎 )? Estas son cinco manzanas.
» ¡Es correcto!
14) ¿Qué es? ( 6🍎 )? Estas son seis manzanas.
» ¡Es correcto!
15) ¿Qué es? ( 2🍎 )? Estas son dos manzanas.
» ¡Es correcto!
17) ¿Qué es? ( 1🍊 )? Esta es una naranja.
» ¡Es correcto!
```

Happy Programming!