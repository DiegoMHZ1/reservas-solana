use anchor_lang::prelude::*;

// Solana Playground reemplaza este ID automáticamente al hacer Build/Deploy.
declare_id!("11111111111111111111111111111111");

pub const PENDIENTE: u8 = 0;
pub const CONFIRMADA: u8 = 1;

#[program]
pub mod reservas {
    use super::*;

    /// CREAR: el dueño registra su negocio (PDA: ["negocio", owner]).
    pub fn crear_negocio(ctx: Context<CrearNegocio>, nombre: String) -> Result<()> {
        require!(nombre.len() <= 32, ErrorReservas::NombreLargo);
        let n = &mut ctx.accounts.negocio;
        n.owner = ctx.accounts.owner.key();
        n.nombre = nombre;
        n.servicios = 0;
        n.bump = ctx.bumps.negocio;
        Ok(())
    }

    /// CREAR (relacionada): servicio del negocio (PDA: ["servicio", negocio, id]).
    /// PERMISO: solo el dueño del negocio.
    pub fn crear_servicio(ctx: Context<CrearServicio>, nombre: String, cupo: u8) -> Result<()> {
        require!(nombre.len() <= 32, ErrorReservas::NombreLargo);
        require!(cupo > 0, ErrorReservas::CupoInvalido);
        let negocio = &mut ctx.accounts.negocio;
        let s = &mut ctx.accounts.servicio;
        s.negocio = negocio.key();
        s.id = negocio.servicios;
        s.nombre = nombre;
        s.cupo = cupo;
        s.activas = 0;
        s.bump = ctx.bumps.servicio;
        negocio.servicios += 1;
        Ok(())
    }

    /// MODIFICAR: cambiar el cupo. PERMISO: solo el dueño.
    /// REGLA: el cupo no puede quedar por debajo de las reservas activas.
    pub fn editar_servicio(ctx: Context<EditarServicio>, cupo: u8) -> Result<()> {
        let s = &mut ctx.accounts.servicio;
        require!(cupo > 0 && cupo >= s.activas, ErrorReservas::CupoInvalido);
        s.cupo = cupo;
        Ok(())
    }

    /// CREAR (relacionada): un cliente reserva (PDA: ["reserva", servicio, cliente]).
    /// REGLA: debe haber cupo disponible.
    pub fn reservar(ctx: Context<Reservar>) -> Result<()> {
        let s = &mut ctx.accounts.servicio;
        require!(s.activas < s.cupo, ErrorReservas::SinCupo);
        s.activas += 1;
        let r = &mut ctx.accounts.reserva;
        r.servicio = s.key();
        r.cliente = ctx.accounts.cliente.key();
        r.estado = PENDIENTE;
        r.bump = ctx.bumps.reserva;
        Ok(())
    }

    /// MODIFICAR: el dueño confirma una reserva pendiente. PERMISO: solo el dueño.
    pub fn confirmar_reserva(ctx: Context<GestionarReserva>) -> Result<()> {
        let r = &mut ctx.accounts.reserva;
        require!(r.estado == PENDIENTE, ErrorReservas::EstadoInvalido);
        r.estado = CONFIRMADA;
        Ok(())
    }

    /// BAJA CONDICIONADA: el dueño finaliza una reserva CONFIRMADA (cierra la cuenta,
    /// la renta vuelve al cliente).
    pub fn finalizar_reserva(ctx: Context<FinalizarReserva>) -> Result<()> {
        require!(ctx.accounts.reserva.estado == CONFIRMADA, ErrorReservas::EstadoInvalido);
        ctx.accounts.servicio.activas -= 1;
        Ok(())
    }

    /// BAJA CONDICIONADA: el cliente cancela su reserva solo si sigue PENDIENTE.
    /// PERMISO: solo el cliente que la creó.
    pub fn cancelar_reserva(ctx: Context<CancelarReserva>) -> Result<()> {
        require!(ctx.accounts.reserva.estado == PENDIENTE, ErrorReservas::EstadoInvalido);
        ctx.accounts.servicio.activas -= 1;
        Ok(())
    }

