import { describe, it, expect } from 'vitest';
import { LightSystemProgram } from '../../src/programs';

describe.only('Serialization test', () => {
  it('serialize utxo ', async () => {
    const utxoData = [
      81, 108, 50, 181, 0, 73, 91, 197, 221, 215, 106, 69, 5, 107, 146, 252, 37,
      252, 123, 175, 62, 200, 168, 230, 111, 6, 217, 71, 108, 186, 184, 83, 1,
      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
      1, 1, 1, 1, 1, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const datalen = utxoData.length;

    const deserializedUtxo = LightSystemProgram.program.coder.types.decode(
      'Utxo',
      Buffer.from(utxoData),
    );

    expect(deserializedUtxo.length).toBe(datalen);
  });

  it('serialize out utxo ', async () => {
    const utxoData = [
      65, 108, 61, 176, 52, 117, 234, 133, 198, 175, 67, 171, 12, 47, 143, 190,
      40, 85, 133, 139, 248, 63, 224, 103, 49, 223, 64, 138, 92, 25, 160, 29, 2,
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
      2, 2, 2, 2, 2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const deserializedUtxo = LightSystemProgram.program.coder.types.decode(
      'Utxo',
      Buffer.from(utxoData),
    );
  });

  it('serialize event ', async () => {
    const data = [
      1, 0, 0, 0, 81, 108, 50, 181, 0, 73, 91, 197, 221, 215, 106, 69, 5, 107,
      146, 252, 37, 252, 123, 175, 62, 200, 168, 230, 111, 6, 217, 71, 108, 186,
      184, 83, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
      65, 108, 61, 176, 52, 117, 234, 133, 198, 175, 67, 171, 12, 47, 143, 190,
      40, 85, 133, 139, 248, 63, 224, 103, 49, 223, 64, 138, 92, 25, 160, 29, 2,
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
      2, 2, 2, 2, 2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
      0, 0, 0, 0, 0, 0,
    ];
    let deserializedEvent = LightSystemProgram.program.coder.types.decode(
      'PublicTransactionEvent',
      Buffer.from(data),
    );
  });
});
