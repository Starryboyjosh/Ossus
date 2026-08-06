# Documento maestro V2 — Fundamentos previos a las WAVEs
## Ecosistema de catálogo confiable y resolución mínima de skills

**Estado:** propuesta revisada para aprobación humana  
**Versión:** 0.2-pre-wave  
**Fecha de referencia:** 1 de agosto de 2026  
**Propósito:** resolver las decisiones estructurales que deben quedar cerradas antes de producir el paquete `.zip` con el plan de implementación por fases y WAVEs.

---

## 0. Resumen de la revisión

La primera propuesta tenía una dirección correcta, pero colocaba demasiados componentes dentro de la primera implementación y dejaba subespecificadas dos piezas críticas: la taxonomía de capacidades y la seguridad del canal de activación.

La versión V2 adopta estos cambios:

1. **El resolver se construye antes que el investigador.**
2. **La taxonomía de capacidades se convierte en una especificación propia y versionada.**
3. **El threat model comienza por el riesgo real:** una skill aprobada introduce instrucciones y, en algunos casos, scripts o permisos dentro de la sesión del agente del usuario.
4. **El manifiesto canónico no pertenece al autor externo.** Lo redacta y firma el proceso de curación.
5. **Los candidatos no viven en una rama del repositorio principal.** Se procesan en un fork o repositorio de staging sin secretos ni CI privilegiado.
6. **La revisión humana se vuelve escalonada según riesgo**, sin eliminar la autoridad humana final.
7. **El catálogo adopta un modelo de índice primero**, con vendorización limitada y explícita.
8. **El MVP admite tres tipos:** `skill`, `prompt-pack` y `mcp-server`.
9. **No se construirá un sandbox propio ni un escáner estático propio.** Se delegará aislamiento en runners desechables existentes y análisis en herramientas como Semgrep y escáneres de dependencias.
10. **Se definen 50 casos dorados antes de ajustar el resolver.**
11. **La compatibilidad se modela por superficie y runtime**, no con una bandera imprecisa como `claude: true`.
12. **Los nombres de modelos no forman parte del protocolo permanente.** Se fijan en la configuración de implementación del proyecto.

---

# Parte I — Tesis del producto

## 1. Qué producto se construirá

El proyecto seguirá siendo un único ecosistema clonable, pero su valor principal cambia de orden:

```text
Catálogo confiable  → describe qué existe y bajo qué condiciones puede usarse
Resolver             → elige el conjunto mínimo para un proyecto y una tarea
Adaptadores          → materializan la selección en una superficie concreta
Investigador         → descubre nuevos candidatos, después de validar lo anterior
```

La regla central permanece:

> **Almacenada no significa aprobada; aprobada no significa instalada; instalada no significa activa.**

El usuario podrá mantener muchas skills disponibles localmente, pero el sistema solo expondrá a cada agente el subconjunto seleccionado y permitido.

## 2. Diferencia frente a marketplaces existentes

Claude Code y Codex ya poseen mecanismos nativos para:

- descubrir skills mediante nombre y descripción;
- cargar el cuerpo de una skill solo cuando se selecciona;
- mantener skills globales y específicas de un proyecto;
- distribuir extensiones mediante plugins o marketplaces;
- invocar skills de forma implícita o explícita.

Por tanto, este proyecto **no intentará reemplazar esos mecanismos**.

Su diferencia será:

1. **Confianza:** procedencia, revisión, permisos observados, licencia y hash.
2. **Compatibilidad cruzada:** distinguir estándar general, extensiones de Claude, Codex, CLI, MCP y otros runtimes.
3. **Resolución a escala:** seleccionar pocas skills antes de que el host reciba un catálogo demasiado grande.
4. **Política local:** permitir que el usuario decida qué fuentes, riesgos y superficies acepta.
5. **Reproducibilidad:** fijar fuentes, versiones, hashes, adaptadores y razones en `skills.lock`.
6. **Evaluación:** medir precisión, cobertura, falsas activaciones y ahorro de contexto.

## 3. Alcance funcional V0

La primera versión funcional incluirá solamente:

- un catálogo manual de aproximadamente 20 entradas;
- tres tipos de recurso;
- taxonomía de capacidades V1;
- manifiestos canónicos redactados por revisores;
- búsqueda y filtrado local;
- detección determinista del proyecto;
- resolución determinista de capacidades y skills;
- explicación de selección y exclusión;
- `skills.lock`;
- un adaptador funcional para Claude Code;
- 50 casos dorados;
- CLI mínima.

No incluirá:

- crawler de Reddit;
- crawler de GitHub;
- investigador automático;
- sitio web público;
- review console visual;
- sandbox propio;
- escáner estático propio;
- marketplace propio de instalación;
- adaptadores completos para cinco agentes;
- catorce tipos de recursos;
- router de modelos.

---

# Parte II — Compatibilidad y clasificación

## 4. Los tres tipos iniciales

### 4.1 `skill`

Paquete de instrucciones reutilizables con metadatos y, opcionalmente, referencias, assets o scripts.

### 4.2 `prompt-pack`

Conjunto declarativo de prompts, system prompts, checklists o plantillas que no depende necesariamente del estándar Agent Skills.

### 4.3 `mcp-server`

Servidor MCP que expone herramientas, recursos o prompts. Se trata como una capacidad ejecutable y de mayor riesgo que una skill declarativa.

Los tipos adicionales quedarán reservados para versiones futuras y requerirán una propuesta de esquema propia.

## 5. Dimensiones de búsqueda obligatorias

El buscador no dependerá de una única categoría. Cada entrada se clasificará por varias dimensiones independientes.

