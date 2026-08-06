# Documento maestro de lectura
## Ecosistema local de investigación, catálogo y resolución de skills

**Estado:** propuesta conceptual para revisión humana  
**Versión:** 0.1 — documento previo al plan por fases  
**Objetivo de esta versión:** acordar la arquitectura, los límites de confianza, la experiencia de uso y la estrategia de seguridad antes de crear el paquete de implementación por WAVEs.

---

## 1. Resumen ejecutivo

El proyecto propone construir un único repositorio clonable que reúna tres sistemas relacionados, pero técnicamente separados:

1. **Investigador:** descubre, descarga, analiza y prepara skills candidatas.
2. **Catálogo:** almacena, versiona y distribuye skills aprobadas.
3. **Resolver:** analiza un proyecto y activa únicamente las skills necesarias para la tarea actual.

El repositorio no será solamente una colección de prompts. Será una plataforma para administrar capacidades reutilizables de agentes: skills, system prompts, reglas, workflows, plantillas, herramientas, servidores MCP, routers y otros artefactos compatibles.

La idea central es permitir que una persona pueda:

- clonar el repositorio completo;
- instalarlo globalmente, dentro de un proyecto o en una ubicación personalizada;
- descargar todas las skills o solamente categorías específicas;
- investigar nuevas skills por su cuenta;
- mantener catálogos privados;
- revisar candidatos antes de admitirlos;
- ejecutar un comando comprensible para seleccionar las skills necesarias;
- trabajar localmente sin depender de un modelo pesado ni gastar tokens obligatoriamente.

La regla principal del ecosistema será:

> **Una skill puede estar almacenada sin estar aprobada, y puede estar aprobada sin estar activa.**

La instalación puede contener cientos o miles de skills. Sin embargo, cada proyecto o tarea solo debe recibir el conjunto mínimo necesario.

---

## 2. Problema que se quiere resolver

Actualmente, las skills, prompts y herramientas para agentes están dispersos entre repositorios, publicaciones, foros y proyectos independientes. Esto crea varios problemas:

- Es difícil descubrir recursos realmente útiles.
- La popularidad no garantiza calidad ni seguridad.
- Muchas skills duplican funciones existentes.
- No existe una clasificación uniforme.
- Instalar demasiadas skills contamina el contexto del agente.
- Los formatos varían según Claude Code, Codex, Cursor y otras herramientas.
- Un repositorio puede contener instrucciones maliciosas, scripts peligrosos o prompt injection.
- El usuario puede gastar tokens únicamente para decidir qué skill utilizar.
- Las colecciones grandes no suelen explicar por qué una skill debe activarse.
- No siempre existe trazabilidad entre la fuente original, la versión instalada y el contenido activo.

El proyecto pretende resolver estos problemas con una combinación de:

- descubrimiento automatizado;
- análisis de seguridad;
- revisión humana final;
- catálogo local versionado;
- manifiestos estructurados;
- resolución determinista de capacidades;
- adaptadores para diferentes agentes;
- activación mínima y reproducible.

---

## 3. Principios no negociables

### 3.1 Un solo ecosistema, tres motores

Los tres sistemas compartirán repositorio, esquemas, CLI, documentación e índices. Sin embargo, no compartirán permisos ni niveles de confianza.

```text
Investigador  → descubre y propone
Catálogo      → almacena y distribuye
Resolver      → selecciona y activa
```

### 3.2 La revisión humana es la última autoridad

Ningún modelo, puntuación automática o cantidad de estrellas puede admitir directamente una skill al catálogo principal.

El flujo termina con una persona que:

- lee la skill completa;
- revisa sus scripts y archivos relacionados;
- confirma su utilidad;
- revisa su licencia y procedencia;
- entiende los permisos solicitados;
- aprueba o rechaza su entrada.

### 3.3 El contenido externo siempre empieza como hostil

README, issues, comentarios, prompts, scripts, configuraciones y archivos descargados se tratan como datos no confiables. Nunca como instrucciones para el investigador.

### 3.4 Instalada no significa activa

El usuario puede mantener un catálogo local grande. El resolver solo activa las skills necesarias para el proyecto y la tarea actual.

### 3.5 La IA es opcional para resolver

El funcionamiento base debe ser local y determinista. Los modelos se usarán únicamente como mejora opcional o fallback para tareas ambiguas.

### 3.6 Todo debe ser explicable

El sistema debe poder responder:

