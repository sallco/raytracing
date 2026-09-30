# Raytracing

Diorama 3D renderizado mediante raytracing por CPU para el curso de Gráficas.
La escena utilizará cubos texturizados y efectos de iluminación implementados
desde cero; `raylib` se limita a ventana, entrada y carga de recursos.

## Entorno

- Rust 1.98 o posterior
- Arch Linux / EndeavourOS
- `raylib` 5.5.1

## Ejecutar

```bash
cargo run
```

Para las capturas y el video final, usar el perfil optimizado:

```bash
cargo run --release
```

El perfil de desarrollo usa optimización moderada para conservar una iteración
fluida. El perfil de lanzamiento habilita optimización máxima y LTO fino para
priorizar el rendimiento del renderizador y aspirar a una experiencia cercana a
60 FPS, según la complejidad de la escena y la resolución elegida.

## Verificación

```bash
cargo fmt --check
cargo check
cargo build --release
```