### 5.1 Tipo de recurso

```text
skill
prompt-pack
mcp-server
```

### 5.2 Categoría funcional

Ejemplos:

```text
frontend
backend
security
testing
devops
documentation
architecture
data
ai-agents
workflow
```

### 5.3 Superficie compatible

```text
agent-skills-standard
claude-code-cli
claude-agent-sdk
claude-api-host
codex-cli
codex-ide
codex-desktop
generic-terminal-agent
generic-mcp-client
standalone-cli
```

No se utilizará simplemente `claude` o `codex`, porque una misma capacidad puede funcionar en una superficie y no en otra.

### 5.4 Dependencia de runtime

```text
instruction-only
filesystem-only
shell-required
network-required
mcp-required
external-cli-required
host-api-required
```

### 5.5 Portabilidad

```text
portable-standard
portable-with-adapter
host-extension
host-exclusive
unknown
```

### 5.6 Scope de instalación admitido

```text
project
user-global
plugin
custom-path
remote-only
```

### 5.7 Riesgo operacional

```text
R0-declarative
R1-readonly
R2-local-write
R3-shell-or-network
R4-credentials-or-remote-actions
R5-privileged-or-destructive
```

## 6. Consultas esperadas

```bash
skills search --category security
skills search --host claude-code-cli
skills search --host codex-cli --category frontend
skills search --runtime standalone-cli
skills search --portable portable-standard
skills search --risk-max R1
skills search --type mcp-server --runtime network-required
skills search --source local
skills search --source official --category testing
```

La CLI podrá combinar filtros. Una búsqueda no activa ni instala nada.

## 7. Compatibilidad no equivale a seguridad

Una entrada puede ser técnicamente compatible y aun así no ser segura para la política del usuario.

Ejemplo:

```yaml
compatibility:
  surfaces:
    - claude-code-cli
    - codex-cli
  portability: portable-with-adapter

runtime:
  requirements:
    - shell-required
    - network-required

risk:
  tier: R3-shell-or-network
```

El resolver puede encontrarla, pero la política puede excluirla.

---

# Parte III — Taxonomía de capacidades V1

## 8. Objetivo

La taxonomía será el contrato semántico entre:

- proyectos detectados;
- tareas solicitadas;
- manifiestos canónicos;
- resolución;
- evaluaciones;
- explicaciones.

No será una lista libre de tags.

## 9. Autoridad y gobernanza

La taxonomía vivirá en:

```text
spec/capabilities/v1/capabilities.yaml
spec/capabilities/v1/aliases.yaml
spec/capabilities/v1/deprecations.yaml
spec/capabilities/v1/examples/
```

La gobernanza inicial tendrá tres roles:

- **Maintainer de taxonomía:** custodia coherencia y versiones.
- **Revisor de dominio:** valida cambios dentro de un área funcional.
- **Revisor de resolver:** confirma que el cambio puede evaluarse y no introduce ambigüedad innecesaria.

Una capacidad nueva requerirá:

1. definición breve;
2. inclusión y exclusión explícitas;
3. al menos tres ejemplos positivos;
4. al menos tres ejemplos negativos;
5. relación con capacidades vecinas;
6. alias aceptados;
7. caso dorado nuevo o modificado;
8. aprobación de dos personas, una de ellas fuera del autor del cambio.

## 10. Reglas de nombres

Formato:

```text
dominio.capacidad
```

Reglas:

- minúsculas;
- singular conceptual;
- nombres estables y no ligados a marcas;
- máximo dos niveles en V1;
- no crear capacidades por framework;
- no duplicar sinónimos;
- no mezclar tarea, tecnología y resultado en un mismo identificador.

Incorrecto:

```text
react-responsive-impeccable
fix-ui
secure-fastapi-login
```

Correcto:

```text
frontend.responsive-layout
frontend.visual-design
security.identity-access
```

Los frameworks pertenecen a `project_signals` y `compatibility`, no a la taxonomía principal.

## 11. Capacidades V1

### Proyecto y arquitectura

1. `project.discovery`
2. `architecture.system-design`
3. `architecture.api-design`
4. `architecture.data-modeling`

### Frontend

5. `frontend.implementation`
6. `frontend.visual-design`
7. `frontend.responsive-layout`
8. `frontend.accessibility`
9. `frontend.motion`
10. `frontend.performance`

### Backend

11. `backend.api-implementation`
12. `backend.authentication`
13. `backend.authorization`
14. `backend.data-access`
15. `backend.background-jobs`

### Base de datos

16. `database.schema-design`
17. `database.migrations`
18. `database.query-performance`

### Pruebas

19. `testing.unit`
20. `testing.integration`
21. `testing.end-to-end`
22. `testing.visual`

### Calidad

23. `quality.code-review`
24. `quality.refactoring`
25. `quality.debugging`

### Seguridad

26. `security.threat-modeling`
27. `security.secure-coding`
28. `security.dependency-audit`
29. `security.secrets-handling`
30. `security.identity-access`
31. `security.supply-chain`

### DevOps

32. `devops.containers`
33. `devops.ci-cd`
34. `devops.deployment`
35. `devops.observability`

### Documentación

36. `documentation.technical-writing`
37. `documentation.api-reference`

### Flujo de desarrollo

38. `workflow.git`
39. `workflow.release`

### IA y agentes

40. `ai.prompt-design`
41. `ai.agent-orchestration`
42. `ai.mcp-integration`

### Datos

43. `data.analysis`
44. `data.visualization`

## 12. Alias y sinónimos

Los alias no crean capacidades nuevas.