- por qué encontró una skill;
- por qué la considera útil;
- qué riesgos detectó;
- quién la aprobó;
- por qué fue seleccionada para un proyecto;
- por qué otras skills fueron omitidas;
- qué versión y hash están activos.

### 3.7 Reproducibilidad antes que conveniencia

Cada resolución debe poder congelarse mediante un archivo de bloqueo. Dos personas trabajando sobre el mismo proyecto deben poder obtener el mismo conjunto de skills.

### 3.8 La seguridad no se delega a un prompt

Los prompts de defensa son una capa auxiliar. La seguridad real depende de aislamiento, políticas, validadores deterministas, permisos mínimos, sandbox y revisión humana.

---

## 4. Alcance del catálogo

El catálogo no debe tratar todos los recursos como si fueran la misma clase de objeto. Cada entrada tendrá un tipo explícito.

Tipos iniciales:

```text
skill
system-prompt
prompt-pack
agent-profile
workflow
rule-set
hook
template
tool
cli
mcp-server
model-router
evaluation-suite
adapter
```

Ejemplos conceptuales:

- Una guía de diseño para frontend puede registrarse como `skill`.
- Un conjunto de instrucciones base puede ser `system-prompt`.
- Un router de proveedores o modelos puede ser `model-router`.
- Un servidor que expone herramientas puede ser `mcp-server`.
- Un conjunto de pruebas para agentes puede ser `evaluation-suite`.

La taxonomía debe permitir que recursos como Impeccable y un router de modelos vivan dentro del mismo ecosistema sin forzarlos a compartir el mismo comportamiento.

---

## 5. Arquitectura general del monorepo

Nombre provisional del repositorio: **Skills Ecosystem**. El nombre comercial podrá decidirse después.

```text
skills-ecosystem/
├── apps/
│   ├── cli/
│   ├── researcher/
│   ├── website/
│   └── review-console/
│
├── packages/
│   ├── schema/
│   ├── registry/
│   ├── resolver/
│   ├── project-detector/
│   ├── capability-engine/
│   ├── security-scanner/
│   ├── sandbox-runner/
│   ├── policy-engine/
│   ├── indexer/
│   ├── adapters/
│   └── shared/
│
├── catalog/
│   ├── approved/
│   ├── manifests/
│   ├── indexes/
│   ├── categories/
│   └── provenance/
│
├── policies/
│   ├── admission/
│   ├── permissions/
│   ├── licenses/
│   ├── sources/
│   └── sandbox/
│
├── evaluations/
├── fixtures/
├── examples/
├── docs/
└── scripts/
```

El repositorio principal contendrá únicamente herramientas y contenido aprobado. La investigación utilizará áreas separadas de trabajo.

---

## 6. Zonas de confianza

El ecosistema tendrá varias zonas claramente separadas.

### 6.1 Cuarentena local

```text
.skills-system/research/quarantine/
```

Contendrá descargas originales sin confianza.

Características:

- ignorada por Git;
- excluida del contexto normal de agentes;
- sin acceso a secretos;
- sin ejecución automática;
- con límites de tamaño;
- con limpieza programada;
- con hashes de cada archivo;
- con logs de procedencia.

### 6.2 Candidatos útiles

```text
.skills-system/research/candidates/
```

Aquí llegan las skills que pasaron los análisis automáticos mínimos y fueron catalogadas como potencialmente útiles.

Un candidato conserva:

- copia sanitizada o referencia exacta al contenido;
- commit de origen;
- hash;
- licencia detectada;
- manifiesto provisional;
- resultados de seguridad;
- resultados de utilidad;
- permisos solicitados;
- evidencias y alertas;
- estado de revisión humana.

Los candidatos no se cargan, no se instalan y no se recomiendan automáticamente.

### 6.3 Rama de revisión dentro del mismo repositorio

Para mantener todo dentro de un único repositorio sin contaminar `main`, los candidatos oficiales pueden enviarse a una rama dedicada:

```text
main                 → herramientas y catálogo aprobado
review/candidates    → propuestas esperando revisión humana
```

El investigador prepara un pull request. La persona revisora lee el contenido completo y decide si se fusiona.

### 6.4 Catálogo aprobado

```text
catalog/approved/
```

Solo contiene contenido que pasó la revisión humana final.

Cada entrada aprobada debe incluir:

- fuente exacta;
- versión o commit fijado;
- hash íntegro;
- licencia;
- manifiesto validado;
- fecha de revisión;
- identidad del revisor;
- permisos declarados;
- compatibilidad;
- historial de cambios;
- estado de mantenimiento.

### 6.5 Skills activas de un proyecto

