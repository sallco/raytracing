# Raytracing

Diorama 3D renderizado mediante raytracing por CPU para el curso de Gráficas.
La escena presenta un sistema solar animado formado por más de 7000 cubos.
`raylib` se limita a ventana, entrada, texto, decodificación de PNG y presentación
del framebuffer; la cámara, intersecciones, iluminación y efectos se calculan en
CPU, sin shaders ni aceleración de la tarjeta de video.

Incluye el Sol, los ocho planetas, anillos de Saturno, una estación, un satélite,
una nave y Marte con terreno craterizado procedural de `20×20`. También incorpora
trayectorias orbitales visibles, traslación y rotación, skybox estelar de seis
caras, halo solar, luz directa, ambiente tenue, sombras, reflexión y refracción
con un máximo de dos rebotes secundarios. Cada cubo visible tiene UV locales y
una textura PBR elegida de forma determinista entre los materiales asignados.

## Video

El video de entrega debe insertarse aquí después de grabar la ejecución en modo
`--release` y subir el archivo a GitHub. Este paso requiere una URL pública del
video y no puede completarse únicamente desde el código fuente.

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

El modo de medición genera el framebuffer CPU de `768×432` sin abrir una ventana:

```bash
cargo run --release -- --benchmark
```

La simulación usa tiempo real y actualiza la animación a 30 pasos por segundo.
Entre actualizaciones se reutiliza el último framebuffer para conservar la
respuesta del HUD y los controles. La imagen CPU se escala a la ventana de
`1024×576`; el HUD se dibuja después a resolución nativa.

## Arquitectura

- Solo se almacenan voxeles expuestos de cada cuerpo.
- La escala visual está desacoplada de la resolución voxel de cada planeta.
- Los voxeles se agrupan en chunks locales de `8×8×8`.
- Cada rayo descarta cuerpos por AABB y recorre sus chunks mediante DDA.
- El framebuffer se divide en hasta 12 franjas usando `std::thread::scope`.
- Los cuerpos permanecen en coordenadas locales; cada rayo se transforma para
  rotarlos sin mover miles de voxeles.
- El entorno es un skybox cúbico con seis caras generadas una sola vez en CPU y
  consultadas mediante proyección por eje dominante.
- Las texturas CC0 se embeben, decodifican una vez en CPU y se muestrean con UV
  locales por cara; albedo, normal GL y roughness modifican la iluminación. No
  se generan albedos planetarios procedurales.

## Evidencia de la rúbrica

| Criterio | Implementación visible |
| --- | --- |
| Complejidad | Sistema solar, anillos, estación, satélite y nave seleccionables. |
| Paralelismo y optimización | Franjas con `std::thread::scope`, AABB por cuerpo, chunks `8×8×8`, DDA y solo superficies en los cuerpos esféricos. |
| Rotación y zoom | Rotación propia y orbital; arrastre para orbitar y rueda para acercar o alejar. |
| Cinco materiales | Continente, regolito, metal, panel solar, hielo y cristal tienen textura y parámetros propios de albedo, especular, transparencia y reflectividad. |
| Refracción | Cristal de la cabina de la nave (`IOR 1.52`) e hielo de Saturno (`IOR 1.31`). |
| Reflexión | Océanos, paneles fotovoltaicos, metal y cristal reflejan el entorno. |
| Mapas normales | Los mapas `NormalGL` alteran la normal de iluminación de cada cara texturizada. |
| Emisión | El Sol y los propulsores de iones emiten luz visible. |
| Skybox | Seis caras (`+X`, `-X`, `+Y`, `-Y`, `+Z`, `-Z`) con estrellas y nebulosa. |
| Terreno procedural | `Marte`, seleccionable desde el HUD, genera una superficie esférica de `20×20` con ruido, relieve y cuatro cráteres. |

La procedencia, licencia y asignación de los recursos se documentan en
[`assets/README.md`](assets/README.md).

El perfil de desarrollo usa optimización moderada para conservar una iteración
fluida. El perfil de lanzamiento habilita optimización máxima y LTO fino para
priorizar el rendimiento del renderizador. La cifra real depende del procesador
y se debe registrar con el modo de medición antes de la entrega.

## Verificación

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo build --release
```
