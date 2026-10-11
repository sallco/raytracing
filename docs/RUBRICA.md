# Correspondencia con la rúbrica

Este documento relaciona cada criterio de evaluación con evidencia visible,
implementación concreta y una forma de comprobarlo. La suma nominal de los
apartados publicados en la guía supera 100 puntos; se conserva cada criterio tal
como fue proporcionado y se asume el límite global de 100 indicado por la misma
guía.

## Restricciones obligatorias

| Restricción | Cumplimiento | Evidencia |
| --- | --- | --- |
| No usar una librería de raytracing | Cumple | Cámara, rayos, AABB, DDA, iluminación, reflexión, refracción y mapas normales están implementados en `src/`. |
| No usar la tarjeta de video para calcular la escena | Cumple | Todos los píxeles se calculan en `renderer::render_into`; no existen shaders ni geometría 3D enviada a la GPU. |
| No usar librerías externas para resolver los algoritmos | Cumple | La única dependencia directa es `raylib`, limitada a ventana, entrada, texto, PNG y presentación. El paralelismo usa `std::thread`. |
| Hacer visibles e intencionales los efectos | Cumple | Los efectos se asignan a objetos con función contextual y se recorren en el video de demostración. |

`raylib` puede usar el sistema gráfico para copiar el framebuffer CPU a la
ventana. Esta presentación 2D no calcula rayos, intersecciones, materiales ni
iluminación.

## Matriz de evaluación

| Criterio | Valor de la guía | Evidencia del proyecto | Verificación sugerida |
| --- | ---: | --- | --- |
| Complejidad de la escena | 20 | 12 elementos, 7089 cubos, ocho planetas, anillos, estación, satélite, nave, animación jerárquica y selección individual. | Recorrer los elementos con las flechas. |
| Atractivo visual | 15 | Composición cenital, identidad cromática por planeta, texturas PBR, luz solar, sombras, halo, órbitas, cielo estelar y HUD. | Observar la vista general y enfocar Tierra, Saturno y la nave. |
| Paralelismo y optimización | 10 | Franjas sin `Mutex`, hasta 12 trabajadores, AABB, chunks `8³`, DDA, any-hit, poda de rayos, LUT y render bajo demanda. | Ejecutar `cargo run --release -- --benchmark`. |
| Rotación y zoom | 10 | Traslación orbital y rotación local continua; cámara orbital, rueda de zoom y transiciones suaves. | Arrastrar, usar la rueda, seleccionar y pulsar `Esc`. |
| Hasta cinco materiales | 5 cada uno | Hay 16 materiales texturizados con parámetros independientes; cinco familias representativas se detallan abajo. | Enfocar los objetos asociados y revisar el HUD. |
| Refracción | 10 | Hielo de Saturno con IOR 1.31 y cristal de cabina con IOR 1.52 mediante ley de Snell. | Enfocar Saturno y la Nave Exploradora. |
| Reflexión | 5 | Océano, hielo, metales, paneles y cristal lanzan rayos reflejados con Fresnel. | Enfocar Tierra, Saturno, estación y nave. |
| Mapas normales | 10 | Los `NormalGL` alteran la normal en la base tangente de cada cara. | Orbitar cerca de un planeta y observar el relieve bajo la luz. |
| Material emisivo | 10 | Plasma solar y propulsores de iones tienen radiancia propia. El Sol coincide con la luz puntual principal. | Enfocar Sol y Nave Exploradora. |
| Skybox | 10 | Cubo de seis caras `±X`, `±Y`, `±Z` con estrellas y nebulosa generadas en CPU. | Orbitar la cámara y observar rayos sin impacto. |
| Terreno procedural mínimo `16 × 16` | 20 | Marte se genera sobre `20 × 20`, con ruido determinista, cuatro cráteres y extracción de superficie. | Enfocar Marte y ejecutar `cargo test`. |

## Materiales que sustentan el criterio de 25 puntos

La rúbrica puntúa un máximo de cinco materiales. Los siguientes cinco tienen
identidad visible, textura propia y parámetros diferenciados:

| Material | Texturas | Rugosidad | Especular | Metálico | Reflectividad | Transparencia | Uso intencional |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| Regolito mercuriano | Rocks011, Rocks014, Rocks024S, Rocks025 | 0.95 | 0.18 | 0.00 | 0.00 | 0.00 | Superficie seca y rocosa de Mercurio. |
| Continente terrestre | Grass005 | 0.78 | 0.18 | 0.00 | 0.00 | 0.00 | Masas verdes contrastadas con el océano. |
| Suelo marciano | Ground111, Metal041B, Metal041C, Metal053C, Metal056C, Rock029 | 0.92 | 0.18 | 0.00 | 0.00 | 0.00 | Relieve procedural rojizo y craterizado. |
| Casco de aleación | Metal040, Metal056C | 0.38 | 0.62 | 0.65 | 0.25 | 0.00 | Estructura reflectante de estación y satélite. |
| Panel fotovoltaico | Metal046B, Metal061B | 0.22 | 0.84 | 0.80 | 0.45 | 0.00 | Alas solares oscuras y pulidas. |