```text
proyecto/.skills/active/
```

Solo contiene o referencia las skills seleccionadas para una tarea o perfil del proyecto.

---

## 7. Motor 1: Investigador

### 7.1 Responsabilidad

El investigador encuentra recursos, recopila evidencia, elimina duplicados, analiza riesgos y prepara candidatos para revisión humana.

No tiene autoridad para aprobar.

### 7.2 Fuentes iniciales

- GitHub.
- Reddit mediante mecanismos permitidos.
- Directorios de skills.
- Listas tipo “awesome”.
- Repositorios oficiales.
- Recomendaciones manuales.
- Archivos o URLs proporcionados por el usuario.

Fuentes futuras:

- Hacker News.
- Blogs técnicos.
- Comunidades de herramientas específicas.
- Marketplaces compatibles.
- Registros privados de organizaciones.

### 7.3 Flujo del investigador

```text
Descubrir candidato
        ↓
Registrar fuente y procedencia
        ↓
Descargar en cuarentena
        ↓
Calcular hashes e inventario
        ↓
Detectar licencia y tipo
        ↓
Ejecutar análisis estático
        ↓
Detectar prompt injection
        ↓
Analizar utilidad y compatibilidad
        ↓
Comparar duplicados
        ↓
Ejecutar pruebas seguras opcionales
        ↓
Aplicar política de admisión preliminar
        ↓
Preparar candidato útil
        ↓
Generar PR de revisión humana
```

### 7.4 Señales de utilidad

La utilidad no se determinará por una sola puntuación. Se analizarán señales independientes:

- problema que resuelve;
- claridad del propósito;
- calidad de documentación;
- ejemplos reales;
- posibilidad de probarla;
- compatibilidad con agentes actuales;
- nivel de especialización;
- redundancia con el catálogo;
- actividad del repositorio;
- historial de mantenimiento;
- recepción comunitaria;
- resultados de evaluaciones;
- costo de contexto;
- permisos exigidos.

### 7.5 Reputación

La reputación será una evidencia, no una autorización.

Se observarán:

- estrellas y evolución;
- forks útiles;
- contribuidores independientes;
- releases;
- issues resueltos;
- tiempo de respuesta;
- discusiones comunitarias;
- menciones positivas y negativas;
- reportes de seguridad;
- señales de promoción artificial;
- antigüedad y continuidad.

### 7.6 Deduplicación

El investigador deberá detectar:

- forks casi idénticos;
- skills copiadas con nombre distinto;
- versiones modificadas mínimamente;
- prompts equivalentes;
- wrappers que solo reempaquetan otra herramienta;
- contenido generado automáticamente sin mejoras.

Cada familia de duplicados debe conservar:

- fuente original probable;
- forks relevantes;
- diferencias;
- candidato recomendado;
- razones de selección.

---

## 8. Seguridad del investigador

Este será el subsistema de mayor riesgo y recibirá un proceso de diseño especial.

### 8.1 Amenazas principales

- Prompt injection dentro de documentos o skills.
- Instrucciones que intenten cambiar la evaluación.
- Scripts de instalación maliciosos.
- Dependencias comprometidas.
- Exfiltración de secretos.
- Lectura del sistema anfitrión.
- Escritura fuera de la carpeta permitida.
- Hooks de Git.
- Symlinks peligrosos.
- Submódulos inesperados.
- Archivos binarios no verificables.
- Código ofuscado.
- Comandos remotos.
- Paquetes con scripts `preinstall` o `postinstall`.
- Modificación de configuraciones globales.
- Acciones de CI maliciosas.
- Ataques de Unicode o contenido invisible.
- Archivos extremadamente grandes.
- Bombas de descompresión.
- Licencias incompatibles o falsas.
- Suplantación de repositorios.
- Manipulación de métricas de reputación.
- Contaminación del catálogo mediante cadenas de dependencias.

### 8.2 Capas defensivas

#### Capa A — Ingesta pasiva

- no ejecutar archivos;
- no instalar paquetes;
- no seguir submódulos automáticamente;
- no cargar plugins;
- no usar secretos;
- no permitir acceso arbitrario a la red;
- limitar tamaño y profundidad.

#### Capa B — Inventario determinista

- hashes;
- tipos MIME;
- extensiones;
- permisos;
- symlinks;
- archivos ocultos;
- ejecutables;
- scripts de ciclo de vida;
- binarios;
- dependencias;
- workflows.

#### Capa C — Escáner estático

Reglas para detectar:

