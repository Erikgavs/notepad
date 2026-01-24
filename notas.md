# NOTAS DE RUST - SIGNALS Y EXPRESIONES

---

## SIGNALS (`use_signal`)

Un signal es como una **"caja" reactiva** que guarda un valor.
Cuando el contenido cambia, Freya re-renderiza los componentes que lo usan.

```rust
let mut new_title = use_signal(String::new);  // crea signal con String vacio
```

---

## LEER Y ESCRIBIR EN SIGNALS

| Metodo | Que hace |
|--------|----------|
| `.read()` | Lee el valor (devuelve referencia `&String`) |
| `.clone()` | Crea una copia propia del valor |
| `.set()` | Escribe/guarda un nuevo valor |

```rust
value: new_title.read().clone()  // LEE del signal para mostrar
new_title.set(title)             // ESCRIBE al signal para guardar
```

---

## POR QUE `.clone()`?

- `.read()` devuelve una **referencia temporal** (`&String`) - como mirar un libro sin llevartelo
- `.clone()` hace una **copia independiente** - como hacer una fotocopia para llevartela

> El Input necesita ser dueno del String, no puede trabajar con una referencia temporal.

---

## SENTENCIAS VS EXPRESIONES

| Codigo | Que pasa? |
|--------|-----------|
| `Note { ... }` | Se devuelve (retorna) |
| `Note { ... };` | Se ejecuta y se pierde |
| `let x = Note { ... };` | Se ejecuta y se guarda en x |

- Con `;` → Se ejecuta internamente, el valor se descarta
- Sin `;` → El valor se **devuelve/retorna**

> Si quieres conservar un valor, guardalo en variable con `let`.

---

## STRUCT NOTE - SINTAXIS

Los campos se separan con **COMAS**, no punto y coma:

```rust
Note {
    title: valor1,     // coma
    content: valor2,   // coma
}
```

El `;` va **DESPUES** de cerrar la struct si es una sentencia:

```rust
let nota = Note {
    title: new_title.read().clone(),
    content: new_content.read().clone(),
};  // <- el ; va aqui
```

---

## CICLO DE UN INPUT

```
Usuario escribe "Hola"
        |
        v
onchange recibe "Hola"
        |
        v
new_title.set("Hola")  ->  El signal ahora contiene "Hola"
        |
        v
Freya detecta el cambio y re-renderiza
        |
        v
value: new_title.read().clone()  ->  Lee "Hola" y lo muestra en el Input
```

---

## VARIABLES Y SCOPE

Una variable solo vive dentro de su bloque `{ }`.
Cuando el bloque termina, la variable **muere** y se pierde.

```rust
onpress: move |_| {
    let nota = Note { ... };
    // nota vive aqui

    // si no haces push, cuando el closure termina, nota se pierde
    notes.write().push(nota);  // <- esto mueve nota al vector, sobrevive
}
```

---

## MODIFICAR UN VECTOR EN UN SIGNAL

| Metodo | Uso |
|--------|-----|
| `.read()` | Para leer (acceso inmutable) |
| `.write()` | Para modificar (acceso mutable) |

```rust
notes.read().iter()      // leer las notas
notes.write().push(nota) // anadir una nota
```

---

## CLOSURES - CODIGO DENTRO DE `{ }`

Un closure es una **funcion anonima**. Todo el codigo debe ir **DENTRO** de las llaves.

**CORRECTO:**
```rust
onpress: move |_| {
    let nota = Note { ... };
    notes.write().push(nota);   // <- DENTRO del closure
}
```

**INCORRECTO:**
```rust
onpress: move |_| {
    let nota = Note { ... };
}
notes.write().push(nota);       // <- FUERA del closure, ERROR!
```

Si el codigo esta fuera del closure:
- No se ejecuta cuando presionas el boton
- La variable `nota` no existe fuera (ya murio)

---

## FLUJO COMPLETO DEL BOTON GUARDAR

```rust
Button {
    onpress: move |_| {
        // 1. Crear la Note con los valores de los inputs
        let nota = Note {
            title: new_title.read().clone(),
            content: new_content.read().clone(),
        };

        // 2. Anadir la nota al vector
        notes.write().push(nota);

        // 3. Freya detecta el cambio en 'notes' y re-renderiza
        // 4. El for loop en el UI muestra la nueva nota
    }
}
```