```yaml
aliases:
  mobile-friendly: frontend.responsive-layout
  a11y: frontend.accessibility
  auth: backend.authentication
  permissions: backend.authorization
  oauth-security: security.identity-access
  pipeline: devops.ci-cd
```

El sistema conservará el texto original para explicaciones, pero resolverá siempre al identificador canónico.

## 13. Versionado

La taxonomía utiliza versionado semántico independiente del programa.

- **PATCH:** documentación, ejemplos y alias sin cambiar significado.
- **MINOR:** nuevas capacidades o deprecaciones compatibles.
- **MAJOR:** fusiones, divisiones o cambios semánticos incompatibles.

El manifiesto declara:

```yaml
capability_schema: 1.0.0
```

`skills.lock` registra la versión exacta utilizada.

## 14. Deprecaciones

Una capacidad deprecada mantiene un alias durante al menos una versión mayor.

```yaml
deprecated:
  frontend.ui-review:
    replacement:
      - frontend.visual-design
      - frontend.accessibility
    removal_after: 2.0.0
```

## 15. Skills sin mapeo

Una skill que no mapea limpiamente a la taxonomía no inventa su propia capacidad.

Estados permitidos:

```text
unmapped
needs-taxonomy-review
out-of-scope
```

No puede participar en resolución automática hasta ser mapeada.

---

# Parte IV — Manifiesto canónico y confianza

## 16. Dos manifiestos diferentes

### 16.1 Manifiesto de origen

Es cualquier metadata proporcionada por el autor externo. Se conserva como evidencia, pero se considera no confiable.

```text
origin-manifest.yaml
```

### 16.2 Manifiesto canónico

Es el contrato utilizado por el catálogo y el resolver.

```text
canonical-manifest.yaml
```

Lo redacta el proceso de curación y lo aprueba una persona autorizada. Nunca se importa directamente del repositorio de origen.

## 17. Campos bajo control del catálogo

Los siguientes campos solo pueden existir en el manifiesto canónico:

- capacidades;
- triggers normalizados;
- exclusiones;
- compatibilidad;
- permisos observados;
- runtime requerido;
- nivel de riesgo;
- costo de contexto medido;
- estado de confianza;
- procedencia verificada;
- hashes;
- licencia y modo de distribución;
- decisión de revisión;
- adaptadores permitidos.

El autor externo puede sugerirlos, pero no controlarlos.

## 18. Protección contra manifiestos adversariales

Controles obligatorios:

1. límite de triggers por entrada;
2. triggers redactados por revisores;
3. detección de solapamiento anormal;
4. penalización por cobertura excesivamente amplia;
5. prohibición de wildcard semántico;
6. permisos derivados del contenido y comportamiento observado, no solo declarados;
7. `negative_triggers` solo como evidencia auxiliar, nunca como mecanismo de ocultamiento;
8. comparación automática entre manifiesto canónico y contenido real;
9. firma del manifiesto o firma del commit de aprobación;
10. revisión especial cuando una actualización amplía capacidades, permisos o superficies.

## 19. Ejemplo mínimo

```yaml
id: example.frontend-review
schema_version: 1.0.0
capability_schema: 1.0.0
resource_type: skill

source:
  mode: remote-index
  repository: example/repo
  commit: 0123456789abcdef
  tree_hash: sha256:...

capabilities:
  required:
    - frontend.visual-design
    - frontend.responsive-layout
  optional:
    - frontend.accessibility

compatibility:
  surfaces:
    - agent-skills-standard
    - claude-code-cli
    - codex-cli
  portability: portable-standard

runtime:
  requirements:
    - instruction-only

risk:
  tier: R0-declarative

review:
  tier: light-human
  approved_commit: abcdef...
  reviewer_ids:
    - reviewer-001

context:
  measured_tokens: 2800
  measurement_method: tokenizer-v1
```

---

# Parte V — Threat model revisado

## 20. Activo crítico principal

> **El catálogo aprobado es un canal de instrucciones hacia el agente del usuario.**

Una entrada aprobada puede influir en:

- qué instrucciones recibe el modelo;
- qué herramientas considera disponibles;
- qué comandos ejecuta;
- qué archivos lee o modifica;
- qué endpoints contacta;
- qué secretos intenta solicitar;
- qué otras skills activa;
- qué decisiones presenta como necesarias.

Este riesgo existe incluso cuando el investigador ya terminó y la skill parece legítima.

## 21. Fronteras de confianza

```text
Fuente externa
    ↓ hostil
Staging / cuarentena
    ↓ analizado, no confiable
Candidato
    ↓ revisado según riesgo
Catálogo aprobado
    ↓ permitido, no universalmente seguro
Instalación local
    ↓ sujeto a política del usuario
Conjunto activo
    ↓ instrucciones expuestas al host
Sesión del agente
    ↓ acciones sujetas a permisos del host
Sistema del usuario
```

Aprobado significa “aceptado bajo condiciones documentadas”, no “seguro en cualquier contexto”.

## 22. Amenazas principales

### 22.1 Inyección persistente

La skill contiene instrucciones para ignorar al usuario, alterar prioridades, ocultar acciones o inducir llamadas innecesarias.

### 22.2 Escalamiento por herramientas

La skill preautoriza o induce uso de shell, escritura, red, MCP o credenciales.

### 22.3 Activación excesiva

Triggers o descripciones amplias provocan que la skill entre en tareas no relacionadas.

### 22.4 Encadenamiento de skills

Una skill induce al agente a cargar otra capacidad más peligrosa.

### 22.5 Actualización comprometida

Un upstream legítimo publica una versión posterior maliciosa.

### 22.6 Confusión de superficie

