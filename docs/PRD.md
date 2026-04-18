---
titulo: Nexus - Motor Visualizador de Algoritmos
version: 1
autor: Jaime Mendivil Martinez
fecha: 2026-04-17
estado: Obsoleto (Concepto Base)
---

# PRD: Nexus - Motor Visualizador de Algoritmos (v1.0)

## 1. Visión y Propósito (El "Por qué")
* **El Problema:** Aprender estructuras de datos es abstracto. Los visualizadores web actuales son lentos con muchos datos, y el software de escritorio tradicional trae algoritmos "quemados" (hardcoded) en el código fuente, impidiendo que el estudiante agregue los suyos.
* **La Solución:** Un motor de escritorio nativo de altísimo rendimiento donde la lógica matemática se separa totalmente de la interfaz. Los algoritmos se escriben en scripts de texto plano, permitiendo una extensibilidad infinita.

## 2. Público Objetivo (User Personas)
* **El Estudiante Principal:** Estudiante de ingeniería de software cursando Estructuras de Datos. Sabe programar lo básico, pero se frustra al no poder "ver" cómo muta la memoria durante un ciclo *while*.
* **El Docente de Algoritmia:** Profesor que necesita una herramienta rápida y confiable que no dependa del internet de la universidad para dar su clase.

## 3. Casos de Uso (User Journeys)
* "Como estudiante, quiero cargar un script `.rhai` desde mi computadora y darle 'Play' para ver cómo se mueven los bloques de un *Bubble Sort* paso a paso, de modo que pueda prepararme para mi examen."
* "Como docente, quiero tener un control deslizante de velocidad para ejecutar el algoritmo rápido hasta llegar al punto crítico que quiero explicarle a la clase."

## 4. Alcance del MVP (In Scope vs. Out of Scope)
* **In Scope (En Alcance):**
    * Motor asíncrono en Rust (con canales `mpsc`).
    * Interfaz nativa en Dioxus.
    * Intérprete para cargar algoritmos locales (`.rhai`).
    * Renderizado de Arreglos 1D (bloques).
    * Controles: Play, Pause, Siguiente Paso, Velocidad.
* **Out of Scope (Fuera de Alcance):**
    * Renderizado de Grafos (Nodos/Aristas).
    * Actualizaciones automáticas.
    * Editor de código (IDE) dentro de la app.

## 5. Requisitos a Alto Nivel
* **Funcionales:** Botones de control de ejecución, selector de archivos locales para cargar algoritmos.
* **No Funcionales / Experiencia:** Rendimiento nativo en macOS y Windows; funcionamiento 100% *offline*; protección anti-cuelgues si el script del usuario tiene un bucle infinito.

## 6. Métricas de Éxito (KPIs)
* **Cuantitativas:** 60 FPS estables renderizando un arreglo de 500 elementos. Tiempo de arranque del software < 1.5 segundos.
* **Cualitativas:** Un usuario nuevo puede cargar su propio algoritmo en menos de 3 clics desde que abre la aplicación.

## 7. Dependencias y Riesgos
* **Técnicos:** Dioxus es un ecosistema joven; una actualización mayor del framework podría romper componentes de la interfaz.
* **De Recursos:** Tiempo de desarrollo limitado por las entregas y laboratorios de la universidad.

## 8. Cronograma / Hitos (Milestones)
* **Hito 1:** Diseño de UI y arquitectura interna aprobados.
* **Hito 2:** Prueba de concepto: El motor Rust compila un script `.rhai` y manda señales por el canal.
* **Hito 3:** MVP de escritorio listo para la primera prueba con un arreglo ordenándose visualmente.
