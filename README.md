# Reservas on-chain (Solana · Anchor · Devnet)

**Ruta elegida:** Proyecto propio
**Autor:** Moreno Hernandez Diego Alberto · L24090243@huixquilucan.tecnm.mx

## Propósito
Sistema de reservas para negocios pequeños (barbería, consultorio, clases). El dueño registra su negocio y sus servicios con un cupo; los clientes reservan desde su wallet y el dueño confirma, finaliza o cierra servicios. Todas las reglas viven en el programa.

## Despliegue
- **Red:** Devnet
- **Program ID:** `97V2zGHbu5Rh4N4MVjVdwqgAX3cvHsZDMRecePfe1zht`
- **Transacción verificable:** https://explorer.solana.com/tx/3sk9z7SNBYXUJHHLWaZ6QnmMBiA4y7NV4jXvsLaLWEFioiMUxr3h1rSSKp69sconkD3WEdV3KfrKUgpSprfbgx86?cluster=devnet

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
lib.rs       # Programa Anchor
client.ts    # Cliente (Solana Playground) coherente con el IDL
index.html   # Frontend con wallet (Phantom)
```

## Configuración y ejecución
1. Abre https://beta.solpg.io → crea proyecto **Anchor (Rust)**.
2. Pega el contenido de `lib.rs` en `src/lib.rs` de Playground.
3. Conecta la wallet de Playground (abajo a la izquierda), red **Devnet**, y obtén SOL de prueba en https://faucet.solana.com.
4. **Build** → **Deploy**. Copia el Program ID.
5. Pega `client.ts` en `client/client.ts` de Playground y pulsa **Run**: imprime links de cada transacción.
6. Exporta el IDL (menú Build → Export IDL) y guárdalo como `idl.json` junto a `index.html`.

### Frontend
1. En `index.html` reemplaza `PEGA_AQUI_TU_PROGRAM_ID` por el Program ID.
2. Sirve la carpeta: `npx serve .` (o activa GitHub Pages en el repo).
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
- Crear negocio: https://explorer.solana.com/tx/3sk9z7SNBYXUJHHLWaZ6QnmMBiA4y7NV4jXvsLaLWEFioiMUxr3h1rSSKp69sconkD3WEdV3KfrKUgpSprfbgx86?cluster=devnet
- Crear servicio: https://explorer.solana.com/tx/yoiz1To94uaADTnqUvZcnTsqRMoerqVznNPuv8AkEU61YcjrAQ2CQB5T1obxAq2SakDmFQ5yHxFL5Dtr2zgp5BK?cluster=devnet

_(Captura de la ejecución del cliente:)_
<img width="1916" height="1078" alt="image" src="https://github.com/user-attachments/assets/53ad255b-c110-4cfd-adb1-e4ee607be742" />