Una skill segura como texto declarativo se vuelve peligrosa en un host que interpreta extensiones, scripts o permisos adicionales.

### 22.7 Dependencia externa mutable

Scripts, instaladores o MCP servers descargan contenido no fijado.

### 22.8 Suplantación

Un repositorio imita el nombre o metadata de una skill reconocida.

### 22.9 Fuga de datos

La skill solicita enviar código, logs, prompts o secretos a un servicio externo.

### 22.10 Ataque de CI durante revisión

Un candidato introduce workflows, hooks o configuraciones que se ejecutan en el repositorio de revisión.

## 23. Controles de activación

El resolver y los adaptadores deben aplicar:

- activación explícita por allowlist;
- límite máximo de skills activas;
- bloqueo por nivel de riesgo;
- exclusión de scripts no requeridos;
- instalación por hash o commit fijado;
- verificación antes de cada activación;
- separación entre metadata indexada y contenido ejecutable;
- modo `instruction-only` cuando sea posible;
- explicación visible de permisos;
- configuración del host generada con mínimo privilegio;
- posibilidad de desactivar invocación automática;
- advertencia cuando el host no ofrece aislamiento real.

## 24. La selección no es un sandbox

Ocultar una skill del contexto del modelo no vuelve inaccesibles sus archivos si el agente tiene lectura o shell sobre el sistema. Por ello:

- `active/` contendrá únicamente enlaces o copias de las seleccionadas;
- el catálogo completo no debe montarse dentro del workspace del agente cuando no sea necesario;
- la instalación global debe quedar fuera de raíces de escritura del proyecto;
- los adaptadores no afirmarán aislamiento cuando solo están filtrando contexto;
- los recursos ejecutables tendrán políticas de host separadas.

## 25. Revisión escalonada por riesgo

### R0 — Declarativa

- sin scripts;
- sin shell;
- sin red;
- sin MCP;
- sin archivos binarios.

Revisión:

- análisis automático;
- lectura humana enfocada del contenido completo;
- un aprobador.

### R1 — Solo lectura

Puede leer archivos del proyecto, sin escritura ni ejecución.

Revisión:

- lectura humana completa;
- pruebas de activación;
- un aprobador y checklist.

### R2 — Escritura local

Puede modificar archivos del proyecto.

Revisión:

- lectura humana completa;
- ejecución en entorno desechable;
- pruebas de diffs esperados;
- dos aprobadores.

### R3 — Shell o red

Revisión:

- análisis de scripts con herramientas existentes;
- ejecución en runner desechable sin secretos;
- allowlist de red cuando aplique;
- dos aprobadores, uno de seguridad.

### R4 — Credenciales o acciones remotas

Revisión:

- threat model específico;
- prueba con credenciales sintéticas;
- permisos mínimos documentados;
- dos aprobadores de seguridad;
- no activación implícita.

### R5 — Privilegiada o destructiva

No entra en el catálogo estable V0. Requiere una RFC futura.

---

# Parte VI — Staging, CI y revisión

## 26. Repositorios y zonas físicas

### Repositorio principal

Contiene:

- código del resolver;
- especificaciones;
- manifiestos canónicos aprobados;
- entradas propias redistribuibles;
- evaluaciones;
- documentación.

### Repositorio o fork de staging

Contiene candidatos y reportes temporales.

Reglas:

- sin secretos;
- sin tokens de escritura al repositorio principal;
- sin `pull_request_target`;
- sin ejecutar workflows aportados por candidatos;
- acciones fijadas por commit;
- permisos `contents: read` por defecto;
- artefactos con expiración;
- aprobación manual antes de cualquier job con red;
- transferencia al principal solo de manifiestos y archivos explícitamente permitidos.

### Cuarentena local

Ignorada por Git y fuera del contexto de los agentes de desarrollo.

## 27. No construir sandbox propio

El proyecto no prometerá aislamiento resistente a escapes.

Para pruebas posteriores se usarán, según disponibilidad:

- contenedores rootless desechables;
- runners efímeros de CI;
- VM efímera para riesgos altos;
- filesystem temporal;
- secretos sintéticos;
- red deshabilitada por defecto;
- límites del proveedor o runtime.

La garantía base será:

> **Ningún candidato se ejecuta por defecto.**

## 28. No construir escáner propio

El pipeline integrará herramientas mantenidas externamente:

- Semgrep para reglas estáticas;
- escáneres de secretos;
- escáneres de dependencias;
- OpenSSF Scorecard como señal de proyecto;
- verificación de licencias;
- antivirus o análisis de binarios cuando aplique.

Las reglas específicas del proyecto podrán vivir en el repositorio, pero el motor no se reimplementará.

---

# Parte VII — Decisión espejo vs. índice

## 29. Decisión: índice primero, vendorización excepcional

El catálogo principal será un **índice confiable**, no un espejo universal.

Para recursos de terceros guardará:

- manifiesto canónico;
- URL lógica o identificador de fuente;
- commit o versión fijada;
- hash esperado;
- licencia detectada y revisada;
- instrucciones de instalación;
- patches o adaptadores propios cuando su licencia lo permita;
- reporte de revisión.

La instalación descargará la fuente fijada y verificará su hash.

## 30. Cuándo se permite almacenar una copia

Solo cuando ocurra al menos una de estas condiciones:

- el recurso es de autoría propia;
- la licencia permite claramente redistribución;
- existe obligación de disponibilidad offline aceptada por mantenimiento;
- se conserva atribución y NOTICE requeridos;
- la copia fue aprobada explícitamente como snapshot.

Las copias de terceros no serán el comportamiento por defecto.

## 31. Archivo o mirror opcional

