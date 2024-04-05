package prover

import (
	"github.com/consensys/gnark/frontend"
	"github.com/reilabs/gnark-lean-extractor/v2/abstractor"
)

type CombinedCircuit struct {
	Inclusion    InclusionCircuit
	NonInclusion NonInclusionCircuit
}

func (circuit *CombinedCircuit) Define(api frontend.API) error {
	abstractor.CallVoid(api, InclusionProof{
		Root:           circuit.Inclusion.Root,
		Leaf:           circuit.Inclusion.Leaf,
		InPathElements: circuit.Inclusion.InPathElements,
		InPathIndices:  circuit.Inclusion.InPathIndices,
		NumberOfUtxos:  circuit.Inclusion.NumberOfUtxos,
		Depth:          circuit.Inclusion.Depth,
	})

	abstractor.CallVoid(api, NonInclusionProof{
		Root:                 circuit.NonInclusion.Root,
		Value:                circuit.NonInclusion.Value,
		LeafLowerRangeValue:  circuit.NonInclusion.LeafLowerRangeValue,
		LeafHigherRangeValue: circuit.NonInclusion.LeafHigherRangeValue,
		LeafIndex:            circuit.NonInclusion.LeafIndex,
		InPathIndices:        circuit.NonInclusion.InPathIndices,
		InPathElements:       circuit.NonInclusion.InPathElements,
		NumberOfUtxos:        circuit.NonInclusion.NumberOfUtxos,
		Depth:                circuit.NonInclusion.Depth,
	})
	return nil
}

func ImportCombinedSetup(inclusionTreeDepth uint32, inclusionNumberOfUtxos uint32, nonInclusionTreeDepth uint32, nonInclusionNumberOfUtxos uint32, pkPath string, vkPath string) (*ProvingSystem, error) {
	ccs, err := R1CSCombined(inclusionTreeDepth, inclusionNumberOfUtxos, nonInclusionTreeDepth, nonInclusionNumberOfUtxos)
	if err != nil {
		return nil, err
	}

	pk, err := LoadProvingKey(pkPath)

	if err != nil {
		return nil, err
	}

	vk, err := LoadVerifyingKey(vkPath)

	if err != nil {
		return nil, err
	}

	return &ProvingSystem{inclusionTreeDepth, inclusionNumberOfUtxos, nonInclusionTreeDepth, nonInclusionNumberOfUtxos, pk, vk, ccs}, nil
}
