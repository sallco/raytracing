# Estado del proyecto

## Primera versión funcional

La primera versión funcional queda cerrada con un sistema solar animado de
6211 voxeles de superficie. La escena contiene únicamente el Sol, los ocho
planetas y los anillos de Saturno.

Está implementado:

- Raytracing por CPU a 1024×576 con framebuffer presentado mediante raylib.
- AABB por cuerpo, chunks locales de 8×8×8 y recorrido DDA.
- Render paralelo por franjas con la biblioteca estándar.
- Luz directa del Sol, ambiente tenue, sombras y halo solar.
- Reflexión y refracción limitadas a dos rebotes secundarios.
- Skybox procedural con estrellas y nebulosa.
- Cámara orbital con arrastre, zoom y transición suave a la vista general con `Esc`.
- Selección de cuerpos con flechas y transición suave a una vista de detalle;
  la distancia inicial es una sugerencia y la rueda conserva el zoom elegido.
- HUD con FPS, tiempo de render, nombre, tipo, voxeles y material.
- Escala visual independiente de la resolución voxel de cada planeta.
- Ocho trayectorias orbitales procedurales visibles.
- Movimiento orbital lento ligado al tiempo real y rotación axial local, con
  proporciones visuales inspiradas en la jerarquía real de periodos planetarios.
- Animación actualizada a 30 pasos por segundo con reutilización del framebuffer.
- Modo reproducible de medición con `--benchmark`.
- Texturas CC0 de ambientCG optimizadas y documentadas, con albedo, normal GL,
  roughness y metalness cuando está disponible.
- Mapeo independiente por cubo y por cara, con recorte, orientación, tinte y
  variación deterministas entre varios materiales dentro de cada planeta; no
  existe una envoltura de textura continua sobre el cuerpo completo.

## Próximas iteraciones

Queda pendiente, en orden aproximado:

1. Mejorar los patrones visuales de escala planetaria.
2. Reintroducir un cinturón de asteroides con composición y densidad controladas.
3. Incorporar estaciones espaciales, satélites y naves como elementos secundarios.
4. Mejorar reflexión, refracción y materiales contextuales.
5. Continuar optimizando el render durante movimiento de cámara y animaciones.

Los elementos secundarios deben añadirse después de consolidar la lectura visual
del sistema solar y nunca volver a dominar u ocultar los planetas.
