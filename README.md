# Reservas on-chain (Solana · Anchor · Devnet)

**Ruta elegida:** Proyecto propio
**Autor:** _TU NOMBRE COMPLETO_ · _tu-correo-de-registro@ejemplo.com_

## Propósito
Sistema de reservas para negocios pequeños (barbería, consultorio, clases). El dueño registra su negocio y sus servicios con un cupo; los clientes reservan desde su wallet y el dueño confirma, finaliza o cierra servicios. Todas las reglas viven en el programa.

## Despliegue
- **Red:** Devnet
- **Program ID:** `PEGA_AQUI_TU_PROGRAM_ID`
- **Transacción verificable:** https://explorer.solana.com/tx/PEGA_AQUI_UNA_FIRMA?cluster=devnet

## Arquitectura de cuentas (PDAs relacionadas)
| Cuenta | Seeds | Relación |
|---|---|---|
| `Negocio` | `["negocio", owner]` | Una por wallet dueña |
| `Servicio` | `["servicio", negocio, id_u64_le]` | Pertenece a un `Negocio` (`has_one = negocio`) |
| `Reserva` | `["reserva", servicio, cliente]` | Pertenece a un `Servicio` y a un cliente |

## Operaciones
| Instrucción | Tipo | Permiso | Regla |
|---|---|---|---|
| `crear_negocio` | Creación | Cualquier wallet | Nombre ≤ 32 |
| `crear_servicio` | Creación | Solo dueño (`has_one = owner`) | Cupo > 0 |
| `editar_servicio` | Modificación | Solo dueño | Cupo ≥ reservas activas |
| `reservar` | Creación | Cliente firmante | Debe haber cupo |
| `confirmar_reserva` | Modificación | Solo dueño | Debe estar Pendiente |
| `cancelar_reserva` | **Baja condicionada** | Solo el cliente que reservó | Solo si está Pendiente |
| `finalizar_reserva` | **Baja condicionada** | Solo dueño | Solo si está Confirmada |
| `cerrar_servicio` | **Cierre condicionado** | Solo dueño | Solo si `activas == 0` |

La lectura se hace con `program.account.<cuenta>.fetch()` / `.all()` en el cliente y el frontend.

## Correspondencia con "Pruebas y evidencias" del proyecto guiado
| Guiado | Este proyecto |
|---|---|
| Registro | `Negocio` (+ `Servicio`) |
| Cliente | `Reserva` (cuenta ligada al cliente y al servicio) |
| Venta | `confirmar_reserva` / `finalizar_reserva` |
| Baja/cierre condicionado | `cancelar_reserva`, `cerrar_servicio` (rechaza con `ReservasActivas`) |

## Estructura
```
programs/reservas/src/lib.rs   # Programa Anchor
client/client.ts               # Cliente (Solana Playground) coherente con el IDL
app/index.html                 # Frontend con wallet (Phantom)
app/idl.json                   # IDL exportado de la versión desplegada
```

## Configuración y ejecución
1. Abre https://beta.solpg.io → crea proyecto **Anchor (Rust)**.
2. Pega `programs/reservas/src/lib.rs` en `src/lib.rs`.
3. Conecta la wallet de Playground (abajo a la izquierda), red **Devnet**, y pide SOL: `solana airdrop 2` en la terminal.
4. **Build** → **Deploy**. Copia el Program ID.
5. Pega `client/client.ts` en `client/client.ts` de Playground y pulsa **Run**: imprime links de cada transacción.
6. Exporta el IDL (menú Build → Export IDL) y guárdalo como `app/idl.json`.

### Frontend
1. En `app/index.html` reemplaza `PEGA_AQUI_TU_PROGRAM_ID`.
2. Sirve la carpeta: `npx serve app` (o activa GitHub Pages sobre `/app`).
3. Abre la página con Phantom en **Devnet**, conecta y prueba: crear negocio, crear servicio, reservar, cancelar, cerrar servicio.

## Cómo probarlo
- Ejecuta `client.ts`: crea, lee, modifica, reserva, demuestra que **cerrar servicio con reservas activas es rechazado**, confirma, finaliza y finalmente cierra el servicio.
- Desde el frontend, intenta "Cerrar servicio" con una reserva pendiente → error `ReservasActivas`.
- Desde otra wallet, intenta cerrar el servicio de otro → error `NoAutorizado`.

## Aporte propio
- Diseño del caso de uso de reservas y del modelo de 3 cuentas relacionadas por PDAs.
- Contador en `Negocio` para derivar PDAs únicas de `Servicio`.
- Reglas de negocio: control de cupo, máquina de estados Pendiente → Confirmada, cierres condicionados y devolución de renta al cliente.
- Cliente de pruebas end-to-end y frontend con wallet.

## Evidencias
_Agrega aquí capturas o links de explorer de las transacciones._