**Orden de ejecucion:**
1. Usuario presiona el boton
2. Se dispara el closure de `onpress`
3. Se crea `nota` con los datos de los inputs
4. Se hace push al vector `notes`
5. El signal `notes` cambia → Freya re-renderiza
6. El for loop ahora incluye la nueva nota
7. La nota aparece en pantalla

---

## PERSISTENCIA CON JSON (serde)

Las notas en memoria **se pierden al cerrar la app**.
Para guardarlas en disco usamos JSON con las crates `serde` y `serde_json`.

**Dependencias en Cargo.toml:**
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## DERIVES PARA JSON

```rust
#[derive(Clone, Serialize, Deserialize)]
struct Note { ... }
```

| Derive | Que permite |
|--------|-------------|
| `Serialize` | Convertir Note → JSON (guardar) |
| `Deserialize` | Convertir JSON → Note (cargar) |

---

## CARGAR NOTAS (`JSON -> Vec<Note>`)

```rust
fn load_notes() -> Vec<Note> {
    match fs::read_to_string("notas.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}
```

- `match` evalua si el archivo existe o no
- `Ok(content)` → archivo existe, convertir JSON a vector
- `Err(_)` → archivo no existe, devolver vector vacio
- `unwrap_or_else` → si el JSON esta corrupto, devolver vector vacio

---

## GUARDAR NOTAS (`Vec<Note> -> JSON`)

```rust
fn save_notes(notes: &Vec<Note>) {
    if let Ok(json) = serde_json::to_string_pretty(notes) {
        let _ = fs::write("notas.json", json);
    }
}
```

- `to_string_pretty` → convierte vector a JSON legible
- `fs::write` → escribe el JSON al archivo

---

## CONVERSION JSON ↔ STRUCT

```
JSON:                              Rust:
{                                  Note {
  "title": "Hola",        <->        title: "Hola".to_string(),
  "content": "Mundo"                 content: "Mundo".to_string(),
}                                  }
```

> **Serde mapea automaticamente los campos por nombre.**

---

---

# LOAD_NOTES() - DESGLOSE PASO A PASO

---

## LA FIRMA

```rust
fn load_notes() -> Vec<Note> {
```

- **No recibe nada**
- **Devuelve** un `Vec<Note>` (un vector de notas)

---

## DENTRO: LEER EL ARCHIVO

`fs::read_to_string(NOTES_FILE)` intenta leer el archivo.
Devuelve un **Result**, que puede ser `Ok` o `Err`.

Usas `match` para manejar ambos casos:

```rust
match fs::read_to_string(NOTES_FILE) {
    Ok(content) => { ... },  // archivo leido correctamente, content es String
    Err(_) => { ... },       // archivo no existe o no se pudo leer
}
```

---

## CASO `Err`: EL ARCHIVO NO EXISTE

Primera vez que abres la app, no hay archivo. Devuelves vector vacio:

```rust
Err(_) => vec![]
```

> Asi la app arranca sin notas, sin crashear.

---

## CASO `Ok`: CONVERTIR JSON A `Vec<Note>`

Aqui `content` es un String con algo como:
```json
[{"title": "Hola", "content": "Mundo"}]
```

Para convertirlo a `Vec<Note>` usas:

```rust
serde_json::from_str(&content)
```

- `from_str` = "desde string"
- `&content` = referencia al string JSON
- Serde mira los campos del JSON y los mapea al struct `Note`

Esto **TAMBIEN** devuelve un Result (puede fallar si el JSON esta mal).
Para manejarlo simple:

```rust
serde_json::from_str(&content).unwrap_or_else(|_| vec![])
```

- `unwrap_or_else` = "si funciona dame el valor, si no, ejecuta esto otro"
- `|_| vec![]` = "devuelve un vector vacio"

---

## RETORNO SIN `;`

En Rust, la ultima expresion **SIN punto y coma** es el valor de retorno.
El `match` entero (sin `;` al final) es lo que retorna la funcion.

```rust
fn load_notes() -> Vec<Note> {
    match ... {         // <-- esto es lo que se retorna
        Ok(...) => ...,
        Err(_) => ...,
    }                   // <-- SIN ; = se devuelve
}
```

---

## FLUJO COMPLETO

```
App se abre
    |
    v
load_notes() se ejecuta
    |
    v
fs::read_to_string("notas.json")
    |
    +--> Err: no hay archivo -> vec![] (sin notas)
    |
    +--> Ok(content): hay archivo
              |
              v
         serde_json::from_str(&content)
              |
              +--> Ok: devuelve Vec<Note> con las notas
              |
              +--> Err: JSON corrupto -> vec![] (sin notas)
```

