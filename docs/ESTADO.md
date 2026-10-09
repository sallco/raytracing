# Estado del proyecto

## Primera versión funcional

La primera versión funcional queda cerrada con un sistema solar estático de
6211 voxeles de superficie. La escena contiene únicamente el Sol, los ocho
planetas y los anillos de Saturno.

Está implementado:

- Raytracing por CPU a 960×540 con framebuffer presentado mediante raylib.
- AABB por cuerpo, chunks locales de 8×8×8 y recorrido DDA.
- Render paralelo por franjas con la biblioteca estándar.
- Luz directa del Sol, ambiente tenue, sombras y halo solar.
- Reflexión y refracción limitadas a dos rebotes secundarios.
- Skybox procedural con estrellas y nebulosa.
- Cámara orbital con arrastre, zoom y transición suave a la vista general con `Esc`.
- Selección de cuerpos con flechas y transición suave a una vista de detalle.
- HUD con FPS, tiempo de render, nombre, tipo, voxeles y material.
- Escala visual independiente de la resolución voxel de cada planeta.
- Caché del último framebuffer cuando la escena permanece inmóvil.
- Modo reproducible de medición con `--benchmark`.

## Próximas iteraciones

Queda pendiente, en orden aproximado:

1. Añadir órbitas visibles y animar el movimiento orbital.
2. Incorporar texturas CC0 optimizadas y documentar sus licencias.
3. Aplicar mapas normales y mejorar los patrones visuales de cada planeta.
4. Añadir rotación local de los cuerpos.
5. Reintroducir un cinturón de asteroides con composición y densidad controladas.
6. Incorporar estaciones espaciales, satélites y naves como elementos secundarios.
7. Mejorar reflexión, refracción y materiales contextuales.
8. Continuar optimizando el render durante movimiento de cámara y animaciones.

Los elementos secundarios deben añadirse después de consolidar la lectura visual
del sistema solar y nunca volver a dominar u ocultar los planetas.
