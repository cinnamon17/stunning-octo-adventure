# Horarios paradas de bus 

Horarios de llegada de buses Salamanca de Transportes

## Descripción 

Aplicación que extrae horarios de llegada de los buses de Salamanca de Transportes,
es una aplicación de terminal que hace peticiones en directo hacia la página oficial
en tiempo real y muestra los resultado en la gui de la terminal

## Requerimientos

Instalación de OpenSSl 

~~~
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
~~~

Instalación de Rust

~~~
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
~~~

Clonar repositorio

~~~
git clone https://github.com/cinnamon17/stunning-octo-adventure.git
cd stunning-octo-adventure
~~~

## Compilación y ejecución

Debug 

~~~
cargo run
~~~

Release

~~~
cargo build --release
~~~