- `curl | bash` y equivalentes;
- descargas y ejecución inmediata;
- PowerShell remoto;
- comandos destructivos;
- acceso a `.env`, SSH, tokens y credenciales;
- elevación de privilegios;
- persistencia en el sistema;
- modificación de perfiles de shell;
- acceso a rutas fuera del workspace;
- ofuscación;
- Base64 sospechoso;
- ejecución dinámica;
- hooks;
- acciones no fijadas;
- instrucciones dirigidas al evaluador.

#### Capa D — Analizador semántico sin herramientas

El modelo recibe contenido delimitado como datos. No tiene:

- shell;
- red;
- filesystem directo;
- secretos;
- capacidad de aprobar;
- capacidad de modificar políticas.

Su salida debe cumplir un esquema estricto y verificable.

#### Capa E — Sandbox de comportamiento

Solo se usa cuando una evaluación necesita ejecutar la skill o sus pruebas.

Características mínimas:

- contenedor o máquina desechable;
- filesystem temporal;
- usuario sin privilegios;
- red deshabilitada por defecto;
- allowlist explícita cuando sea necesaria;
- sin secretos;
- sin directorio personal montado;
- límites de CPU, memoria, disco y tiempo;
- logs completos;
- snapshot antes y después;
- eliminación al finalizar.

#### Capa F — Motor de políticas

El motor aplica decisiones reproducibles. El modelo informa; la política decide.

Ejemplos:

- licencia desconocida → no distribuible;
- acceso a secretos no justificado → rechazo;
- binario no verificable → revisión especial;
- ejecución remota ofuscada → rechazo;
- modificación global silenciosa → rechazo;
- shell legítimo y documentado → revisión humana reforzada;
- skill declarativa sin ejecución → riesgo menor.

#### Capa G — Revisión humana completa

La persona revisora inspecciona la totalidad del contenido y no solamente el resumen generado.

### 8.3 Revisión obligatoria por modelos especializados

Antes de implementar las fases de seguridad, el plan técnico será revisado por dos modelos independientes:

- **Opus 5:** revisión arquitectónica, amenazas, omisiones, coherencia y profundidad.
- **Fable 5:** revisión enfocada en seguridad práctica, aislamiento, políticas, evasiones y pruebas adversariales.

El paquete posterior a este documento incluirá un prompt específico para que ambos revisen el plan completo y propongan mejoras antes de comenzar las WAVEs de seguridad.

### 8.4 Regla obligatoria para las WAVEs de seguridad

Toda WAVE cuyo objetivo principal incluya seguridad deberá ser trabajada por **Fable 5**.

Esto incluye, como mínimo:

- threat model;
- cuarentena;
- escáner estático;
- prompt-injection defense;
- sandbox;
- motor de políticas;
- permisos;
- integridad y firmas;
- supply chain;
- pruebas adversariales;
- hardening del investigador;
- revisión de canales de actualización.

Opus 5 actuará como revisión independiente posterior. Cuando la WAVE sea especialmente crítica, se recomendará una segunda pasada de Fable 5 después de la revisión de Opus 5.

La seguridad no se considerará cerrada porque “las pruebas pasan”. Debe existir evidencia de ataques simulados, límites documentados y riesgos residuales.

---

## 9. Motor 2: Catálogo y almacenamiento

### 9.1 Responsabilidad

El catálogo conserva el contenido aprobado, sus metadatos, su procedencia y sus versiones.

### 9.2 Manifiesto canónico

Cada entrada debe contar con un manifiesto legible por máquinas.

Ejemplo conceptual:

```yaml
schema_version: 1
id: organization.skill-name
name: Skill Name
type: skill
version: 1.2.0

source:
  provider: github
  repository: organization/repository
  commit: abc123
  path: skills/example

license:
  spdx: MIT
  verified: true

capabilities:
  - frontend.visual-review
  - frontend.responsive-layout

triggers:
  - frontend
  - landing page
  - responsive
  - visual audit

negative_triggers:
  - backend only
  - database migration

project_signals:
  files:
    - package.json
    - "*.tsx"
  dependencies:
    - react
    - next

permissions:
  filesystem:
    read: project
    write: project
  network: none
  shell: optional
  secrets: none

compatibility:
  agents:
    - claude-code
    - codex

context:
  estimated_tokens: 6000
  loading_mode: on-demand

trust:
  status: approved
  reviewed_at: YYYY-MM-DD
  reviewer: human-id

integrity:
  content_hash: sha256:...
```

### 9.3 Estados del catálogo

