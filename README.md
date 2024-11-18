# Proyecto: Simulador del Sistema Solar

Este proyecto consiste en un simulador del sistema solar utilizando un software renderer en Rust. El sistema solar simulado incluye varios cuerpos celestes como el Sol, planetas rocosos, gigantes gaseosos, lunas, y anillos, con la opción de agregar más cuerpos celestes opcionales.

## Video de explicación y demostración
- He grabado un pequeño video demostrando y explicando brevemente el programa, puedes verlo en YouTube: [Haciendo clic en este video](https://)

## Instrucciones de Uso

### Requisitos Previos

1. **Rust**: Asegúrese de tener instalado Rust en su sistema. Puede descargarlo desde [rust-lang.org](https://www.rust-lang.org/).
2. **Cargo**: Cargo se incluye con la instalación de Rust. Asegúrese de que esté disponible en su sistema.

### Archivos Incluidos

- `src/`: Carpeta que contiene todos los archivos de ejecución.
  - `camera.rs`: Configuración de la cámara.
  - `main.rs`: Configuraciones iniciales e importaciones de los demás archivos
  - `model.rs`: Configuración para los modelos utilizados.
  - `nave.rs`: Configuraciones de la nave.
  - `planet.rs`: Configuraciones de los planetas.
  - `renderer.rs`: Configuraciones para renderizar.
  - `skybox.rs`: Configuraciones para la creación del skybox.
  - `utils.rs`: Configuraciones para proyectar un vértice 3D en el espacio de la cámara hacia el espacio 2D de la ventana.
- `assets/`: Carpeta con los recursos para los cuerpos celestes y los objetos.

### Ejecución

1. Clona el repositorio o descarga el proyecto.
2. Entra al directorio del proyecto.
3. Ejecuta el simulador utilizando el siguiente comando en la terminal:
    ```bash
    cargo run
    ```
    Esto compilará y ejecutará el simulador.

### Funcionalidades

1. **Simulación del Sistema Solar**:
   - El sistema solar incluye varios planetas alineados en un plano eclíptico.
   - Los planetas se mueven en órbitas elípticas y rotan sobre su propio eje.
   - Se puede observar el Sol y sus efectos visuales, incluidos los reflejos y las sombras en los planetas.
   - Las lunas pueden orbitar alrededor de los planetas rocosos.
   - Se pueden agregar más planetas o lunas personalizadas.

2. **Cámara y Movimiento**:
   - La cámara sigue la órbita de los planetas o se puede mover en 3D.
   - Soporte para navegación y zoom en el sistema solar.
   
3. **Efectos Visuales**:
   - Se incluyen efectos visuales como anillos en los planetas, nubes y atmósfera para los planetas rocosos.
   - El Sol emite luz y las sombras se calculan dinámicamente.

4. **Ampliación**:
   - Se puede añadir más planetas o cuerpos celestes opcionales.
   - El sistema de anillos puede personalizarse para diferentes planetas.

### Créditos

Este proyecto fue desarrollado como parte de un laboratorio sobre sistemas solares utilizando gráficos y simulaciones en Rust.

| ![Foto de Perfil](https://avatars.githubusercontent.com/u/115051589?v=4) | **Carlos Valladares**  
|:-------------------------------------------------:|------------------------  
| **Rol:** Desarrollador principal                 | 📚 Estudiante de Ciencias de la Computación en UVG  
| **Contacto:** [GitHub](https://github.com/vgcarlol) | ✨ Pasión por resolver problemas computacionales  