---

---

# LOAD_NOTES() - CADA ELEMENTO EXPLICADO

```rust
fn load_notes() -> Vec<Note> {
    match fs::read_to_string("notas.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}
```

---

## FIRMA DE LA FUNCION

| Elemento | Significado |
|----------|-------------|
| `fn` | Keyword para declarar una funcion |
| `load_notes` | Nombre de la funcion (tu lo eliges) |
| `()` | No recibe ningun parametro |
| `->` | Indica que tipo de dato devuelve |
| `Vec<Note>` | Devuelve un vector (lista dinamica) de Notes |
| `{` ... `}` | Cuerpo de la funcion |

---

## `match`

Es como un "switch". Evalua una expresion y ejecuta codigo diferente segun el resultado.

---

## `fs::read_to_string("notas.json")`

| Elemento | Significado |
|----------|-------------|
| `fs` | Modulo de filesystem (sistema de archivos) de Rust |
| `::` | Accedes a algo dentro de ese modulo |
| `read_to_string` | Funcion que lee un archivo entero y lo devuelve como String |
| `"notas.json"` | La ruta del archivo que quieres leer |

> Devuelve un `Result<String, Error>`: puede ser `Ok(String)` si leyo bien, o `Err(Error)` si fallo.

---

## `Ok(content) =>`

| Elemento | Significado |
|----------|-------------|
| `Ok` | El caso exitoso del Result |
| `content` | Variable que tu nombras, contiene el String con el texto del archivo |
| `=>` | "entonces haz esto" |

---

## `serde_json::from_str(&content)`

| Elemento | Significado |
|----------|-------------|
| `serde_json` | La crate de JSON |
| `from_str` | Funcion que convierte un `&str` (texto) a un tipo de Rust |
| `&content` | Referencia al string (le prestas el valor, no se lo das) |

> Rust infiere que quieres un `Vec<Note>` porque eso es lo que devuelve la funcion.

---

## `.unwrap_or_else(|_| vec![])`

| Elemento | Significado |
|----------|-------------|
| `unwrap_or_else()` | "si el Result es Ok dame el valor, si es Err ejecuta esta alternativa" |
| `|_|` | Closure (funcion anonima), el `_` ignora el error |
| `vec![]` | Macro que crea un vector vacio |

---

## `Err(_) =>`

| Elemento | Significado |
|----------|-------------|
| `Err` | El caso de fallo del Result |
| `_` | Ignoras el error (no te importa cual fue) |
| `=>` | "entonces haz esto" |
| `vec![]` | Devuelves un vector vacio. La app arranca sin notas |

---

## ¿Por que NO hay `;` al final del match?

Porque el `match` entero **es el valor de retorno** de la funcion. Sin `;` = "devuelve esto".

## ¿Por que NO hay `return`?

En Rust, la ultima expresion sin `;` se devuelve automaticamente. Es equivalente a `return match ... { };` pero mas idiomatico.

---

## QUE HACE LA LINEA DE `Ok` REALMENTE

No mete el texto directamente en el vector. Lo **transforma** en 3 pasos:

1. `fs::read_to_string` → lee el archivo y te da el texto crudo (un String con JSON)
2. `serde_json::from_str` → **interpreta** ese texto JSON y lo convierte en structs `Note`
3. Esos structs se meten en un `Vec<Note>`

```
Archivo en disco:                   En memoria (Rust):
[{"title": "Hola",          →      vec![Note {
  "content": "Mundo"}]                  title: "Hola".to_string(),
                                        content: "Mundo".to_string(),
                                    }]
```

> Serde hace esa conversion automaticamente porque tu struct tiene `Deserialize` y los nombres de los campos coinciden con las keys del JSON.

**Resumen simple:** Serde toma el JSON y lo **adapta a tu struct** — mapea cada campo del JSON al campo correspondiente de `Note` con el tipo correcto de Rust (`String`, `u32`, `bool`, etc.).

---

## POR QUE `&` EN LOS PARAMETROS (REFERENCIAS)

Cuando una funcion recibe un parametro, puede recibirlo de dos formas:

**Sin `&` (ownership/adueñarse):**
```rust
fn save_notes(notes: Vec<Note>) { ... }
```
La funcion se **adueña** del vector. Despues de llamarla, no podrias usar `notes` en el resto del codigo porque se "movio" dentro de la funcion.

