import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {Tracker} from "../target/types/tracker";

describe("tracker_contracts", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.tracker as Program<Tracker>;

    it("Is initialized!", async () => {
        // Add your test here.
        const tx = await program.methods.initialize().rpc();
        const connection = anchor.getProvider().connection;
        const parsed = await connection.getParsedTransaction(tx, {commitment: "confirmed"});
        console.log(parsed.meta.logMessages);
        console.log("Your transaction signature", tx);
    });
});