Si en el futuro se desea preservar upstreams desaparecidos, se utilizará un repositorio o sistema de releases separado del código principal, con política legal específica.

## 32. Consecuencias

Ventajas:

- repo más pequeño;
- menor exposición legal;
- procedencia clara;
- actualización controlada;
- menos contenido ejecutable dentro del repositorio principal.

Costo:

- una instalación inicial puede requerir red;
- se necesita manejar desaparición de upstreams;
- la reproducibilidad depende de hashes y disponibilidad del origen o archivo autorizado.

---

# Parte VIII — Resolver determinista

## 33. Pipeline

```text
Tarea del usuario
    ↓
Normalización local
    ↓
Detección del proyecto
    ↓
Mapeo a capacidades V1
    ↓
Filtrado por superficie, runtime, riesgo y política
    ↓
Recuperación de candidatos por índice local
    ↓
Selección de cobertura mínima
    ↓
Reglas de conflicto y redundancia
    ↓
Explicación
    ↓
skills.lock
    ↓
Adaptador del host
```

## 34. Detección del proyecto

Se usarán señales deterministas:

- archivos de configuración;
- extensiones;
- dependencias;
- scripts de package manager;
- directorios;
- CI;
- infraestructura;
- imports o símbolos seleccionados;
- lenguaje predominante.

No se leerá todo el repositorio ni se enviará a un modelo por defecto.

## 35. Interpretación de la tarea

Orden de resolución:

1. reglas exactas;
2. alias de taxonomía;
3. BM25/FTS local;
4. patrones de verbos y objetos;
5. embeddings locales opcionales;
6. modelo pequeño opcional como fallback;
7. revisión interactiva cuando la confianza sea baja.

## 36. Selección

El objetivo no es elegir la entrada con más similitud, sino el conjunto mínimo que cubra capacidades requeridas bajo restricciones.

Costo conceptual:

```text
costo = cantidad_de_skills
      + redundancia
      + contexto_estimado
      + riesgo
      + incompatibilidad
      + necesidad_de_descarga
```

Restricciones duras:

- superficie compatible;
- nivel de riesgo permitido;
- fuente permitida;
- capabilities obligatorias;
- conflictos;
- versión de esquema;
- disponibilidad del adaptador.

## 37. Modo de baja confianza

Cuando la confianza sea insuficiente, el sistema no inventará certeza.

Respuesta esperada:

```text
No existe una resolución suficientemente confiable.
Capacidades probables:
- frontend.visual-design
- frontend.motion

Opciones:
1. Resolver solo visual-design.
2. Incluir motion.
3. No activar ninguna skill.
```

## 38. Límite de activación

Valor inicial recomendado:

```yaml
resolver:
  max_active_skills: 6
```

Una resolución que necesite superar el límite debe justificarlo o dividir la tarea.

---

# Parte IX — Casos dorados del resolver

## 39. Formato del dataset

Cada caso tendrá:

```yaml
id: GOLD-001
project_fixture: react-landing
user_task: "Haz la página responsive y mejora accesibilidad"
expected_capabilities:
  - frontend.responsive-layout
  - frontend.accessibility
forbidden_capabilities:
  - backend.authentication
expected_skill_profiles:
  - responsive-accessibility
max_selected_skills: 2
notes: "No activar visual testing salvo solicitud o señal adicional"
```

Antes de programar scoring, los 50 casos se congelarán y revisarán manualmente.

## 40. Lista inicial de 50 casos

### Frontend

1. **GOLD-001 — React landing responsive:** espera `frontend.responsive-layout` y `frontend.accessibility`; excluye backend.
2. **GOLD-002 — Mejorar estética de dashboard:** espera `frontend.visual-design`; no activar motion automáticamente.
3. **GOLD-003 — Animaciones de transición:** espera `frontend.motion`; visual-design es opcional.
4. **GOLD-004 — Reducir carga inicial Next.js:** espera `frontend.performance`; no activar refactoring general salvo evidencia.
5. **GOLD-005 — Construir componentes desde mockup:** espera `frontend.implementation` y `frontend.visual-design`.
6. **GOLD-006 — Corregir contraste y navegación por teclado:** espera `frontend.accessibility`.
7. **GOLD-007 — Screenshots inconsistentes entre browsers:** espera `testing.visual` y `quality.debugging`.
8. **GOLD-008 — UI desfasada después de cambios CSS:** espera `quality.debugging` y `frontend.visual-design`.
9. **GOLD-009 — Página estática sin framework:** espera capacidades frontend generales; no exigir React.
10. **GOLD-010 — Solo cambiar copy del hero:** no activar skill frontend pesada; posiblemente ninguna skill o prompt-pack de escritura.

### Backend y API

11. **GOLD-011 — Crear endpoint FastAPI:** espera `backend.api-implementation` y, si hay contrato, `architecture.api-design`.
12. **GOLD-012 — Diseñar API antes de implementar:** espera solo `architecture.api-design`.
13. **GOLD-013 — Autenticación con OAuth:** espera `backend.authentication` y `security.identity-access`.
14. **GOLD-014 — Roles y permisos:** espera `backend.authorization` y `security.identity-access`.
15. **GOLD-015 — Worker de tareas en segundo plano:** espera `backend.background-jobs`.
16. **GOLD-016 — Error intermitente en endpoint:** espera `quality.debugging`; backend API como contexto opcional.
17. **GOLD-017 — Integrar API externa:** espera `backend.api-implementation`; network-required solo si la skill lo necesita.
18. **GOLD-018 — Refactor de servicio sin cambio funcional:** espera `quality.refactoring`; no activar arquitectura completa.

### Base de datos

