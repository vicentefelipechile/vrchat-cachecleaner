[![Release](https://github.com/vicentefelipechile/vrchat-cachecleaner/actions/workflows/release.yml/badge.svg)](https://github.com/vicentefelipechile/vrchat-cachecleaner/actions/workflows/release.yml)

# VRChat Cache Cleaner

Una herramienta ligera y eficiente escrita en Rust que mantiene tu disco limpio borrando automáticamente la caché de VRChat cada vez que cierras el juego.

## 🚀 Características

- **Funcionamiento Automático**: Se ejecuta en segundo plano y detecta cuándo se cierra `VRChat.exe`.
- **Limpieza Profunda**: Borra automáticamente las siguientes carpetas de caché para liberar espacio:
  - `Cache-WindowsPlayer`
  - `HTTPCache-WindowsPlayer`
  - `TextureCache-WindowsPlayer`
- **Inicio con Windows**: Se configura automáticamente para iniciarse cuando enciendes tu PC.
- **Ligero**: Consume recursos mínimos del sistema mientras espera.

## 🛠️ Instalación y Uso

1. **Descargar/Compilar**: Obtén el ejecutable `vrchat-cachecleaner.exe`.
2. **Ejecutar**: Abre el programa una vez.
3. **Listo**: El programa se añadirá al inicio de Windows y comenzará a monitorear VRChat silenciosamente.

## 📦 Compilación

Si deseas compilarlo desde el código fuente, necesitas tener [Rust](https://www.rust-lang.org/) instalado.

```bash
git clone https://github.com/vicentefelipechile/vrchat-cachecleaner.git
cd vrchat-cachecleaner
cargo build --release
```

El ejecutable se generará en `target/release/vrchat-cachecleaner.exe`.
