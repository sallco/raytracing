# Diorama solar voxelizado

Raytracer interactivo escrito en Rust que representa un sistema solar animado
mediante cubos texturizados. La CPU genera cada píxel de la escena: cámara,
intersecciones, iluminación, sombras, mapas normales, reflexión, refracción y
skybox. `raylib` se limita a crear la ventana, leer controles, decodificar PNG,
dibujar texto y presentar el framebuffer terminado.

El proyecto fue desarrollado para poner en práctica los fundamentos de
raytracing sin usar shaders, APIs de trazado por hardware ni librerías que
resuelvan los algoritmos de renderizado.

## Video de demostración

**Recorrido interactivo del diorama solar — 1 min 31 s**

https://github.com/user-attachments/assets/06b487fe-db9c-4577-bfcd-6c6450fb0bb2

El video muestra la vista general, las órbitas animadas, la cámara orbital, el
zoom libre, la selección de cuerpos, el texturizado individual de los cubos y
las métricas de rendimiento en tiempo real.

## Resultado

| Característica | Implementación |
| --- | --- |
| Escena | Sol, ocho planetas, anillos de Saturno, estación, satélite y nave |
| Geometría | 12 elementos seleccionables y 7089 cubos visibles |
| Terreno procedural | Marte craterizado sobre un dominio de `20 × 20` |
| Materiales | 16 configuraciones con texturas y parámetros ópticos propios |
| Recursos | 17 familias CC0, 59 PNG optimizados de `256 × 256` |
| Iluminación | Sol emisivo, luz puntual cálida, ambiente tenue y sombras |
| Efectos | Normal mapping, reflexión, refracción, Fresnel y halo solar |
| Entorno | Skybox procedural de seis caras con estrellas y nebulosa |
| Animación | Órbitas jerarquizadas y rotación axial a 30 Hz |
| Salida | Ventana `1280 × 720`, raytracing interno `768 × 432` |
| Paralelismo | Franjas independientes con `std::thread::scope`, hasta 12 hilos |

Las escalas y velocidades están comprimidas para que la escena completa sea
legible y el movimiento resulte visible. Se conserva una relación cualitativa:
los planetas interiores orbitan más rápido que los exteriores y los gigantes
gaseosos tienen una rotación relativamente rápida.

## Controles

| Acción | Control |
| --- | --- |
| Orbitar la cámara | Arrastrar con el botón izquierdo |
| Acercar o alejar | Rueda del ratón |
| Elemento anterior o siguiente | Flechas izquierda y derecha |
| Volver suavemente a la vista general | `Esc` |

Al seleccionar un elemento, la cámara hace una transición suave hacia una
distancia inicial apropiada. Esa distancia no bloquea el zoom: la rueda puede
seguir acercando o alejando la vista. El HUD muestra FPS, tiempo del último
raytrace, nombre, tipo, cantidad de cubos y material predominante.

## Elementos y cantidad de cubos

| Elemento | Tipo | Cubos | Material predominante |
| --- | --- | ---: | --- |
| Sol | Estrella | 1416 | Plasma solar |
| Mercurio | Planeta rocoso | 240 | Regolito mercuriano |
| Venus | Planeta rocoso | 336 | Nubes de Venus |
| Tierra | Planeta oceánico | 456 | Océano terrestre |
| Estación Orbital | Estación espacial | 118 | Casco de aleación |
| Ik'sat | Satélite artificial | 64 | Casco de aleación |
| Marte | Planeta procedural | 888 | Suelo marciano |
| Nave Exploradora | Nave espacial | 72 | Blindaje compuesto |
| Júpiter | Gigante gaseoso | 968 | Atmósfera de Júpiter |
| Saturno y sus anillos | Gigante con anillos | 1267 | Atmósfera de Saturno |
| Urano | Gigante helado | 632 | Hielo de Urano |
| Neptuno | Gigante helado | 632 | Hielo de Neptuno |
| **Total** |  | **7089** |  |

La estación e Ik'sat orbitan la Tierra en planos inclinados; la nave posee una
órbita heliocéntrica independiente. Saturno integra el planeta y sus anillos en
un solo cuerpo seleccionable.

## Materiales y texturas

Cada cubo visible tiene sus propias UV por cara. Un hash derivado de su posición
selecciona de manera determinista la familia, el recorte, la escala y la
variación tonal de la textura. Por tanto, los planetas no están recubiertos por
una única imagen continua: cubos vecinos pueden presentar detalles distintos
sin perder la identidad visual del cuerpo.

Los 16 materiales incluyen plasma solar, regolitos, nubes, océano, continente,
suelo marciano, atmósferas gaseosas, hielos, aleaciones, paneles fotovoltaicos,
blindaje, propulsores y cristal. Todos definen albedo, rugosidad, brillo
especular, metalicidad, reflectividad y transparencia; los materiales que lo
requieren añaden emisión o índice de refracción.