19. **GOLD-019 — Diseñar entidades nuevas:** espera `database.schema-design` y `architecture.data-modeling`.
20. **GOLD-020 — Crear migración Alembic:** espera `database.migrations`.
21. **GOLD-021 — Query PostgreSQL lenta:** espera `database.query-performance` y `quality.debugging`.
22. **GOLD-022 — Revisar migración peligrosa:** espera `database.migrations`, `quality.code-review` y señal de seguridad si afecta datos críticos.
23. **GOLD-023 — Cambiar ORM sin cambiar esquema:** espera `backend.data-access` y `quality.refactoring`.

### Pruebas y calidad

24. **GOLD-024 — Agregar pruebas unitarias:** espera `testing.unit`.
25. **GOLD-025 — Probar integración API-DB:** espera `testing.integration`.
26. **GOLD-026 — Flujo completo de registro:** espera `testing.end-to-end`; auth como contexto, no necesariamente skill activa.
27. **GOLD-027 — Revisar PR:** espera `quality.code-review`.
28. **GOLD-028 — Bug con stack trace:** espera `quality.debugging`.
29. **GOLD-029 — Limpiar duplicación:** espera `quality.refactoring`.
30. **GOLD-030 — “Mejora todo el proyecto”:** baja confianza; no activar muchas skills silenciosamente.

### Seguridad

31. **GOLD-031 — Threat model de una API:** espera `security.threat-modeling` y `architecture.api-design` como contexto opcional.
32. **GOLD-032 — Revisar manejo de secretos:** espera `security.secrets-handling`.
33. **GOLD-033 — Dependencias vulnerables:** espera `security.dependency-audit` y `security.supply-chain`.
34. **GOLD-034 — Hardening de login:** espera `security.identity-access`, `backend.authentication` y `security.secure-coding`.
35. **GOLD-035 — Revisar GitHub Actions:** espera `security.supply-chain` y `devops.ci-cd`.
36. **GOLD-036 — Skill externa con shell:** el resolver debe filtrar por riesgo; no activarla en política R1.
37. **GOLD-037 — MCP con acceso de escritura remoto:** requiere R4 y activación explícita; nunca implícita.
38. **GOLD-038 — Prompt-pack declarativo:** R0; no inferir permisos de shell.

### DevOps y operaciones

39. **GOLD-039 — Crear Dockerfile:** espera `devops.containers`.
40. **GOLD-040 — Pipeline CI de tests:** espera `devops.ci-cd` y capacidades de pruebas detectadas.
41. **GOLD-041 — Desplegar aplicación:** espera `devops.deployment`; cloud específico es señal, no capability.
42. **GOLD-042 — Añadir logs y métricas:** espera `devops.observability`.
43. **GOLD-043 — Error solo en contenedor:** espera `quality.debugging` y `devops.containers`.

### Documentación, workflow, IA y datos

44. **GOLD-044 — Documentar API:** espera `documentation.api-reference`.
45. **GOLD-045 — Crear manual técnico:** espera `documentation.technical-writing`.
46. **GOLD-046 — Preparar commits limpios:** espera `workflow.git`.
47. **GOLD-047 — Preparar release:** espera `workflow.release`.
48. **GOLD-048 — Diseñar una skill nueva:** espera `ai.prompt-design`; no activar investigador.
49. **GOLD-049 — Conectar agente a MCP:** espera `ai.mcp-integration`; requiere compatibilidad de host.
50. **GOLD-050 — Analizar CSV y generar gráfica:** espera `data.analysis` y `data.visualization`; no activar frontend.

## 41. Segunda capa de goldens

Cuando se seleccione el catálogo manual de 20 entradas, cada caso debe incluir IDs exactos esperados y alternativas aceptables.

Ejemplo:

```yaml
expected_skills:
  required:
    - catalog.responsive-accessibility
  allowed_alternatives:
    - catalog.frontend-a11y
  forbidden:
    - catalog.fullstack-architect
```

No comienza la implementación del scoring hasta que esta segunda capa exista.

---

# Parte X — Métricas y criterios de éxito

## 42. Métricas del mapeo de capacidades

- precision macro y micro;
- recall macro y micro;
- F1;
- tasa de `unmapped`;
- tasa de baja confianza correcta;
- confusión entre capacidades vecinas.

## 43. Métricas de selección de skills

- precision@k;
- recall de skills requeridas;
- exact set match;
- falsas activaciones;
- skills redundantes por caso;
- violaciones de restricciones;
- selecciones incompatibles con la superficie;
- selecciones bloqueadas correctamente por riesgo.

## 44. Métricas de eficiencia

- tiempo de resolución local;
- tamaño del índice;
- tokens estimados de metadata;
- tokens del conjunto activo;
- ahorro frente a exponer todo el catálogo;
- cantidad de archivos materializados;
- llamadas a modelos externos: objetivo cero por defecto.

## 45. Umbrales iniciales del MVP

Antes de declarar el corte vertical funcional:

```text
Capability micro-F1                  ≥ 0.90
Recall de skills requeridas          ≥ 0.90
Falsas activaciones por caso         ≤ 0.20
Violaciones de restricciones         = 0
Casos R4 activados implícitamente    = 0
Resolución local p95                 < 500 ms en catálogo de 1,000 manifests sintéticos
Reducción de metadata/contexto       ≥ 80% frente a catálogo completo
```

Los umbrales podrán cambiar mediante RFC, no para hacer pasar una implementación deficiente.

---

# Parte XI — Corte vertical funcional

## 46. Objetivo

Demostrar de extremo a extremo:

