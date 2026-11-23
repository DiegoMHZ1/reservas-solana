// Cliente para Solana Playground (pestaña "client.ts" → botón Run).
// Usa el IDL generado por el programa (pg.program) y ejecuta el flujo completo,
// imprimiendo cada transacción con su link al explorer de Devnet.

const programId = pg.program.programId;
const owner = pg.wallet.publicKey;
const link = (sig: string) =>
  `https://explorer.solana.com/tx/${sig}?cluster=devnet`;

const [negocioPda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("negocio"), owner.toBuffer()],
  programId
);

// 1) Crear negocio (si no existe)
let negocio = await pg.program.account.negocio.fetchNullable(negocioPda);
if (!negocio) {
  const tx = await pg.program.methods
    .crearNegocio("Barberia Demo")
    .accounts({ negocio: negocioPda, owner, systemProgram: web3.SystemProgram.programId })
    .rpc();
  console.log("Crear negocio:", link(tx));
  negocio = await pg.program.account.negocio.fetch(negocioPda);
}
console.log("Negocio:", negocio.nombre, "| servicios:", negocio.servicios.toString());

// 2) Crear servicio (PDA con el contador del negocio)
const id = new BN(negocio.servicios);
const [servicioPda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("servicio"), negocioPda.toBuffer(), id.toArrayLike(Buffer, "le", 8)],
  programId
);
let tx = await pg.program.methods
  .crearServicio("Corte de cabello", 2)
  .accounts({ negocio: negocioPda, servicio: servicioPda, owner, systemProgram: web3.SystemProgram.programId })
  .rpc();
console.log("Crear servicio:", link(tx));

// 3) Leer servicio
let servicio = await pg.program.account.servicio.fetch(servicioPda);
console.log("Servicio leído:", servicio.nombre, "| cupo:", servicio.cupo, "| activas:", servicio.activas);

// 4) Modificar servicio (cupo 2 -> 3)
tx = await pg.program.methods
  .editarServicio(3)
  .accounts({ negocio: negocioPda, servicio: servicioPda, owner })
  .rpc();
console.log("Editar servicio:", link(tx));

// 5) Reservar
const [reservaPda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("reserva"), servicioPda.toBuffer(), owner.toBuffer()],
  programId
);
tx = await pg.program.methods
  .reservar()
  .accounts({ servicio: servicioPda, reserva: reservaPda, cliente: owner, systemProgram: web3.SystemProgram.programId })
  .rpc();
console.log("Reservar:", link(tx));

// 6) Regla: cerrar servicio con reservas activas DEBE fallar
try {
  await pg.program.methods
    .cerrarServicio()
    .accounts({ negocio: negocioPda, servicio: servicioPda, owner })
    .rpc();
  console.log("ERROR: se cerró con reservas activas");
} catch (e) {
  console.log("OK, rechazado como se esperaba: ReservasActivas");
}

// 7) Confirmar (dueño) y finalizar reserva (cierra la cuenta)
tx = await pg.program.methods
  .confirmarReserva()
  .accounts({ negocio: negocioPda, servicio: servicioPda, reserva: reservaPda, owner })
  .rpc();
console.log("Confirmar reserva:", link(tx));

tx = await pg.program.methods
  .finalizarReserva()
  .accounts({ negocio: negocioPda, servicio: servicioPda, reserva: reservaPda, cliente: owner, owner })
  .rpc();
console.log("Finalizar reserva (cierre):", link(tx));

// 8) Ahora sí: cerrar servicio (0 reservas activas)
tx = await pg.program.methods
  .cerrarServicio()
  .accounts({ negocio: negocioPda, servicio: servicioPda, owner })
  .rpc();
console.log("Cerrar servicio:", link(tx));
console.log("Program ID:", programId.toBase58());
