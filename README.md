# 🦀 Programación I — FICCT · UAGRM
 
> **Lenguaje principal:** Rust · **Paradigma:** Programación Orientada a Objetos  
> **Docente:** Msc. Ing. Víctor Hugo Acosta Ortega  
> **Facultad:** FICCT — Universidad Autónoma Gabriel René Moreno · Santa Cruz de la Sierra, Bolivia
 
---
 
## 📋 Descripción
 
Repositorio oficial de la materia **Programación I** correspondiente al ciclo académico **2026**.  
Aquí se publican los proyectos, ejemplos y ejercicios desarrollados durante las clases.
 
El objetivo de la materia es enseñar **lógica de programación y POO** utilizando el lenguaje **Rust** como vehículo pedagógico, desarrollando en el estudiante pensamiento computacional sólido y buenas prácticas de diseño de software.
 
---
 
## 🏫 Grupos
 
| Grupo | Código | Horario |
|-------|--------|---------|
| Grupo A | **SC** | Lun,Mie, Vie / 16:45 - 18:15 |
| Grupo B | **SH** | Mar, Jue / 16:00 - 18:15 |
 
 
---
 
## 📚 Contenido temático
 
La materia sigue la siguiente secuencia de aprendizaje progresivo:
 
### 1. 🔢 Números
Tipos numéricos en Rust (`i32`, `u64`, `f64`, etc.), operaciones aritméticas, conversiones, y representación de datos numéricos mediante `struct` e `impl`.
 
### 2. 🔤 Cadenas
Manejo de `String` y `&str`, métodos de cadenas, entrada/salida por terminal, y modelado de entidades con atributos de texto.
 
### 3. 📦 Vectores
Colecciones dinámicas con `Vec<T>`, iteradores, closures y aplicación de POO para gestionar listas de objetos.
 
### 4. 🧮 Matrices
Representación de matrices con `Vec<Vec<T>>`, operaciones matriciales y estructuras de datos compuestas con POO.
 
---
 
## 🧩 Conceptos de POO en Rust
 
A lo largo del curso se trabajan los siguientes pilares de la Programación Orientada a Objetos aplicados al lenguaje Rust:
 
| Concepto POO | Equivalente en Rust |
|---|---|
| Clase / Objeto | `struct` + `impl` |
| Encapsulamiento | `pub` / visibilidad de módulos |
| Herencia / Polimorfismo | `trait` |
| Composición | Structs anidados |
| Constructores | Métodos asociados (`new`) |
| Métodos de instancia | `fn metodo(&self)` / `fn metodo(&mut self)` |
 
---
 
## 🛠️ Requisitos para ejecutar los proyectos
 
Asegúrate de tener instalado:
 
```bash
# Instalar Rust (via rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 
# Verificar instalación
rustc --version
cargo --version
```
 
### Ejecutar un proyecto
 
```bash
# Ir a la carpeta del proyecto
cd SC/01_numeros/nombre_proyecto
 
# Compilar y ejecutar
cargo run
 
# Solo compilar
cargo build
```
 
---
 
## 📅 Estructura de evaluación
 
| Evaluación | Descripción |
|---|---|
| 1er Parcial | Números y Cadenas |
| 2do Parcial | Vectores y Matrices |
| Proyecto | A definir |
| Examen Final | TODO |
 
---

Accede a los recursos preparados por el docente:
 
| Recurso | Descripción | Enlace |
|---|---|---|
| 🦀 **Introducción a Rust** | Historia, características, ownership, por qué aprender Rust | [hugonex.com/rust](https://hugonex.com/rust) |
| 🧩 **POO en Rust** | `struct`, `impl`, `pub`, `trait` y composición con ejemplos | [hugonex.com/poorust](https://hugonex.com/poorust) |
| 🧩 **Cadenas POO - Rust** | `struct`, `impl`, `pub`, `Cadenas` | [hugonex.com/cadenas](https://hugonex.com/cadenas) |
| 📺 **Canal de YouTube** | Videos, tutoriales y clases del docente | [youtube.com/@hnexcode](https://www.youtube.com/@hnexcode) |
 
---
 
## 📌 Convenciones del repositorio
 
- Cada carpeta de proyecto sigue la estructura estándar de **Cargo** (`src/main.rs`, `Cargo.toml`).
- Los proyectos son los desarrollados **en clase**, tal como se construyeron durante las sesiones.
- Los comentarios en el código están en **español** para facilitar la comprensión del estudiante.
 
---
 
## 👨‍🏫 Docente
 
**Msc. Ing. Víctor Hugo Acosta Ortega**  
Docente de Programación I — FICCT, UAGRM  
Santa Cruz de la Sierra, Bolivia
 
---
 
> *"Rust no es solo un lenguaje de programación — es una forma de pensar con precisión."*
 
---
 
![Rust](https://img.shields.io/badge/Lenguaje-Rust-orange?style=flat-square&logo=rust)
![UAGRM](https://img.shields.io/badge/Universidad-UAGRM-blue?style=flat-square)
![FICCT](https://img.shields.io/badge/Facultad-FICCT-green?style=flat-square)
![Estado](https://img.shields.io/badge/Semestre-2026-yellow?style=flat-square)
 