```text
20 manifests manuales
    ↓
search por categorías y superficies
    ↓
scan de proyecto
    ↓
resolve de una tarea
    ↓
explain
    ↓
skills.lock
    ↓
activación en Claude Code
    ↓
prueba de que solo las skills elegidas quedan expuestas
```

## 47. CLI mínima

```bash
skills init
skills catalog list
skills search
skills scan
skills resolve
skills explain
skills activate --target claude-code
skills deactivate
skills verify
```

No se implementarán dieciséis comandos antes de validar estos.

## 48. Adaptador Claude Code V0

El adaptador aprovechará capacidades nativas en lugar de recrearlas:

- skills de proyecto y usuario;
- carga progresiva;
- invocación explícita o automática;
- configuración de visibilidad;
- symlinks cuando sean compatibles;
- plugins existentes cuando corresponda.

Responsabilidades propias:

- materializar solo el conjunto resuelto;
- evitar que el catálogo completo quede dentro del workspace;
- generar configuración de mínimo privilegio;
- registrar la versión del host;
- advertir sobre extensiones no portables;
- verificar hashes antes de enlazar;
- no afirmar que el filtro de skills es un sandbox.

## 49. Codex en V0

Codex estará presente en el esquema de compatibilidad y en casos dorados, pero su adaptador completo se pospone.

Razones:

- validar primero el modelo de resolver;
- evitar mantener dos integraciones mientras cambia el contrato;
- sus mecanismos nativos ya ofrecen carga progresiva y scopes;
- el adaptador deberá detectar la superficie y versión real en lugar de asumir una ruta fija universal.

## 50. Catálogo manual de 20 entradas

La selección inicial debe cubrir:

- 5 frontend;
- 3 backend/API;
- 2 base de datos;
- 3 testing/calidad;
- 3 seguridad;
- 2 DevOps;
- 1 documentación/workflow;
- 1 IA/MCP.

Preferencias:

- mayoría R0–R1;
- máximo cuatro R2;
- ninguna R4 activable implícitamente;
- licencias claras;
- diversidad de autores;
- mezcla de estándar portable y extensiones específicas;
- al menos dos entradas incompatibles con Claude Code para probar filtros;
- al menos dos exclusivas de Claude Code;
- al menos dos compatibles con Codex;
- al menos dos herramientas `standalone-cli` catalogadas pero no activables como skill.

---

# Parte XII — Investigación futura

## 51. El investigador no forma parte del MVP

Solo comienza cuando:

- la taxonomía V1 esté estable;
- el manifiesto canónico haya sido usado con 20 entradas;
- los 50 goldens pasen umbrales;
- el adaptador Claude Code funcione;
- se conozcan los campos realmente necesarios;
- exista política legal de fuentes;
- exista staging separado.

## 52. Qué podrá hacer después

- descubrir repositorios y referencias comunitarias;
- recopilar señales de GitHub y Reddit;
- descargar en cuarentena;
- detectar duplicados;
- producir evidencia;
- sugerir un manifiesto provisional;
- generar un paquete para revisión humana.

Nunca podrá:

- escribir el manifiesto canónico final;
- aprobar una entrada;
- activar una skill;
- ejecutar candidatos por defecto;
- escribir directamente en el repositorio principal;
- modificar políticas de admisión.

---

# Parte XIII — Gobernanza de modelos y seguridad

## 53. Roles permanentes

El repositorio define roles, no marcas de modelo:

```text
security-implementer
security-reviewer-independent
architecture-reviewer
implementation-agent
human-approver
```

Reglas:

- el revisor de seguridad debe ser distinto del implementador;
- los modelos producen hallazgos, no garantías;
- ninguna revisión de modelo reemplaza pruebas, políticas o revisión humana;
- los resultados se guardan como evidencia versionada;
- un hallazgo crítico bloquea el avance hasta decisión humana.

## 54. Configuración concreta de este proyecto

Para este proyecto específico, el futuro paquete de implementación fijará en configuración:

```yaml
orchestration:
  security_implementer:
    model: fable-5
    required_for_all_security_waves: true

  security_plan_reviewers:
    - model: fable-5
    - model: opus-5

  independent_security_reviewer:
    model: opus-5
    must_differ_from_implementer: true
```

Esta selección no se codifica dentro del esquema público del catálogo. Se guarda en la carpeta de orquestación del plan de implementación para poder cambiarla sin alterar el producto.

## 55. Prompts futuros obligatorios

El `.zip` de implementación incluirá, como mínimo:

1. prompt de revisión arquitectónica del plan;
2. prompt de threat-model review para Fable 5;
3. prompt de revisión independiente para Opus 5;
4. prompt de implementación de cada WAVE de seguridad, exclusivamente para Fable 5;
5. prompt de comparación entre hallazgos de ambos modelos;
6. prompt de cierre humano con checklist y riesgos residuales;
7. prompt de escalamiento para generar diagnóstico completo cuando un modelo quede realmente bloqueado.

Todavía no se redactan los prompts ejecutables en esta etapa.

---

# Parte XIV — Secuencia previa a las WAVEs

## 56. PRE-0 — Decisiones cerradas por este documento

- resolver primero;
- investigador después;
- taxonomía gobernada;
- manifiesto canónico del catálogo;
- threat model centrado en activación;
- revisión escalonada;
- staging separado;
- índice primero;
- tres tipos;
- un adaptador inicial;
- sin sandbox ni scanner propios;
- goldens antes de scoring;
- roles de modelo abstractos con configuración concreta externa.

## 57. PRE-1 — Especificación de taxonomía

Entregables posteriores a aprobación:

```text
spec/capabilities/v1/capabilities.yaml
spec/capabilities/v1/aliases.yaml
spec/capabilities/v1/deprecations.yaml
spec/capabilities/v1/README.md
```