```text
discovered
quarantined
candidate
human-review
approved
recommended
deprecated
unmaintained
compromised
rejected
```

### 9.4 Versionado

Una skill aprobada no se actualiza silenciosamente.

Cada nueva versión debe:

- volver a calcular hashes;
- comparar cambios;
- repetir análisis relevantes;
- revisar nuevos permisos;
- marcar cambios de riesgo;
- requerir revisión humana según política.

### 9.5 Distribución

El catálogo podrá distribuirse de distintas formas:

- clonación completa de Git;
- sparse checkout por categoría;
- release comprimido;
- paquetes individuales;
- actualización incremental;
- espejo privado;
- registro remoto opcional.

---

## 10. Modos de instalación

### 10.1 Instalación global

Ubicación sugerida:

```text
~/.local/share/skills-ecosystem/
```

Ventajas:

- una sola descarga;
- disponible para todos los proyectos;
- rápida;
- sin duplicar contenido.

### 10.2 Instalación local por proyecto

```text
mi-proyecto/.skills/
```

Ventajas:

- reproducible;
- fácil de compartir;
- permite reglas y skills privadas;
- el proyecto controla sus versiones.

### 10.3 Ubicación personalizada

El usuario podrá definir una ruta propia:

```bash
skills init --path ~/Dev/my-skills
```

### 10.4 Precedencia de fuentes

Orden recomendado:

```text
1. Skills del proyecto
2. Catálogo privado del usuario
3. Catálogo global local
4. Catálogo oficial remoto
```

La precedencia debe ser configurable y visible.

---

## 11. Motor 3: Resolver de skills

### 11.1 Responsabilidad

El resolver analiza un proyecto y una tarea para seleccionar el conjunto mínimo suficiente de skills.

No debe cargar todo el catálogo en el contexto del agente.

### 11.2 Nombre del comando

`sort` puede confundirse con ordenar resultados. El término técnico recomendado es `resolve`.

CLI:

```bash
skills resolve --task "Mejorar la landing page"
```

Comando dentro de un agente:

```text
/skills resolve Mejorar la landing page
```

También pueden existir alias más amigables:

```text
/skills choose
/skills prepare
```

Pero `resolve` debe ser el comando canónico.

### 11.3 Flujo del resolver

```text
Escanear proyecto
      ↓
Detectar stack
      ↓
Interpretar tarea
      ↓
Derivar capacidades necesarias
      ↓
Buscar candidatos compatibles
      ↓
Eliminar incompatibles y conflictos
      ↓
Calcular cobertura, confianza y costo
      ↓
Seleccionar conjunto mínimo
      ↓
Mostrar explicación
      ↓
Activar mediante adaptador
      ↓
Crear skills.lock
```

### 11.4 Detección determinista del proyecto

Se analizarán señales como:

- archivos de configuración;
- dependencias;
- extensiones predominantes;
- estructura de carpetas;
- frameworks;
- herramientas de pruebas;
- CI;
- contenedores;
- bases de datos;
- infraestructura;
- lenguaje.

Ejemplos:

```text
package.json        → JavaScript/TypeScript
next.config.*       → Next.js
pyproject.toml      → Python
alembic.ini         → Alembic
pubspec.yaml        → Flutter
Cargo.toml          → Rust
docker-compose.yml  → Docker
```

### 11.5 Interpretación de la tarea sin modelo pesado

El sistema base combinará:

- taxonomía de capacidades;
- triggers;
- alias y sinónimos;
- SQLite FTS5 o BM25;
- señales del repositorio;
- reglas deterministas;
- historial de selecciones;
- preferencias del usuario.

La tarea no se compara directamente contra miles de skills completas. Primero se traduce a capacidades.

Ejemplo:

```text
"Haz que la página se vea mejor en móvil"
```

Se transforma en:

```text
frontend.responsive-layout
frontend.visual-review
frontend.accessibility
```

### 11.6 Selección por cobertura mínima

El resolver tratará la selección como un problema de cobertura con costo.

Debe buscar el conjunto más pequeño que cubra las capacidades necesarias sin introducir redundancia excesiva.

Factores positivos:

- compatibilidad con el stack;
- relación con la tarea;
- cobertura de capacidades;
- confianza;
- disponibilidad local;
- preferencia del usuario;
- mantenimiento;
- calidad comprobada.

Penalizaciones:

- conflictos;
- capacidades redundantes;
- permisos peligrosos;
- costo alto de contexto;
- versión desactualizada;
- falta de integridad;
- incompatibilidad con el agente;
- descarga remota innecesaria.