- El océano, los metales, los paneles y el cristal producen reflexión.
- El hielo de Saturno usa IOR 1.31 y la cabina de la nave IOR 1.52.
- El Sol y los propulsores de iones son emisivos.
- Los mapas `NormalGL` alteran la iluminación en el espacio tangente de la cara.
- Albedo, normal, roughness y metalness proceden estrictamente de los PNG
  versionados en el proyecto.

El inventario completo de archivos, asignaciones, optimización, procedencia y
licencia está en [assets/README.md](assets/README.md). Todas las fuentes son de
[ambientCG](https://ambientcg.com/) y se distribuyen bajo
[CC0 1.0 Universal](https://docs.ambientcg.com/license/).

## Generación procedural

Marte se construye al iniciar el programa sin cargar un modelo. Una esfera de
radio 9.5 celdas recibe relieve mediante hash espacial y cuatro depresiones
craterizadas. Después se inspeccionan los seis vecinos de cada celda y solo se
almacena la superficie expuesta. El resultado tiene 888 cubos y cubre un área de
`20 × 20`, por encima del mínimo de `16 × 16` solicitado.

La generación es determinista: una misma versión del programa produce siempre
la misma geometría. Una prueba automatizada verifica el tamaño mínimo del
terreno.

## Arquitectura del raytracer

```text
Cámara → rayo primario → AABB del cuerpo → DDA por celdas → impacto del cubo
       → textura PBR → iluminación/sombra → reflexión o refracción → RGBA8
       └──────────────────── sin impacto → skybox + órbitas + halo solar
```

Los cuerpos permanecen en coordenadas locales. Para animarlos se actualiza su
transformación y se lleva el rayo al espacio local, evitando mover miles de
cubos. Cada cuerpo tiene una AABB; dentro de ella, un DDA recorre únicamente las
celdas atravesadas y consulta ocupación en chunks de `8 × 8 × 8`.

El framebuffer se divide en franjas horizontales no solapadas. Hasta 12 hilos de
la biblioteca estándar escriben directamente en su región, sin `Mutex`. La
reflexión y la refracción se limitan a dos rebotes y se podan cuando su aporte
acumulado es imperceptible.

La explicación completa del pipeline, las velocidades, las decisiones de
memoria y todas las optimizaciones está en
[docs/ARQUITECTURA.md](docs/ARQUITECTURA.md).

## Rendimiento

En la medición de referencia del equipo de desarrollo, la vista general tardó
13.8 ms por raytrace (72.45 FPS equivalentes). Saturno fue la vista más costosa
con 27.0 ms (37.08 FPS equivalentes), por encima del objetivo de 30 FPS. Estos
valores dependen del procesador, el número de hilos y la carga del sistema.

Las principales optimizaciones son:

- almacenamiento exclusivo de voxeles superficiales;
- descarte temprano mediante AABB por cuerpo;
- chunks `8³` y recorrido DDA con acceso directo a celdas;
- cuerpo enfocado evaluado primero y retorno temprano en su impacto;
- sombras `any-hit` y omisión de rayos sin aporte luminoso;
- poda de rayos secundarios por profundidad y contribución;
- texturas, skybox y tabla gamma inicializados una sola vez;
- reutilización del framebuffer y renderizado solo cuando cambia la escena;
- resolución interna desacoplada del HUD y la ventana.

Para repetir la medición sin abrir una ventana:

```bash
cargo run --release -- --benchmark
```

El reporte imprime la vista, tipo, cubos, material, milisegundos y FPS
equivalentes de la composición general y de cada elemento.

## Requisitos y ejecución

Se requiere una instalación estable de Rust compatible con la edición 2024 y
las herramientas nativas que necesita `raylib` en el sistema operativo. La
versión de referencia fue compilada con Rust 1.99.0 y `raylib` 5.5.1.

```bash
git clone git@github.com:sallco/raytracing.git
cd raytracing
cargo run --release
```

El perfil `release` habilita optimización nivel 3, LTO fino, una sola unidad de
código y eliminación de símbolos. Es el perfil recomendado para evaluación,
capturas y video. El perfil de desarrollo conserva optimización nivel 1 para que
la interacción siga siendo utilizable:

```bash
cargo run
```

## Verificación

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo build --release
```

## Documentación

| Documento | Contenido |
| --- | --- |
| [Arquitectura y rendimiento](docs/ARQUITECTURA.md) | Pipeline, inventario, animación, paralelismo, optimizaciones y benchmark completo |
| [Correspondencia con la rúbrica](docs/RUBRICA.md) | Evidencia por criterio, parámetros de materiales y lista de entrega |
| [Texturas y licencias](assets/README.md) | Catálogo de los 59 PNG, asignación, carga, fuente y licencia CC0 |

La evidencia específica para cada apartado de evaluación está organizada en
[docs/RUBRICA.md](docs/RUBRICA.md).

## Referencias de la guía

- [Ejemplo de diorama](https://www.youtube.com/watch?v=91kxRGeg9wQ)
- [Dioramas físicos de referencia](https://www.youtube.com/watch?v=WN98_qIKVds)
