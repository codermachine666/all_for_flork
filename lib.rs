use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;

// Declaramos el ID del contrato en la red de Solana
// Este ID debe coincidir con el de la cuenta que desplegó el programa
declare_id!("4T9LLpoUA5Dmu6q5z9TppZQVZiH32AzBWTC6mmmGncA9");

#[program]
pub mod smart_contract {
    use super::*;

    // Función para inicializar el contrato
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Mensaje para debug: muestra quién está inicializando el contrato
        msg!("Contrato inicializado por: {}", ctx.accounts.authority.key());
        Ok(())
    }

    // Función para retirar los fees acumulados en el contrato
    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        // Validamos que solo el dueño de la cuenta de fees pueda retirarlos
        require_keys_eq!(
            ctx.accounts.authority.key(),
            ctx.accounts.fee_account.owner,
            CustomError::Unauthorized // Mensaje de error si no es el autorizado
        );

        // Configuramos las cuentas necesarias para hacer la transferencia
        let cpi_accounts = Transfer {
            from: ctx.accounts.fee_account.to_account_info(), // Origen: cuenta de fees
            to: ctx.accounts.authority.to_account_info(), // Destino: cuenta del autorizado
            authority: ctx.accounts.authority.to_account_info(), // Autorización
        };

        // Creamos el contexto CPI (Cross-Program Invocation) para hacer la transferencia
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        
        // Ejecutamos la transferencia de fees (en este caso, 1_000_000 unidades de token)
        anchor_spl::token::transfer(cpi_ctx, 1_000_000)?;
        Ok(())
    }
}

// Definimos la estructura de cuentas necesarias para inicializar el contrato
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // El usuario que inicia el contrato
    
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub fee_account: Account<'info, TokenAccount>, // Cuenta de token donde se acumulan los fees
    
    pub mint: Account<'info, Mint>, // El token que se usará para los fees
    pub system_program: Program<'info, System>, // Programa del sistema de Solana
    pub token_program: Program<'info, Token>, // Programa SPL para manejar tokens
    pub associated_token_program: Program<'info, AssociatedToken>, // Programa para cuentas asociadas
}

// Estructura de cuentas necesarias para retirar los fees
#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // El usuario que intenta retirar los fees
    
    #[account(mut)]
    pub fee_account: Account<'info, TokenAccount>, // La cuenta donde están los fees
    
    pub token_program: Program<'info, Token>, // Programa SPL para manejar tokens
}

// Definimos un enum de errores personalizados
#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access.")]
    Unauthorized, // Error que se lanza si alguien no autorizado intenta retirar los fees
}
