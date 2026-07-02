import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TuktukGptOracle } from "../target/types/tuktuk_gpt_oracle";
import { PublicKey } from "@solana/web3.js";

describe("tuktuk-gpt-oracle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TuktukGptOracle as Program<TuktukGptOracle>;
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const llmProgramAddress = new PublicKey(
    "LLMrieZMpbJFwN52WgmBNMxYojrpRVYXdC1RCweEbab"
  );

  const getLLMProgram = async () => {
    const llmProgramIDL = await Program.fetchIdl(llmProgramAddress, provider);
    const llmProgram: any = new Program(llmProgramIDL);
    return llmProgram;
  };

  const GetAgentAndInteraction = async () => {
    const [agentAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("agent")],
      program.programId
    );

    const agent = await program.account.agent.fetch(agentAddress);

    const [interactionAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("interaction"),
        provider.wallet.publicKey.toBuffer(),
        agent.context.toBuffer(),
      ],
      llmProgramAddress
    );

    return { agent, interactionAddress };
  };



  xit("Is initialized!", async () => {
    const llmProgram: any = await getLLMProgram();

    const [counterAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      llmProgramAddress
    );

    const counter = await llmProgram.account.counter.fetch(counterAddress);

    const [llmContext] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("test-context"),
        new anchor.BN(counter.count).toArrayLike(Buffer, "le", 4),
      ],
      llmProgramAddress
    );

    const tx = await program.methods
      .initialize()
      .accounts({
        counter: counterAddress,
        llmContext,
        signer: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("Your count", counter.count);
  });

  it("input agent", async () => {
    // it's more like ur web3 aura score
    const { agent, interactionAddress } = await GetAgentAndInteraction();
    const country = `Nigeria`;

    const tx = await program.methods
      .interactAgent(country)
      .accounts({
        interaction: interactionAddress,
        contextAccount: agent.context,
        user: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature ", tx);
  });


});
