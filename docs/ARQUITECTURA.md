# Arquitectura técnica y rendimiento

Este documento describe la implementación del diorama solar voxelizado. Su
propósito es dejar trazabilidad entre lo que aparece en pantalla y el código que
lo produce. La escena se calcula íntegramente mediante raytracing en CPU; `raylib`
se usa para crear la ventana, leer la entrada, decodificar los PNG, dibujar el HUD
y presentar el framebuffer terminado.

## Configuración de ejecución

| Parámetro | Valor |
| --- | ---: |
| Ventana | `1280 × 720` |
| Framebuffer calculado en CPU | `768 × 432` |
| Frecuencia objetivo de la ventana | 60 FPS |
| Frecuencia de actualización de la animación | 30 Hz |
| Límite de trabajadores | 12 hilos |
| Tamaño de chunk | `8 × 8 × 8` celdas |
| Rebotes secundarios máximos | 2 |
| Umbral de aporte de un rayo secundario | 0.08 |
| Skybox | 6 caras de `256 × 256` |
| Texturas conservadas en el repositorio | `256 × 256` PNG |
| Texturas muestreadas durante el trazado | `64 × 64` |

La imagen interna mantiene una relación de aspecto 16:9 y se escala al tamaño de
la ventana. El HUD se dibuja después a resolución nativa, por lo que permanece
legible sin aumentar el número de rayos primarios.

## Inventario de la escena

La escena contiene 12 elementos seleccionables y 7089 cubos. En los cuerpos
esféricos se guardan únicamente los cubos expuestos; la cantidad corresponde a
geometría que realmente puede participar en una intersección.

| Elemento | Tipo | Cubos | Material predominante | Órbita (rad/s) | Rotación (rad/s) |
| --- | --- | ---: | --- | ---: | ---: |
| Sol | Estrella | 1416 | Plasma solar | — | 0.060 |
| Mercurio | Planeta rocoso | 240 | Regolito mercuriano | 0.105 | 0.045 |
| Venus | Planeta rocoso | 336 | Nubes de Venus | 0.080 | -0.025 |
| Tierra | Planeta oceánico | 456 | Océano terrestre | 0.065 | 0.160 |
| Estación Orbital | Estación espacial | 118 | Casco de aleación | 0.420 | 0.150 |
| Ik'sat | Satélite artificial | 64 | Casco de aleación | -0.350 | -0.200 |
| Marte | Planeta procedural | 888 | Suelo marciano | 0.052 | 0.150 |
| Nave Exploradora | Nave espacial | 72 | Blindaje compuesto | 0.058 | 0.180 |
| Júpiter | Gigante gaseoso | 968 | Atmósfera de Júpiter | 0.032 | 0.280 |
| Saturno y sus anillos | Gigante con anillos | 1267 | Atmósfera de Saturno | 0.024 | 0.250 |
| Urano | Gigante helado | 632 | Hielo de Urano | 0.017 | -0.200 |
| Neptuno | Gigante helado | 632 | Hielo de Neptuno | 0.013 | 0.210 |
| **Total** |  | **7089** |  |  |  |

Las velocidades son deliberadamente visibles y no equivalen a tiempo
astronómico. Conservan una jerarquía reconocible: los planetas interiores
completan la órbita más rápido, los exteriores avanzan más despacio y los
gigantes gaseosos presentan una rotación relativamente rápida. Los signos
negativos representan movimiento retrógrado en la convención de la escena.

La estación e Ik'sat describen órbitas inclinadas alrededor de la Tierra. La
nave sigue una trayectoria heliocéntrica independiente. Las ocho órbitas
planetarias se dibujan como anillos luminosos sobre el plano orbital.

## Generación procedural de Marte

Marte se genera de forma determinista al iniciar la escena sobre un dominio de
`20 × 20` columnas, superior al mínimo de `16 × 16` exigido. Para cada celda 3D:

1. se calcula una esfera base de radio 9.5 celdas;
2. un hash espacial añade variación granular a la superficie;
3. cuatro direcciones normalizadas producen depresiones craterizadas;
4. se comprueban los seis vecinos ortogonales;
5. solo se conserva el voxel que limita con espacio vacío.

