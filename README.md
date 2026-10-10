# Raytracing

Diorama 3D renderizado mediante raytracing por CPU para el curso de Gráficas.
La primera versión presenta un sistema solar animado formado por 6211 cubos de
superficie. `raylib` se limita a ventana, entrada, texto y presentación del
framebuffer; la cámara, intersecciones, iluminación y efectos se calculan en CPU.

La escena se concentra exclusivamente en el Sol y los ocho planetas, incluidos
los anillos de Saturno. Incluye trayectorias orbitales visibles, traslación y
rotación planetaria, cielo estelar procedural, halo solar, luz directa, ambiente
tenue, sombras, reflexión y refracción con un máximo de dos rebotes secundarios.
Cada cubo tiene una textura PBR propia, elegida y orientada de forma
determinista entre los materiales asignados a su planeta.

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

El modo de medición genera un cuadro completo de `1024×576` sin abrir una ventana:

```bash
cargo run --release -- --benchmark
```

La simulación usa tiempo real y actualiza la animación a 30 pasos por segundo.
Entre actualizaciones se reutiliza el último framebuffer para conservar la
respuesta del HUD y los controles.

## Arquitectura

- Solo se almacenan voxeles expuestos de cada cuerpo.
- La escala visual está desacoplada de la resolución voxel de cada planeta.
- Los voxeles se agrupan en chunks locales de `8×8×8`.
- Cada rayo descarta cuerpos por AABB y recorre sus chunks mediante DDA.
- El framebuffer se divide en hasta 12 franjas usando `std::thread::scope`.
- Los cuerpos permanecen en coordenadas locales; cada rayo se transforma para
  rotarlos sin mover miles de voxeles.
- Las texturas CC0 se embeben, decodifican una vez en CPU y se muestrean con UV
  locales por cara; albedo, normal GL y roughness modifican la iluminación. No
  se generan albedos planetarios procedurales.

La procedencia, licencia y asignación de los recursos se documentan en
[`assets/README.md`](assets/README.md).

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
