"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.VerifierTwo = void 0;
const verifier_program_two_1 = require("../idls/verifier_program_two");
const anchor_1 = require("@coral-xyz/anchor");
const index_1 = require("../index");
const web3_js_1 = require("@solana/web3.js");
class VerifierTwo {
    constructor() {
        this.verifierProgram = new anchor_1.Program(verifier_program_two_1.VerifierProgramTwo, index_1.verifierProgramTwoProgramId);
        this.wtnsGenPath = "transactionApp4_js/transactionApp4.wasm";
        this.zkeyPath = "transactionApp4.zkey";
        this.calculateWtns = require("../../build-circuits/transactionApp4_js/witness_calculator.js");
        this.nrPublicInputs = 15;
        this.config = { in: 4, out: 4 };
        this.pubkey = (0, index_1.hashAndTruncateToCircuit)(new web3_js_1.PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").toBytes());
    }
    parsePublicInputsFromArray(publicInputsBytes) {
        if (publicInputsBytes.length == this.nrPublicInputs) {
            return {
                root: publicInputsBytes[0],
                publicAmount: publicInputsBytes[1],
                extDataHash: publicInputsBytes[2],
                feeAmount: publicInputsBytes[3],
                mintPubkey: publicInputsBytes[4],
                nullifiers: Array.from(publicInputsBytes.slice(5, 9)),
                leaves: Array.from([publicInputsBytes.slice(9, 11), publicInputsBytes.slice(11, 13)]),
                checkedParams: Array.from(publicInputsBytes.slice(13, 15)),
                connectingHash: Array.from(publicInputsBytes.slice(13, 14)),
                verifier: Array.from(publicInputsBytes.slice(14, 15)),
            };
        }
        else {
            throw `publicInputsBytes.length invalid ${publicInputsBytes.length} != ${this.nrPublicInputs}`;
        }
    }
    initVerifierProgram() {
        this.verifierProgram = new anchor_1.Program(verifier_program_two_1.VerifierProgramTwo, index_1.verifierProgramTwoProgramId);
    }
    // Do I need a getData fn?
    // I should be able to fetch everything from the object
    getInstructions(transaction) {
        return __awaiter(this, void 0, void 0, function* () {
            console.log("empty is cpi");
        });
    }
}
exports.VerifierTwo = VerifierTwo;