El resultado es un planeta completo de 888 cubos superficiales. La prueba
`marte_procedural_supera_diecisies_por_diecisies` verifica automáticamente que
la proyección del terreno cubra al menos 16 celdas en ambos ejes horizontales.

## Recorrido de un rayo

El pipeline de cada píxel sigue estas etapas:

1. La cámara construye un rayo primario con FOV vertical de 42° y corrección de
   aspecto.
2. El rayo se transforma al espacio local de cada cuerpo. Así, la rotación no
   obliga a mover ni reconstruir sus voxeles.
3. Una intersección rayo-AABB descarta cuerpos completos antes de consultar su
   geometría.
4. Un DDA avanza celda por celda desde la entrada hasta la salida de la AABB.
5. La celda consulta directamente el chunk y el índice del voxel; no se prueba
   el rayo contra todos los cubos.
6. El impacto calcula normal geométrica, cara, base tangente/bitangente y UV
   locales.
7. El material muestrea albedo, normal OpenGL, rugosidad y metalicidad.
8. Se evalúan ambiente tenue, luz solar puntual, atenuación cuadrática, sombra,
   difusión y brillo especular.
9. Cuando el material lo requiere, se trazan reflexión y refracción hasta dos
   rebotes secundarios.
10. Si no existe impacto se consulta el skybox cúbico y se añaden las órbitas y
    el halo visible del Sol.
11. El color lineal recibe mapeo tonal y corrección gamma mediante una tabla
    precalculada antes de escribirse como RGBA8.

La refracción usa la ley de Snell y contempla reflexión interna total. El hielo
de los anillos emplea IOR 1.31 y el cristal de cabina IOR 1.52.

## Texturizado por cubo

No existe una envoltura única aplicada a un planeta. Cada impacto identifica la
cara concreta del cubo y genera UV dentro de ella. La posición del voxel alimenta
un hash estable que selecciona:

- la familia de textura asignada al material;
- el recorte y desplazamiento dentro de la imagen;
- una de cuatro escalas de muestreo;
- la variación de luminosidad;
- la separación tonal de los bordes.

Esto hace que dos cubos vecinos puedan usar recortes e incluso familias de
textura diferentes sin perder la identidad cromática del cuerpo. Los recursos,
sus canales PBR, usos y licencias están inventariados en
[`assets/README.md`](../assets/README.md).

## Iluminación y efectos

- El Sol es un objeto visible con emisión `(5.2, 2.1, 0.25)` y coincide con una
  luz puntual cálida de intensidad 1600.
- Una contribución ambiental de 0.11 conserva legibilidad en caras no iluminadas.
- Las sombras usan un recorrido de oclusión que termina en el primer bloqueador.
- La reflexión mezcla el rayo reflejado según reflectividad y efecto Fresnel en
  ángulos rasantes.
- La refracción se usa de forma contextual en hielo y cristal.
- Los mapas `NormalGL` se transforman desde espacio tangente a espacio global.
- El skybox procedural se formaliza como seis caras cúbicas (`±X`, `±Y`, `±Z`),
  con estrellas y nebulosa generadas una sola vez.
- El halo solar se calcula para rayos de fondo cercanos a la fuente luminosa.

## Paralelismo

El framebuffer se divide en franjas horizontales independientes. Se usa
`std::thread::scope` para prestar a cada trabajador una sección mutable y no
solapada del vector de píxeles. No hay `Mutex`, datos compartidos mutables ni una
dependencia externa de concurrencia. El número de trabajadores se obtiene de
`available_parallelism()` y se limita a 12 o a la altura del framebuffer.

## Optimizaciones aplicadas