**Con `&` (referencia/prestamo):**
```rust
fn save_notes(notes: &Vec<Note>) { ... }
```
La funcion solo lo **toma prestado** para leerlo. Cuando termina, el vector sigue siendo tuyo.

### Analogia

| | Sin `&` | Con `&` |
|---|---------|---------|
| Que pasa | Le das tu cuaderno y no te lo devuelve | Le dejas ver tu cuaderno pero tu lo sigues teniendo |
| Despues de llamar la funcion | No puedes usar el vector | Sigues usando el vector normal |

### Como se llama

```rust
save_notes(&notes.read());
//         ^ el & pasa una referencia (prestamo)
```

---

## POR QUE `load_notes()` NO TIENE PARAMETROS Y `save_notes()` SI

| Funcion | Recibe | Devuelve | Por que |
|---------|--------|----------|---------|
| `load_notes()` | Nada | `Vec<Note>` | Lee del archivo y te da los datos |
| `save_notes(notes)` | `&Vec<Note>` | Nada | Tu le das los datos y ella los guarda |

**`load_notes()`** — no necesita que le des nada. Ella sola sabe de donde sacar los datos (lee el archivo). Su trabajo es **producir** datos y devolvertelos.

```
load_notes():  archivo → te devuelve datos
```

**`save_notes(notes)`** — necesita que le **pases** los datos que quieres guardar. Ella no sabe que notas tienes, tu se las das. No devuelve nada porque su trabajo es solo escribir al archivo.

```
save_notes():  tu le das datos → archivo
```

> Son inversas: una produce datos, la otra los consume.

---

## QUE ES `let _ =`

`fs::write()` devuelve una respuesta: "lo hice bien" o "fallo".

Rust te **obliga** a hacer algo con esa respuesta. No te deja ignorarla sin mas.

```rust
fs::write(NOTES_FILE, json);        // WARNING: "hay un Result que no estas manejando"
let _ = fs::write(NOTES_FILE, json); // OK: "recibido, no me importa"
```

`let _ =` es como decir: **"ok, recibido, no me importa si fallo o no."**

> `_` no es una variable real. Nadie puede usarla despues. Es solo la forma de decir "lo ignoro conscientemente."

---

## COMO SABE `from_str` QUE STRUCT USAR

Rust lo vincula por el **tipo de retorno** de la funcion:

```rust
fn load_notes() -> Vec<Note> {
```

Cuando Rust ve que `from_str` devuelve un valor que la funcion retorna, piensa:
"la funcion dice que devuelve `Vec<Note>`, entonces `from_str` debe producir un `Vec<Note>`."

Rust **infiere** el tipo automaticamente. No necesitas escribirlo, pero si quisieras ser explicito:

```rust
let notas: Vec<Note> = serde_json::from_str(&content);
//          ^^^^^^^^^ esto le dice a serde "convierte el JSON a esto"
```

> **`from_str` sabe que debe crear `Vec<Note>` porque la funcion dice que devuelve `Vec<Note>`. Rust conecta los puntos automaticamente.**

---

## Integracion de load_notes() con use_signal

El vector sigue siendo la estructura principal en memoria. `load_notes()` simplemente devuelve un `Vec<Note>`, igual que cuando lo creabas a mano con datos hardcodeados.

```rust
// Antes: vector con datos hardcodeados
let mut notes = use_signal(|| {
    vec![Note {
        title: "Nota de ejemplo".to_string(),
        content: "Contenido de ejemplo!".to_string(),
    }]
});

// Ahora: vector con datos del JSON
let mut notes = use_signal(|| load_notes());
```

El resultado es el mismo tipo (`Vec<Note>`), solo cambia el origen de los datos. Si el archivo no existe, `load_notes()` devuelve un `vec![]` vacio.

## Persistencia: como se conecta el vector con el JSON

El JSON solo interviene en **dos momentos**:

1. **Al abrir la app** — `load_notes()` lee el archivo y llena el vector.
2. **Al añadir una nota** — `save_notes()` escribe el vector completo al archivo.

```rust
notes.write().push(nota);       // añade la nota al vector en memoria
save_notes(&notes.read());      // guarda TODO el vector al archivo JSON
```

Durante la ejecucion, todo funciona con el vector en memoria (lees con `.read()`, modificas con `.write().push()`). El JSON es solo el mecanismo de **persistencia entre sesiones** — para que las notas no se pierdan al cerrar la app.
