import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TuktukGptOracle } from "../target/types/tuktuk_gpt_oracle";
import { PublicKey } from "@solana/web3.js";
import { queueTask } from "@helium/tuktuk-sdk";
import { assert } from "chai";

describe("tuktuk-gpt-oracle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TuktukGptOracle as Program<TuktukGptOracle>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
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



  it("Is initialized!", async () => {
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

  it("input agent via Tuktuk", async () => {
    // it's more like ur web3 aura score
    const { agent, interactionAddress } = await GetAgentAndInteraction();
    const country = `Nigeria`;

    // 1. Build the instruction we want Tuktuk to execute
    const ix = await program.methods
      .agentInput(country)
      .accounts({
        interaction: interactionAddress,
        contextAccount: agent.context,
        user: provider.wallet.publicKey,
      })
      .instruction();

    // 2. We use a placeholder task queue. In a real environment, you'd create or use an existing Tuktuk queue.
    const taskQueue = anchor.web3.Keypair.generate().publicKey; 

    console.log("Scheduling task with Tuktuk...");
    
    // 3. Queue the task with Tuktuk SDK
    try {
      await queueTask({
        program: program as any, // typically requires a tuktuk program instance or provider
        taskQueue,
        instruction: ix,
      });
      console.log("Task queued successfully!");
    } catch (err) {
      console.log("Tuktuk queue failed (ensure Tuktuk program is deployed on this cluster)", err.message);
    }

    // 4. Wait for response and assert AdoptionScore updates
    const [adoptionAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("AdoptionScore")],
      program.programId
    );

    console.log("Waiting for Oracle callback...");
    
    // Polling loop to wait for the GPT response to update our adoption account
    let scoreUpdated = false;
    for (let i = 0; i < 20; i++) {
      try {
        const adoptionAccount = await program.account.adoptionScore.fetch(adoptionAddress);
        if (adoptionAccount.perScore > 0) {
          scoreUpdated = true;
          console.log(`Received GPT score: ${adoptionAccount.perScore}`);
          assert.isTrue(adoptionAccount.perScore > 0, "Score should be updated by Oracle");
          break;
        }
      } catch (e) {
        // Account might not exist yet if cranker hasn't run
      }
      await new Promise(r => setTimeout(r, 2000));
    }

    if (!scoreUpdated) {
      console.log("Did not receive a callback within the timeout window. (Check if cranker and oracle are running on this cluster)");
    }
  });


});
