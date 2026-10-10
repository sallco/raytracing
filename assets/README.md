# Texturas de ejecución

Los archivos de este directorio son copias PNG de `256×256` optimizadas a
partir de los paquetes `1K-JPG` proporcionados para el proyecto. El ejecutable
embebe únicamente albedo, normal OpenGL, roughness y metalness cuando existe.

Todas las fuentes proceden de [ambientCG](https://ambientcg.com/) y están
publicadas bajo [Creative Commons CC0 1.0 Universal](https://docs.ambientcg.com/license/).
La licencia permite copiar, modificar y redistribuir los archivos, incluso
dentro del proyecto y para uso comercial, sin exigir atribución.

| Recurso | Mapas versionados | Uso actual | Fuente |
| --- | --- | --- | --- |
| Ground104 | Albedo, NormalGL, Roughness | Venus, Tierra, Júpiter y Saturno | [ambientCG Ground104](https://ambientcg.com/a/Ground104) |
| Ground111 | Albedo, NormalGL, Roughness | Venus, Tierra, Marte y Júpiter | [ambientCG Ground111](https://ambientcg.com/a/Ground111) |
| Metal034 | Albedo, NormalGL, Roughness, Metalness | Sol | [ambientCG Metal034](https://ambientcg.com/a/Metal034) |
| Metal040 | Albedo, NormalGL, Roughness, Metalness | Tierra, Urano y Neptuno | [ambientCG Metal040](https://ambientcg.com/a/Metal040) |
| Rocks014 | Albedo, NormalGL, Roughness | Mercurio, Marte, anillos, Urano y Neptuno | [ambientCG Rocks014](https://ambientcg.com/a/Rocks014) |
| Rocks025 | Albedo, NormalGL, Roughness | Mercurio, Saturno y anillos | [ambientCG Rocks025](https://ambientcg.com/a/Rocks025) |

Cada cara visible usa coordenadas UV locales al cubo: no se proyecta una sola
imagen sobre el planeta completo. La posición del voxel determina de forma
estable qué recurso recibe, su orientación, escala, recorte y variación de
luminosidad. El patrón planetario se evalúa en el centro del voxel para obtener
un único tinte por cubo; el albedo de ambientCG es la superficie principal y no
una modulación secundaria. En los cuerpos rocosos y gigantes cálidos conserva
entre 85% y 92% de su color original; Tierra y los gigantes azules reciben más
tinte para mantener su identidad. Los mapas PBR aportan el detalle dentro de
cada cara.
Una separación tonal estrecha en los bordes evita que dos caras coplanares se
lean como una superficie continua.

Para reducir aliasing y presión de caché durante el raytracing, los PNG se
decodifican una sola vez en CPU y se filtran a `64×64` para el muestreo. Las
copias `256×256` se conservan como fuente de ejecución para futuras mejoras de
nivel de detalle.