    /// CIERRE CONDICIONADO: el dueño cierra un servicio solo si no tiene reservas activas.
    pub fn cerrar_servicio(ctx: Context<CerrarServicio>) -> Result<()> {
        require!(ctx.accounts.servicio.activas == 0, ErrorReservas::ReservasActivas);
        Ok(())
    }
}

// ---------------- Cuentas ----------------

#[account]
#[derive(InitSpace)]
pub struct Negocio {
    pub owner: Pubkey,
    #[max_len(32)]
    pub nombre: String,
    pub servicios: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Servicio {
    pub negocio: Pubkey,
    pub id: u64,
    #[max_len(32)]
    pub nombre: String,
    pub cupo: u8,
    pub activas: u8,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Reserva {
    pub servicio: Pubkey,
    pub cliente: Pubkey,
    pub estado: u8,
    pub bump: u8,
}

// ---------------- Contextos ----------------

#[derive(Accounts)]
pub struct CrearNegocio<'info> {
    #[account(
        init, payer = owner, space = 8 + Negocio::INIT_SPACE,
        seeds = [b"negocio", owner.key().as_ref()], bump
    )]
    pub negocio: Account<'info, Negocio>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CrearServicio<'info> {
    #[account(mut, has_one = owner @ ErrorReservas::NoAutorizado)]
    pub negocio: Account<'info, Negocio>,
    #[account(
        init, payer = owner, space = 8 + Servicio::INIT_SPACE,
        seeds = [b"servicio", negocio.key().as_ref(), &negocio.servicios.to_le_bytes()], bump
    )]
    pub servicio: Account<'info, Servicio>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditarServicio<'info> {
    #[account(has_one = owner @ ErrorReservas::NoAutorizado)]
    pub negocio: Account<'info, Negocio>,
    #[account(mut, has_one = negocio)]
    pub servicio: Account<'info, Servicio>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Reservar<'info> {
    #[account(mut)]
    pub servicio: Account<'info, Servicio>,
    #[account(
        init, payer = cliente, space = 8 + Reserva::INIT_SPACE,
        seeds = [b"reserva", servicio.key().as_ref(), cliente.key().as_ref()], bump
    )]
    pub reserva: Account<'info, Reserva>,
    #[account(mut)]
    pub cliente: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarReserva<'info> {
    #[account(has_one = owner @ ErrorReservas::NoAutorizado)]
    pub negocio: Account<'info, Negocio>,
    #[account(has_one = negocio)]
    pub servicio: Account<'info, Servicio>,
    #[account(mut, has_one = servicio)]
    pub reserva: Account<'info, Reserva>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct FinalizarReserva<'info> {
    #[account(has_one = owner @ ErrorReservas::NoAutorizado)]
    pub negocio: Account<'info, Negocio>,
    #[account(mut, has_one = negocio)]
    pub servicio: Account<'info, Servicio>,
    #[account(mut, has_one = servicio, has_one = cliente, close = cliente)]
    pub reserva: Account<'info, Reserva>,
    /// CHECK: solo recibe la renta de vuelta; validado con has_one en `reserva`.
    #[account(mut)]
    pub cliente: UncheckedAccount<'info>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelarReserva<'info> {
    #[account(mut)]
    pub servicio: Account<'info, Servicio>,
    #[account(
        mut, has_one = servicio, has_one = cliente @ ErrorReservas::NoAutorizado,
        close = cliente
    )]
    pub reserva: Account<'info, Reserva>,
    #[account(mut)]
    pub cliente: Signer<'info>,
}

#[derive(Accounts)]
pub struct CerrarServicio<'info> {
    #[account(has_one = owner @ ErrorReservas::NoAutorizado)]
    pub negocio: Account<'info, Negocio>,
    #[account(mut, has_one = negocio, close = owner)]
    pub servicio: Account<'info, Servicio>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ErrorReservas {
    #[msg("El nombre excede 32 caracteres")]
    NombreLargo,
    #[msg("Cupo inválido")]
    CupoInvalido,
    #[msg("No hay cupo disponible")]
    SinCupo,
    #[msg("No autorizado")]
    NoAutorizado,
    #[msg("Estado de la reserva inválido para esta operación")]
    EstadoInvalido,
    #[msg("El servicio aún tiene reservas activas")]
    ReservasActivas,
}
