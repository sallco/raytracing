# Catálogo de texturas y licencias

Los archivos de este directorio son copias PNG de `256×256` optimizadas a
partir de los paquetes `1K-JPG` proporcionados para el proyecto. El ejecutable
embebe únicamente albedo, normal OpenGL, roughness y metalness cuando existe.

## Resumen del inventario

| Dato | Valor |
| --- | ---: |
| Familias de textura | 17 |
| Archivos PNG | 59 |
| Resolución de cada archivo | `256 × 256` |
| Tamaño total aproximado | 4.4 MiB |
| Resolución de muestreo en ejecución | `64 × 64` |
| Licencia de las fuentes | CC0 1.0 Universal |

No se conservan los paquetes originales de 1K ni archivos que el ejecutable no
utilice. Todos los materiales visibles cargan estrictamente recursos incluidos
en `assets/textures/`; no existe un albedo de respaldo generado en tiempo de
ejecución.

## Procedencia y licencia

Todas las fuentes proceden de [ambientCG](https://ambientcg.com/) y están
publicadas bajo [Creative Commons CC0 1.0 Universal](https://docs.ambientcg.com/license/).
La licencia permite copiar, modificar y redistribuir los archivos, incluso
dentro del proyecto y para uso comercial, sin exigir atribución.

| Recurso | Mapas versionados | Uso actual | Fuente |
| --- | --- | --- | --- |
| Grass005 | Albedo, NormalGL, Roughness | Continentes de la Tierra | [ambientCG Grass005](https://ambientcg.com/a/Grass005) |
| Ground104 | Albedo, NormalGL, Roughness | Venus, Júpiter y Saturno | [ambientCG Ground104](https://ambientcg.com/a/Ground104) |
| Ground111 | Albedo, NormalGL, Roughness | Marte y Júpiter | [ambientCG Ground111](https://ambientcg.com/a/Ground111) |
| Metal034 | Albedo, NormalGL, Roughness, Metalness | Sol | [ambientCG Metal034](https://ambientcg.com/a/Metal034) |
| Metal040 | Albedo, NormalGL, Roughness, Metalness | Urano y Neptuno | [ambientCG Metal040](https://ambientcg.com/a/Metal040) |
| Metal041B | Albedo, NormalGL, Roughness, Metalness | Marte | [ambientCG Metal041B](https://ambientcg.com/a/Metal041B) |
| Metal041C | Albedo, NormalGL, Roughness, Metalness | Venus y Marte | [ambientCG Metal041C](https://ambientcg.com/a/Metal041C) |
| Metal046B | Albedo, NormalGL, Roughness, Metalness | Urano, Neptuno, paneles y propulsores de iones | [ambientCG Metal046B](https://ambientcg.com/a/Metal046B) |
| Metal053C | Albedo, NormalGL, Roughness, Metalness | Marte | [ambientCG Metal053C](https://ambientcg.com/a/Metal053C) |
| Metal056C | Albedo, NormalGL, Roughness, Metalness | Venus y Marte | [ambientCG Metal056C](https://ambientcg.com/a/Metal056C) |
| Metal061B | Albedo, NormalGL, Roughness, Metalness | Océanos, Urano, Neptuno, paneles y cristal de cabina | [ambientCG Metal061B](https://ambientcg.com/a/Metal061B) |
| Rock029 | Albedo, NormalGL, Roughness | Marte procedural | [ambientCG Rock029](https://ambientcg.com/a/Rock029) |
| Rocks011 | Albedo, NormalGL, Roughness | Mercurio y anillos | [ambientCG Rocks011](https://ambientcg.com/a/Rocks011) |
| Rocks012 | Albedo, NormalGL, Roughness | Júpiter y anillos | [ambientCG Rocks012](https://ambientcg.com/a/Rocks012) |
| Rocks014 | Albedo, NormalGL, Roughness | Mercurio, Saturno, anillos, Urano y Neptuno | [ambientCG Rocks014](https://ambientcg.com/a/Rocks014) |
| Rocks024S | Albedo, NormalGL, Roughness | Mercurio y anillos | [ambientCG Rocks024S](https://ambientcg.com/a/Rocks024S) |
| Rocks025 | Albedo, NormalGL, Roughness | Mercurio, Júpiter, Saturno y anillos | [ambientCG Rocks025](https://ambientcg.com/a/Rocks025) |

Los nombres de archivo conservan el identificador de ambientCG para que cada
recurso pueda rastrearse directamente hasta su página de origen. Los mapas
`Metalness` solo se incluyen en las familias metálicas que los proporcionan.

## Aplicación por cubo

Cada cara visible usa coordenadas UV locales al cubo: no se proyecta una sola
imagen sobre el planeta completo. La posición del voxel determina de forma
estable qué recurso recibe, su orientación, escala, recorte y variación de
luminosidad. No se genera ningún albedo procedural: la superficie procede
exclusivamente de estos PNG. Un color fijo del material puede teñir el albedo
para conservar la identidad de cada planeta, sin añadir otra imagen o patrón.
Los mapas PBR aportan el detalle dentro de cada cara.
Una separación tonal estrecha en los bordes evita que dos caras coplanares se
lean como una superficie continua.

Los canales se interpretan de la siguiente forma:

- `Color`: albedo lineal modulado por el tinte del material.
- `NormalGL`: normal tangente con componente Y en convención OpenGL.
- `Roughness`: mezcla del valor de la imagen y la rugosidad base del material.
- `Metalness`: multiplicador de la metalicidad base; en materiales no metálicos
  se usa cero.

## Carga y optimización

Para reducir aliasing y presión de caché durante el raytracing, los PNG se
decodifican una sola vez en CPU y se filtran a `64×64` para el muestreo. Las
copias `256×256` se conservan como fuente de ejecución para futuras mejoras de
nivel de detalle.

La carga se realiza mediante `include_bytes!`, por lo que las texturas quedan
embebidas en el binario. Cada familia usa `OnceLock`: se decodifica únicamente la
primera vez y luego todas las muestras comparten el mismo mapa residente.
