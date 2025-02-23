import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SmartContract } from "../target/types/smart_contract";
import assert from "assert";
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  createMint, 
  getAssociatedTokenAddress, 
  mintTo, 
  createAssociatedTokenAccount 
} from "@solana/spl-token";

describe("smart_contract", () => {
  // Configuración del proveedor y programa
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SmartContract as Program<SmartContract>;
  const authority = provider.wallet.publicKey;

  let mint: anchor.web3.PublicKey;
  let feeAccount: anchor.web3.PublicKey;

  it("Inicializa el contrato correctamente", async () => {
    // ✅ Crear el mint del token
    mint = await createMint(
      provider.connection,
      provider.wallet.payer,
      authority, // Authority del mint
      null, // Freeze authority (opcional)
      9 // Decimales
    );

    console.log("✅ Mint creado correctamente:", mint.toBase58());

    // ✅ Obtener la cuenta de tarifas asociada (ATA)
    feeAccount = await getAssociatedTokenAddress(mint, authority);

    // ✅ Asegurar que la cuenta de tarifas existe
    try {
      await createAssociatedTokenAccount(
        provider.connection,
        provider.wallet.payer,
        mint,
        authority
      );
      console.log("✅ Cuenta de tarifas creada:", feeAccount.toBase58());
    } catch (error) {
      console.log("⚠️ La cuenta de tarifas ya existe o no se pudo crear:", error);
    }

    // ✅ Mintear algunos tokens al feeAccount para pruebas
    await mintTo(provider.connection, provider.wallet.payer, mint, feeAccount, authority, 1000 * 10 ** 9);
    console.log("✅ Tokens minteados a la cuenta de tarifas");

    // ✅ Inicializar el contrato
    const tx = await program.methods
      .initialize()
      .accounts({
        feeAccount,
        authority,
        mint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("✅ Contrato inicializado correctamente:", tx);

    // ✅ Verificar que la cuenta de tarifas se haya creado correctamente
    const feeAccountInfo = await provider.connection.getAccountInfo(feeAccount);
    assert.ok(feeAccountInfo !== null, "La cuenta de tarifas no fue creada correctamente");
  });

  it("Debe permitir al authority retirar fees", async () => {
    try {
      // ✅ Ejecutar el método `withdraw_fees`
      const tx = await program.methods
        .withdrawFees()
        .accounts({
          feeAccount,
          authority,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log("✅ Fees retirados con éxito:", tx);
    } catch (error) {
      console.error("❌ Error al retirar fees:", error);
      assert.fail("No se pudo retirar los fees correctamente");
    }
  });
});