### 11.7 Capas opcionales de IA

#### Nivel 0 — Sin IA

Modo por defecto.

- reglas;
- FTS/BM25;
- taxonomía;
- selección determinista;
- cero llamadas externas.

#### Nivel 1 — Clasificador local pequeño

Opcional para tareas ambiguas. Solo recibe la tarea, señales mínimas del proyecto y taxonomía.

#### Nivel 2 — Modelo remoto económico

Solo con autorización explícita y presupuesto configurable.

#### Nivel 3 — Resolución profunda

Para tareas complejas que mezclan arquitectura, seguridad, infraestructura y varios dominios.

El usuario nunca estará obligado a utilizar un modelo pesado para una resolución normal.

### 11.8 Explicabilidad

Ejemplo de salida:

```text
Proyecto detectado:
- Next.js
- TypeScript
- Tailwind
- Playwright

Tarea:
- Mejorar diseño responsive y accesibilidad

Seleccionadas:
- visual-design-review
- responsive-layout
- accessibility-audit

Omitidas:
- backend-security
  Motivo: no corresponde a la tarea.

- alternative-ui-review
  Motivo: redundante con visual-design-review.

Costo estimado de contexto:
7,800 tokens
```

---

## 12. Activación y adaptadores

Una skill canónica puede necesitar formatos distintos según el agente.

Adaptadores iniciales previstos:

- Claude Code.
- Codex.
- Cursor.
- GitHub Copilot.
- formato genérico basado en archivos.

El resolver no debe modificar el contenido aprobado de forma silenciosa. Los adaptadores deben generar wrappers o representaciones derivadas, conservando referencia al original.

Ejemplo:

```text
catalog/approved/skill-x/
       ↓ adapter
proyecto/.claude/skills/skill-x/

catalog/approved/skill-x/
       ↓ adapter
proyecto/.codex/skills/skill-x/
```

---

## 13. Archivo `skills.lock`

Cada resolución debe poder congelarse.

Ejemplo conceptual:

```yaml
lock_version: 1
catalog:
  source: official
  commit: 81da92c

project:
  fingerprint: sha256:...

task:
  normalized: frontend.responsive-improvement

skills:
  - id: organization.visual-review
    version: 1.4.0
    hash: sha256:...
    source: global
    reasons:
      - frontend.visual-review
      - frontend.responsive-layout

resolver:
  version: 0.1.0
  ai_mode: disabled
```

Beneficios:

- reproducibilidad;
- detección de modificaciones;
- auditoría;
- colaboración;
- rollback;
- comparación de resoluciones.

---

## 14. Integridad y confianza local

Una skill local puede ser más privada y conveniente, pero no necesariamente más segura.

El sistema mostrará estados como:

```text
✓ Coincide con versión aprobada
△ Modificada localmente
✗ Hash desconocido
! Versión marcada como comprometida
```

Configuración posible:

```yaml
sources:
  priority:
    - project
    - user
    - official

integrity:
  allow_modified_local: true
  warn_on_hash_mismatch: true
  block_compromised: true
```

---

## 15. Experiencia de usuario prevista

### 15.1 Instalación

```bash
git clone <repo>
cd <repo>
./install.sh
skills sync
```

### 15.2 Uso en un proyecto

```bash
cd ~/Dev/mi-proyecto
skills scan
skills resolve --task "Mejorar autenticación con Google"
skills explain
skills activate --target codex
```

### 15.3 Investigación propia

```bash
skills research github <url>
skills research search "frontend accessibility skill"
skills research review <candidate-id>
```

El usuario podrá mantener sus candidatos localmente sin proponerlos al catálogo oficial.

### 15.4 Comandos previstos

```text
skills init
skills sync
skills scan
skills resolve
skills explain
skills activate
skills deactivate
skills list
skills search
skills install
skills update
skills audit
skills doctor
skills research
skills review
skills verify
```

---

## 16. Website y consola de revisión

La página web será una interfaz del catálogo, no la fuente de verdad.

Funciones previstas:

- búsqueda por capacidad;
- categorías;
- comparación de skills;
- historial de versiones;
- nivel de confianza;
- permisos;
- compatibilidad;
- reportes de seguridad;
- estado de mantenimiento;
- instalación rápida;
- visualización del manifiesto;
- procedencia;
- candidatos esperando revisión, cuando el usuario tenga permisos.

La consola de revisión debe facilitar que el humano lea:

- todos los archivos de la skill;
- diferencias respecto a versiones anteriores;
- scripts;
- dependencias;
- permisos;
- alertas;
- resultados de sandbox;
- comentarios de Opus 5 y Fable 5;
- decisión final.

---

## 17. Gobernanza

### 17.1 Roles

- **Descubridor:** propone fuentes o ejecuta el investigador.
- **Analizador automático:** genera evidencia, nunca aprueba.
- **Revisor humano:** lee y decide admisión.
- **Mantenedor:** gestiona catálogo, versiones y políticas.
- **Revisor de seguridad:** valida cambios sensibles.

### 17.2 Admisión

Una entrada aprobada debe tener:

- procedencia verificable;
- licencia compatible;
- manifiesto completo;
- evaluación de seguridad;
- revisión humana;
- hash;
- historial;
- permisos explícitos.

### 17.3 Revocación

Una skill puede retirarse o marcarse comprometida si:

- cambia su fuente de forma sospechosa;
- aparece una vulnerabilidad;
- se descubre comportamiento no declarado;
- pierde licencia;
- es reemplazada por una opción superior;
- queda abandonada;
- sus dependencias son comprometidas.

El sistema debe advertir a proyectos que la tengan en `skills.lock`.

---

## 18. Evaluación de calidad

Cada dimensión se mantiene separada:

```text
Utility Score
Quality Score
Maintenance Score
Security Score
Community Score
Compatibility Score
Evidence Confidence
Context Cost
```

No se usará una media simple para aprobar.

Algunas dimensiones funcionarán como puertas:

- seguridad crítica → bloqueo;
- licencia desconocida → no distribución;
- procedencia dudosa → revisión reforzada;
- confianza insuficiente → candidato experimental;
- mantenimiento bajo → advertencia o deprecación.

---

## 19. Pruebas requeridas

### 19.1 Investigador

- fuentes falsas;
- repositorios inexistentes;
- rate limits;
- descargas incompletas;
- archivos gigantes;
- symlinks;
- submódulos;
- prompt injection;
- scripts maliciosos;
- duplicados;
- licencias ambiguas;
- cambios de procedencia.

### 19.2 Resolver

- detección de stack;
- tareas claras y ambiguas;
- selección mínima;
- conflictos;
- redundancia;
- precedencia local/global;
- funcionamiento sin red;
- funcionamiento sin IA;
- hashes modificados;
- skills faltantes;
- adaptadores incompatibles;
- reproducibilidad.

### 19.3 Seguridad

- intentos de escape del sandbox;
- exfiltración simulada;
- prompt injection indirecta;
- archivos Unicode invisibles;
- comandos codificados;
- dependencias maliciosas;
- hooks;
- acciones CI inseguras;
- manipulación del reporte;
- intento de autoaprobación;
- contaminación entre candidatos.

---

## 20. Observabilidad y auditoría

Toda operación sensible debe dejar trazabilidad.

Registrar:

- fuente consultada;
- fecha;
- commit;
- hashes;
- comandos ejecutados;
- red utilizada;
- archivos creados;
- resultados de escáner;
- modelo utilizado, cuando aplique;
- versión del prompt de análisis;
- política aplicada;
- decisión automática;
- decisión humana;
- versión finalmente aprobada.

Los logs no deben almacenar secretos ni contenido externo innecesario.

---

## 21. Privacidad y costos

El resolver local será la opción predeterminada para evitar:

- enviar código privado a servicios externos;
- gastar tokens;
- depender de conexión;
- revelar el propósito del proyecto;
- introducir latencia.

Cuando se use un modelo remoto, el sistema debe mostrar antes:

- qué información se enviará;
- proveedor;
- modelo;
- presupuesto máximo;
- propósito;
- posibilidad de cancelar.

---

## 22. Relación futura con Dvadi y routers

El ecosistema puede integrarse con Dvadi sin fusionarse con él.

```text
Dvadi define la tarea o WAVE
          ↓
Resolver selecciona skills
          ↓
Router opcional elige modelo
          ↓
Implementador ejecuta
          ↓
Revisor valida
```

El resolver responde “qué capacidades cargar”. Un model router responde “qué modelo ejecutar”. Son responsabilidades distintas.

---

## 23. Riesgos arquitectónicos principales

### 23.1 Convertir el catálogo en un repositorio inmanejable

Mitigación:

- manifests pequeños;
- descarga por categorías;
- sparse checkout;
- releases parciales;
- índices separados;
- contenido pesado fuera del núcleo.

### 23.2 Confiar demasiado en métricas de popularidad