Gate:

- 44 capacidades definidas;
- ejemplos positivos y negativos;
- proceso de RFC;
- validación de esquema.

## 58. PRE-2 — Threat model formal

Entregables:

```text
docs/security/THREAT_MODEL.md
docs/security/TRUST_BOUNDARIES.md
docs/security/RISK_TIERS.md
docs/security/SECURITY_ASSUMPTIONS.md
```

Gate:

- revisión Fable 5;
- revisión Opus 5;
- resolución humana de desacuerdos;
- riesgos residuales explícitos.

## 59. PRE-3 — Dataset dorado

Entregables:

```text
evaluations/goldens/capability-cases.yaml
evaluations/goldens/selection-cases.yaml
evaluations/fixtures/
evaluations/README.md
```

Gate:

- 50 casos;
- IDs esperados después de elegir las 20 entradas;
- revisión humana;
- casos negativos y de baja confianza.

## 60. PRE-4 — Catálogo semilla

Entregables:

- 20 manifiestos canónicos;
- reportes de licencia;
- compatibilidad por superficie;
- niveles de riesgo;
- hashes y procedencia;
- contenido vendorizado solo cuando esté permitido.

Gate:

- ninguna entrada depende de metadata del autor como fuente de verdad;
- todas las entradas tienen revisión;
- diversidad suficiente para probar filtros.

## 61. PRE-5 — Especificación del corte vertical

Entregables:

- contratos CLI;
- esquema de `skills.lock`;
- arquitectura del índice local;
- reglas de materialización Claude Code;
- criterios de rendimiento;
- estrategia de rollback.

Solo después de PRE-1 a PRE-5 se genera el plan completo por WAVEs.

---

# Parte XV — Decisiones pendientes reales

## 62. Nombre del proyecto

No bloquea arquitectura.

## 63. Lenguaje de implementación

Recomendación para evaluar en el plan por WAVEs:

- TypeScript si se prioriza ecosistema CLI, distribución npm y facilidad de contribución;
- Rust si el rendimiento, binario único y aislamiento de dependencias son prioritarios;
- Python solo si el investigador y análisis dominan desde el inicio, lo cual ya no ocurre en el MVP.

La decisión debe tomarse con un pequeño ADR antes de implementar.

## 64. Firma y transparencia

Debe decidirse si V0 usa:

- commits firmados;
- firmas Sigstore;
- log de transparencia;
- o una combinación progresiva.

No bloquea el prototipo local, pero sí una distribución pública confiable.

## 65. Fuente oficial y catálogos privados

Debe definirse si la CLI admite desde V0:

- un solo índice local;
- varios índices con prioridad;
- catálogo oficial + privado;
- overlays por proyecto.

Recomendación: soportar múltiples fuentes en el esquema, pero implementar una local y una oficial en el corte vertical.

---

# Parte XVI — Criterios de aprobación de este documento

El documento puede aprobarse para convertirlo en paquete de implementación si se acepta que:

1. El investigador queda fuera del MVP.
2. La taxonomía V1 es WAVE/PRE-0 real y gobernada.
3. El manifiesto canónico lo controla el catálogo.
4. El riesgo principal es la inyección y ejecución durante activación.
5. Los candidatos usan staging separado.
6. La revisión humana se escala según riesgo.
7. El catálogo es índice primero.
8. Se empiezan solo tres tipos.
9. No se construye sandbox ni motor de análisis estático propios.
10. El resolver se evalúa con 50 goldens antes de ajustar scoring.
11. Claude Code es el primer adaptador.
12. Codex y otras superficies se modelan desde el esquema, pero se implementan después.
13. La búsqueda filtra por categoría, host, runtime, portabilidad, riesgo, scope y fuente.
14. Las WAVEs de seguridad serán implementadas obligatoriamente por Fable 5 en la configuración de este proyecto.
15. Opus 5 realizará revisión independiente y ninguna revisión de modelo se presentará como garantía de seguridad.

---

# Parte XVII — Entregable posterior a aprobación

Después de aprobar esta V2 se generará un `.zip` estructurado, no una única lista de tareas.

Estructura propuesta:

```text
skills-ecosystem-implementation-plan/
├── 00-master-context/
├── 01-architecture-decisions/
├── 02-capability-taxonomy/
├── 03-security/
│   ├── threat-model/
│   ├── review-prompts/
│   └── security-wave-prompts/
├── 04-evaluations/
├── 05-seed-catalog/
├── 06-vertical-slice/
├── 07-waves/
├── 08-review-checklists/
├── 09-model-orchestration/
├── 10-diagnostics-escalation/
└── README.md
```

Cada WAVE contendrá:

- objetivo;
- contexto;
- alcance;
- fuera de alcance;
- archivos esperados;
- contratos;
- criterios de aceptación;
- pruebas;
- riesgos;
- modelo/rol recomendado;
- prompt de implementación;
- prompt de revisión;
- regla de escalamiento;
- handoff esperado.

---

## Conclusión

El sistema útil no empieza rastreando Internet. Empieza demostrando que puede describir, filtrar y activar un conjunto pequeño de capacidades de forma correcta, segura, explicable y reproducible.

La arquitectura revisada prioriza ese núcleo:

```text
Taxonomía gobernada
      +
Manifiestos canónicos confiables
      +
Catálogo índice-first
      +
Resolver medido con goldens
      +
Un adaptador real
      =
Corte vertical útil
```

Solo cuando ese corte produzca datos reales deberá comenzar el investigador, que es el componente con mayor costo, superficie de ataque y carga legal.
