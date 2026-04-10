# Nexus 🌊

**Nexus** es un entorno interactivo de alto rendimiento para el análisis y visualización de algoritmos y estructuras de
datos.

A diferencia de los visualizadores estáticos tradicionales, Nexus está diseñado como un motor gráfico interactivo.
Separa estrictamente la representación lógica matemática de la capa visual, permitiendo visualizar conceptos complejos
como recursión, *Sliding Window*, *Memoization* y complejidad espacial de forma fluida e intuitiva.

## ✨ Características Principales

* **Arquitectura Híbrida (Native + Scripting):** Los algoritmos pueden ser ejecutados de forma nativa en Rust o cargados
  en tiempo de ejecución (Hot-Reloading) mediante el motor de scripting interno **Rhai**, sin necesidad de recompilar el
  proyecto.
* **Diseño Orientado a Datos (Data-Driven):** Explicaciones matemáticas, teoría de notación Big $O(n)$, y esquemas de
  colores se cargan dinámicamente desde archivos `TOML`.
* **Cámara 2D Espacial con Interpolación (Lerp):** Navegación fluida por estructuras de grafos masivos con zoom, paneo
  libre y seguimiento automático de los nodos activos durante la ejecución del algoritmo.
* **HUD de Complejidad Espacial:** Visualización en tiempo real de estructuras secundarias (Call Stacks, Min-Heaps,
  Tablas Hash) para entender el costo de memoria real de paradigmas como Programación Dinámica y *Backtracking*.

## 🛠️ Stack Tecnológico

* **[Rust](https://www.rust-lang.org/):** Lenguaje principal. Gestión de memoria segura y concurrencia.
* **[Raylib-rs](https://github.com/deltaphc/raylib-rs):** Renderizado gráfico acelerado por hardware para simulaciones
  de alta fluidez.
* **[Rhai](https://rhai.rs/):** Motor de scripting embebido para crear algoritmos sobre la marcha.
* **[Serde](https://serde.rs/) & TOML:** Serialización para el sistema de *temas* y metadatos explicativos.

## 🏗️ Arquitectura del Motor

El proyecto sigue un patrón estricto de separación de responsabilidades:

1. **DataContainer:** La estructura matemática universal (`Array`, `Tree`, `Graph`) que ignora por completo la pantalla.
   Utiliza diseño basado en índices para evitar referencias cruzadas en memoria.
2. **Algorithm State Machine:** Los algoritmos no usan bucles bloqueantes. Son máquinas de estado que mutan el
   `DataContainer` paso a paso y devuelven un `StepResult`.
3. **Renderer & Camera:** Transforma las coordenadas del mundo normalizado a la pantalla utilizando matrices de
   proyección matemática, calculando el *clamping* y escalamiento dinámico.

## 🚀 Cómo Empezar

### Prerrequisitos

Asegúrate de tener instalado [Rust y Cargo](https://rustup.rs/) y las dependencias del sistema necesarias para compilar
**Raylib** (CMake, compilador de C).

### Instalación

```bash
git clone [https://github.com/](https://github.com/)scarfish41987/Nexus.git
cd Nexus
cargo run --release