Mitigación:

- separar reputación, utilidad y seguridad;
- detectar manipulación;
- exigir evidencia técnica;
- revisión humana.

### 23.3 Hacer que el resolver dependa de IA costosa

Mitigación:

- resolución determinista por defecto;
- taxonomía;
- FTS/BM25;
- reglas;
- IA local opcional;
- modelos remotos solo como fallback.

### 23.4 Permitir que una skill candidata influya en el investigador

Mitigación:

- delimitación de datos;
- modelo sin herramientas;
- sandbox;
- política externa;
- no autoaprobación;
- aislamiento entre candidatos.

### 23.5 Confundir almacenamiento con activación

Mitigación:

- catálogo separado de `active/`;
- activación explícita;
- `skills.lock`;
- adaptadores controlados.

### 23.6 Mantener miles de skills desactualizadas

Mitigación:

- revalidación incremental;
- monitoreo de fuentes;
- estados `unmaintained` y `deprecated`;
- alertas a proyectos;
- priorización por uso.

---

## 24. Decisiones que deben aprobarse antes del plan por WAVEs

1. Mantener un monorepo para los tres motores.
2. Mantener `main` libre de candidatos no aprobados.
3. Usar cuarentena y candidatos locales para investigaciones personales.
4. Usar una rama o PR de revisión para candidatos oficiales.
5. Exigir revisión humana completa antes de aprobar.
6. Mantener el resolver funcional sin IA externa.
7. Usar `skills resolve` como comando canónico.
8. Soportar instalación global, local y personalizada.
9. Incluir tipos distintos además de `skill`.
10. Crear `skills.lock` para reproducibilidad.
11. Hacer obligatoria la intervención de Fable 5 en WAVEs de seguridad.
12. Hacer que Opus 5 revise el plan de seguridad y las implementaciones críticas.
13. Incluir en el futuro paquete un prompt conjunto de revisión para Opus 5 y Fable 5.
14. No comenzar implementación crítica de seguridad hasta incorporar sus observaciones.

---

## 25. Entregable posterior a la aprobación

Después de aprobar este documento se preparará un archivo `.zip` con el plan de implementación completo.

El paquete incluirá, como mínimo:

```text
00-master-context/
01-architecture/
02-threat-model/
03-roadmap/
04-waves/
05-prompts/
06-security-reviews/
07-schemas/
08-testing/
09-governance/
10-acceptance-criteria/
```

Cada WAVE contendrá:

- contexto;
- objetivo;
- alcance;
- fuera de alcance;
- arquitectura esperada;
- archivos o módulos previstos;
- pasos de implementación;
- riesgos;
- pruebas;
- criterios de aceptación;
- reporte esperado;
- modelo recomendado;
- esfuerzo recomendado;
- reglas de commit y push;
- condiciones de escalamiento;
- prompt listo para copiar.

Las WAVEs de seguridad marcarán explícitamente:

```text
Implementador obligatorio: Fable 5
Revisor independiente: Opus 5
Revisión adicional: Fable 5 cuando Opus detecte cambios críticos
```

También se incluirá un prompt previo para que Opus 5 y Fable 5 auditen el plan completo, propongan correcciones y fortalezcan la seguridad antes de iniciar dichas WAVEs.

---

## 26. Conclusión

La propuesta no debe convertirse únicamente en una colección gigantesca de prompts. Su valor real estará en la combinación de cuatro propiedades:

1. **Descubrimiento amplio.**
2. **Admisión segura y humana.**
3. **Distribución local y reproducible.**
4. **Selección mínima por proyecto y tarea.**

La arquitectura recomendada conserva todo dentro de un mismo ecosistema y permite que el usuario clone, investigue, almacene y utilice skills de forma flexible. Al mismo tiempo, separa físicamente los niveles de confianza y evita que material no revisado se mezcle con el catálogo principal.

El resolver no dependerá de un modelo pesado. La mayor parte de su trabajo se realizará mediante manifiestos, señales del proyecto, taxonomía, búsqueda local y un algoritmo de cobertura mínima. La IA quedará como una mejora opcional.

El investigador será el componente más delicado. Por esa razón, las decisiones de seguridad se someterán a revisión especializada antes de su implementación y todas las WAVEs de seguridad serán trabajadas obligatoriamente por Fable 5, con Opus 5 como revisor independiente.

Este documento define la dirección del producto. El siguiente paso, después de su aprobación, será convertir estas decisiones en arquitectura detallada, contratos y WAVEs ejecutables dentro del paquete de implementación.