Los demás materiales refuerzan la variedad visual y los efectos específicos:

| Material | Rugosidad | Especular | Metálico | Reflexión | Transparencia | IOR | Emisión |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| Plasma solar | 0.90 | 0.18 | 0.00 | 0.00 | 0.00 | 1.00 | `(5.2, 2.1, 0.25)` |
| Nubes de Venus | 0.82 | 0.18 | 0.00 | 0.00 | 0.00 | 1.00 | — |
| Océano terrestre | 0.20 | 0.72 | 0.00 | 0.14 | 0.00 | 1.00 | — |
| Atmósfera de Júpiter | 0.72 | 0.18 | 0.00 | 0.00 | 0.00 | 1.00 | — |
| Atmósfera de Saturno | 0.76 | 0.18 | 0.00 | 0.00 | 0.00 | 1.00 | — |
| Hielo de los anillos | 0.44 | 0.58 | 0.10 | 0.15 | 0.18 | 1.31 | — |
| Hielo de Urano | 0.40 | 0.18 | 0.00 | 0.00 | 0.00 | 1.00 | — |
| Hielo de Neptuno | 0.36 | 0.18 | 0.00 | 0.00 | 0.00 | 1.00 | — |
| Blindaje compuesto | 0.42 | 0.48 | 0.50 | 0.22 | 0.00 | 1.00 | — |
| Propulsor de iones | 0.10 | 0.18 | 0.00 | 0.00 | 0.00 | 1.00 | `(2.5, 6.0, 9.0)` |
| Cristal de la cabina | 0.08 | 0.95 | 0.05 | 0.20 | 0.62 | 1.52 | — |

Cada uno de los 16 materiales tiene una configuración de textura y un conjunto
de parámetros propios. Que dos materiales compartan alguna familia PNG no los
convierte en el mismo material: difieren en selección de mapas, tinte, escala,
recorte y propiedades ópticas. Para el máximo evaluable de cinco se eligieron
deliberadamente materiales con asignaciones texturales claramente distintas.

## Evidencia visible por efecto

### Emisión e iluminación solar

El Sol no es únicamente una esfera amarilla: su material suma radiancia al color
y su centro es la posición de una luz puntual. El halo aparece en el fondo
alrededor de esta misma posición, reforzando visualmente la emisión.

### Reflexión

Los océanos de la Tierra introducen reflejos suaves; la estación y los paneles
usan reflejos metálicos más fuertes. La mezcla depende del ángulo mediante una
aproximación de Fresnel y se limita para conservar estabilidad.

### Refracción

Los fragmentos de hielo en los anillos de Saturno y el cristal frontal de la
nave son contextos naturales para transmitir y desviar rayos. La implementación
cambia la relación de índices al entrar o salir y evita un rayo transmitido
cuando ocurre reflexión interna total.

### Mapas normales

Cada cara del cubo construye tangente y bitangente coherentes. La normal RGB del
PNG se decodifica, se modera para evitar artefactos y se transforma a la base de
la cara antes de la iluminación. Por ello el relieve responde a la posición del
Sol sin modificar la geometría.

### Skybox

El fondo no es un gradiente plano. Al iniciar se generan seis imágenes cuadradas
que corresponden a las direcciones de un cubo. Un rayo sin impacto selecciona la
cara por su componente dominante y consulta estrellas y nebulosa en UV. Una
prueba cubre las seis direcciones cardinales.

## Lista de comprobación para la entrega

- [x] Repositorio Git con historial de commits atómicos.
- [x] Video de demostración enlazado en el README.
- [x] Ejecución documentada con perfil `release`.
- [x] Controles visibles en el HUD y explicados en el README.
- [x] Procedencia y licencia de texturas documentadas.
- [x] Conteo exacto de cuerpos y cubos disponible en HUD y benchmark.
- [x] Benchmark reproducible sin abrir una ventana.
- [x] Prueba automatizada del tamaño mínimo del terreno.
- [x] Formato, pruebas, Clippy y compilación de lanzamiento incluidos en la validación.

La explicación técnica de estas evidencias se encuentra en
[`ARQUITECTURA.md`](ARQUITECTURA.md), y el inventario legal y operativo de las
texturas en [`assets/README.md`](../assets/README.md).