| Optimización | Efecto |
| --- | --- |
| Solo voxeles superficiales | Evita memoria e intersecciones con cubos interiores invisibles. |
| Escala visual separada del detalle | Aumenta el número de cubos sin alterar la composición orbital. |
| AABB por cuerpo | Descarta una agrupación completa con una prueba barata. |
| Chunks locales `8³` | Agrupa ocupación espacial y permite acceso directo a una celda. |
| DDA directo | Visita únicamente las celdas atravesadas por el rayo. |
| Coordenadas locales | Anima rotación y traslación sin regenerar geometría. |
| Cuerpo seleccionado primero | Acelera el caso dominante de las vistas enfocadas. |
| Retorno temprano del cuerpo enfocado | Finaliza al confirmar su impacto frontal. |
| Oclusión de tipo `any-hit` | Una sombra termina al encontrar el primer bloqueador opaco. |
| Poda de sombras | Omite rayos de sombra en emisores y caras sin aporte directo. |
| Poda por contribución | No lanza un rayo secundario cuyo aporte acumulado sea menor que 0.08. |
| Profundidad limitada | Reflexión y refracción se detienen después de dos rebotes. |
| Texturas residentes | `OnceLock` decodifica y reduce cada mapa una sola vez. |
| Skybox residente | Las seis caras se generan una vez y luego se consultan por eje dominante. |
| LUT de gamma | Sustituye una potencia por píxel por una consulta de 4096 niveles. |
| Reutilización de memoria | El vector RGBA conserva su capacidad entre fotogramas. |
| Render bajo demanda | El framebuffer solo se recalcula al avanzar la animación o cambiar la cámara. |
| Resolución interna | Reduce rayos primarios y mantiene HUD y ventana a resolución nativa. |
| Perfil de lanzamiento | `opt-level=3`, LTO fino, una unidad de código y símbolos eliminados. |

## Medición reproducible

El benchmark traza la vista general y las 12 vistas enfocadas sin abrir una
ventana:

```bash
cargo run --release -- --benchmark
```

Medición de referencia realizada el 10 de octubre de 2026 en el equipo de
desarrollo, a `768 × 432` y con el perfil `release`:

| Vista | Tiempo | FPS equivalentes |
| --- | ---: | ---: |
| General | 13.8 ms | 72.45 |
| Sol | 14.6 ms | 68.32 |
| Mercurio | 14.7 ms | 68.14 |
| Venus | 13.4 ms | 74.79 |
| Tierra | 16.0 ms | 62.31 |
| Estación Orbital | 23.6 ms | 42.35 |
| Ik'sat | 14.5 ms | 68.88 |
| Marte | 13.6 ms | 73.63 |
| Nave Exploradora | 12.7 ms | 79.02 |
| Júpiter | 12.8 ms | 77.99 |
| Saturno | 27.0 ms | 37.08 |
| Urano | 13.8 ms | 72.63 |
| Neptuno | 12.5 ms | 79.92 |

Los FPS equivalentes son el inverso del tiempo de un raytrace aislado; no miden
la presentación de la ventana. Los resultados dependen del procesador, el número
de hilos disponible, la carga del sistema y la vista. Incluso el caso más costoso
de esta muestra supera el objetivo de 30 FPS.

## Distribución del código

| Archivo | Responsabilidad |
| --- | --- |
| `src/main.rs` | Ventana, entrada, ciclo temporal, framebuffer y benchmark. |
| `src/camera.rs` | Rayos primarios, órbita, zoom, enfoque y transiciones. |
| `src/generation.rs` | Planetas, Marte procedural, estación, satélite y nave. |
| `src/world.rs` | Voxeles, chunks, cuerpos, órbitas y actualización local. |
| `src/geometry.rs` | AABB e información geométrica de impacto. |
| `src/scene.rs` | Materiales, luz, DDA, sombras y mapeo de caras. |
| `src/renderer.rs` | Paralelismo, trazado, iluminación, reflexión y refracción. |
| `src/texture.rs` | Carga embebida, reducción, selección y muestreo PBR. |
| `src/material.rs` | Parámetros de material, color lineal, tono y LUT gamma. |
| `src/sky.rs` | Generación y proyección del skybox cúbico. |
| `src/hud.rs` | Métricas, ficha del elemento y ayuda de controles. |
| `src/math.rs` | Vectores y operaciones matemáticas implementadas localmente. |

## Alcance del uso de GPU

No se usan shaders, cómputo, rasterización 3D ni APIs de raytracing en GPU. La
CPU produce cada valor RGBA del framebuffer. `raylib` entrega esa imagen a la
ventana como textura 2D, una operación de presentación que puede ser acelerada
por el sistema gráfico, pero que no participa en el cálculo de la escena.
