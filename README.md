# Raytracing

Diorama 3D renderizado mediante raytracing por CPU para el curso de Gráficas.
La primera versión presenta un sistema solar estático formado por 5173 cubos de
superficie. `raylib` se limita a ventana, entrada, texto y presentación del
framebuffer; la cámara, intersecciones, iluminación y efectos se calculan en CPU.

La escena contiene el Sol, los ocho planetas, anillos de Saturno, un cinturón de
asteroides, terreno lunar procedural y una estación espacial. Incluye cielo
estelar procedural, luz solar directa, ambiente tenue, sombras, reflexión y
refracción con un máximo de dos rebotes secundarios.

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

## Controles

- Arrastrar con el botón izquierdo: orbitar la cámara.
- Rueda del ratón: acercar o alejar.
- Flechas izquierda y derecha: seleccionar el elemento anterior o siguiente.
- `Esc`: regresar a la vista general.

El HUD muestra los FPS, el tiempo del último raytrace y, al enfocar un elemento,
su nombre, tipo, cantidad de cubos y material predominante.

## Medir rendimiento

El modo de medición genera un cuadro completo de `960×540` sin abrir una ventana:

```bash
cargo run --release -- --benchmark
```

Los cuadros inmóviles se reutilizan para mantener fluida la presentación. La
escena vuelve a trazarse cuando cambia la cámara o durante una transición de
enfoque.

## Arquitectura

- Solo se almacenan voxeles expuestos de cada cuerpo.
- Los voxeles se agrupan en chunks locales de `8×8×8`.
- Cada rayo descarta cuerpos por AABB y recorre sus chunks mediante DDA.
- El framebuffer se divide en hasta 12 franjas usando `std::thread::scope`.
- Los cuerpos permanecen en coordenadas locales para facilitar transformaciones
  posteriores.

El perfil de desarrollo usa optimización moderada para conservar una iteración
fluida. El perfil de lanzamiento habilita optimización máxima y LTO fino para
priorizar el rendimiento del renderizador y aspirar a una experiencia cercana a
60 FPS, según la complejidad de la escena y la resolución elegida.

## Verificación

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo build --release
```
