import { log } from "../../utils/logger";
import * as anchor from "@coral-xyz/anchor";
import ora from 'ora';
import {
    ADMIN_AUTH_KEYPAIR,
} from "light-sdk";
import { getLocalProvider, getWalletConfig, readPayerFromIdJson } from "../../utils/utils"
import { Command, program } from "commander";


export const configure = new Command("configure").argument("method")
    .description("Update the configuration of the merkle Tree nfts , permissions less spl tokens and lock duration")
    .option("-l , --lockDuration [lockDuration]", "Update the lockDuration configuration")
    .description("initialize the Merkle Tree Authority")
    .action(async (command: string, options: any) => {

        let spinner = ora('Updating Merkle Tree Configuration...\n')

        try {
            const payer = new anchor.Wallet(readPayerFromIdJson());
            const provider = await getLocalProvider(payer);
            let merkleTreeConfig = await getWalletConfig(provider)

            if (command === "nfts") {
                try {
                    spinner.start("Updating NFT Merkle Tree Configuration...");
                    const tx = await merkleTreeConfig.enableNfts(true);
                }
                catch (err) {
                    throw err;
                }
            }

            else if (command === "spl") {
                try {
                    spinner.start("Updating SPL Merkle Tree Configuration...");
                    await merkleTreeConfig.enablePermissionlessSplTokens(true);
                }
                catch (err) {
                    throw err;
                }
            }

            else if (command === "lock") {
                try {
                    spinner.start("Updating Lock Merkle Tree Configuration...");
                    await merkleTreeConfig.updateLockDuration(parseInt(program.args[2]));

                }
                catch (err) {
                    throw err;
                }
            }
            else {
                log("Invalid commad try using [nfts,spl,lock] along with configure command");
            }
            spinner.succeed("Merkle Tree Configuration updated successfully");
        } catch (error) {
            spinner.fail("Error updating Merkle Tree Configuration");
            let errorMessage = "Aborted.";
            if (error instanceof Error) {
                errorMessage = error.message;
            }
            // @ts-ignore
            if (error.logs && error.logs.length > 0) {
                // @ts-ignore
                errorMessage = error.logs;
            }
            log(errorMessage, "error");
        }

    })