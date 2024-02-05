export type AccountCompression = {
  "version": "0.3.1",
  "name": "account_compression",
  "constants": [
    {
      "name": "GROUP_AUTHORITY_SEED",
      "type": "bytes",
      "value": "[103, 114, 111, 117, 112, 95, 97, 117, 116, 104, 111, 114, 105, 116, 121]"
    },
    {
      "name": "MERKLE_TREE_HEIGHT",
      "type": {
        "defined": "usize"
      },
      "value": "22"
    },
    {
      "name": "MERKLE_TREE_CHANGELOG",
      "type": {
        "defined": "usize"
      },
      "value": "0"
    },
    {
      "name": "MERKLE_TREE_ROOTS",
      "type": {
        "defined": "usize"
      },
      "value": "2800"
    },
    {
      "name": "PROGRAM_ID",
      "type": "string",
      "value": "\"DmtCHY9V1vqkYfQ5xYESzvGoMGhePHLja9GQ994GKTTc\""
    },
    {
      "name": "NOOP_PROGRAM_ID",
      "type": "publicKey",
      "value": ":: anchor_lang :: prelude :: Pubkey :: new_from_array ([11u8 , 188u8 , 15u8 , 192u8 , 187u8 , 71u8 , 202u8 , 47u8 , 116u8 , 196u8 , 17u8 , 46u8 , 148u8 , 171u8 , 19u8 , 207u8 , 163u8 , 198u8 , 52u8 , 229u8 , 220u8 , 23u8 , 234u8 , 203u8 , 3u8 , 205u8 , 26u8 , 35u8 , 205u8 , 126u8 , 120u8 , 124u8])"
    }
  ],
  "instructions": [
    {
      "name": "initializeGroupAuthority",
      "docs": [
        "initialize group (a group can be used to give multiple programs acess to the same Merkle trees by registering the programs to the group)"
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "groupAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "seed",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "authority",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "updateGroupAuthority",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "groupAuthority",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "authority",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "initializeConcurrentMerkleTree",
      "docs": [
        "Initializes a new Merkle tree from config bytes.",
        "Can only be called from the merkle_tree_authority."
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "merkleTree",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u64"
        },
        {
          "name": "owner",
          "type": "publicKey"
        },
        {
          "name": "delegate",
          "type": {
            "option": "publicKey"
          }
        }
      ]
    },
    {
      "name": "insertLeavesIntoMerkleTrees",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "registeredVerifierPda",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "logWrapper",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "leaves",
          "type": {
            "vec": {
              "array": [
                "u8",
                32
              ]
            }
          }
        }
      ]
    },
    {
      "name": "initializeIndexedArray",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "indexedArray",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u64"
        },
        {
          "name": "owner",
          "type": "publicKey"
        },
        {
          "name": "delegate",
          "type": {
            "option": "publicKey"
          }
        }
      ]
    },
    {
      "name": "insertIntoIndexedArrays",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "registeredVerifierPda",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        }
      ],
      "args": [
        {
          "name": "elements",
          "type": {
            "vec": {
              "array": [
                "u8",
                32
              ]
            }
          }
        },
        {
          "name": "lowElementIndexes",
          "type": {
            "vec": "u16"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "groupAuthority",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "seed",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "indexedArrayAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "delegate",
            "type": "publicKey"
          },
          {
            "name": "array",
            "type": "publicKey"
          },
          {
            "name": "indexedArray",
            "type": {
              "array": [
                "u8",
                112008
              ]
            }
          }
        ]
      }
    },
    {
      "name": "concurrentMerkleTreeAccount",
      "docs": [
        "Concurrent state Merkle tree used for public compressed transactions."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "docs": [
              "Unique index."
            ],
            "type": "u64"
          },
          {
            "name": "nextMerkleTree",
            "docs": [
              "Public key of the next Merkle tree."
            ],
            "type": "publicKey"
          },
          {
            "name": "owner",
            "docs": [
              "Owner of the Merkle tree."
            ],
            "type": "publicKey"
          },
          {
            "name": "delegate",
            "docs": [
              "Delegate of the Merkle tree."
            ],
            "type": "publicKey"
          },
          {
            "name": "stateMerkleTree",
            "docs": [
              "Merkle tree for the transaction state."
            ],
            "type": {
              "array": [
                "u8",
                90368
              ]
            }
          }
        ]
      }
    },
    {
      "name": "registeredProgram",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pubkey",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Changelogs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "changelogs",
            "type": {
              "vec": {
                "defined": "ChangelogEvent"
              }
            }
          }
        ]
      }
    },
    {
      "name": "PathNode",
      "docs": [
        "Node of the Merkle path with an index representing the position in a",
        "non-sparse Merkle tree."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "node",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "index",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "ChangelogEventV1",
      "docs": [
        "Version 1 of the [`ChangelogEvent`](light_merkle_tree_program::state::ChangelogEvent)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "docs": [
              "Public key of the tree."
            ],
            "type": "publicKey"
          },
          {
            "name": "paths",
            "type": {
              "vec": {
                "vec": {
                  "defined": "PathNode"
                }
              }
            }
          },
          {
            "name": "seq",
            "docs": [
              "Number of successful operations on the on-chain tree."
            ],
            "type": "u64"
          },
          {
            "name": "index",
            "docs": [
              "Changelog event index."
            ],
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "ChangelogEvent",
      "docs": [
        "Event containing the Merkle path of the given",
        "[`StateMerkleTree`](light_merkle_tree_program::state::StateMerkleTree)",
        "change. Indexers can use this type of events to re-build a non-sparse",
        "version of state Merkle tree."
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "V1",
            "fields": [
              {
                "defined": "ChangelogEventV1"
              }
            ]
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAuthority",
      "msg": "InvalidAuthority"
    },
    {
      "code": 6001,
      "name": "InvalidVerifier",
      "msg": "InvalidVerifier"
    },
    {
      "code": 6002,
      "name": "NumberOfLeavesMismatch",
      "msg": "Leaves <> remaining accounts missmatch. The number of remaining accounts must match the number of leaves."
    },
    {
      "code": 6003,
      "name": "IntegerOverflow",
      "msg": "Integer overflow, value too large"
    },
    {
      "code": 6004,
      "name": "InvalidNoopPubkey",
      "msg": "Provided noop program public key is invalid"
    },
    {
      "code": 6005,
      "name": "EventNoChangelogEntry",
      "msg": "Emitting an event requires at least one changelog entry"
    }
  ]
};

export const IDL: AccountCompression = {
  "version": "0.3.1",
  "name": "account_compression",
  "constants": [
    {
      "name": "GROUP_AUTHORITY_SEED",
      "type": "bytes",
      "value": "[103, 114, 111, 117, 112, 95, 97, 117, 116, 104, 111, 114, 105, 116, 121]"
    },
    {
      "name": "MERKLE_TREE_HEIGHT",
      "type": {
        "defined": "usize"
      },
      "value": "22"
    },
    {
      "name": "MERKLE_TREE_CHANGELOG",
      "type": {
        "defined": "usize"
      },
      "value": "0"
    },
    {
      "name": "MERKLE_TREE_ROOTS",
      "type": {
        "defined": "usize"
      },
      "value": "2800"
    },
    {
      "name": "PROGRAM_ID",
      "type": "string",
      "value": "\"DmtCHY9V1vqkYfQ5xYESzvGoMGhePHLja9GQ994GKTTc\""
    },
    {
      "name": "NOOP_PROGRAM_ID",
      "type": "publicKey",
      "value": ":: anchor_lang :: prelude :: Pubkey :: new_from_array ([11u8 , 188u8 , 15u8 , 192u8 , 187u8 , 71u8 , 202u8 , 47u8 , 116u8 , 196u8 , 17u8 , 46u8 , 148u8 , 171u8 , 19u8 , 207u8 , 163u8 , 198u8 , 52u8 , 229u8 , 220u8 , 23u8 , 234u8 , 203u8 , 3u8 , 205u8 , 26u8 , 35u8 , 205u8 , 126u8 , 120u8 , 124u8])"
    }
  ],
  "instructions": [
    {
      "name": "initializeGroupAuthority",
      "docs": [
        "initialize group (a group can be used to give multiple programs acess to the same Merkle trees by registering the programs to the group)"
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "groupAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "seed",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "authority",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "updateGroupAuthority",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "groupAuthority",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "authority",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "initializeConcurrentMerkleTree",
      "docs": [
        "Initializes a new Merkle tree from config bytes.",
        "Can only be called from the merkle_tree_authority."
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "merkleTree",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u64"
        },
        {
          "name": "owner",
          "type": "publicKey"
        },
        {
          "name": "delegate",
          "type": {
            "option": "publicKey"
          }
        }
      ]
    },
    {
      "name": "insertLeavesIntoMerkleTrees",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "registeredVerifierPda",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "logWrapper",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "leaves",
          "type": {
            "vec": {
              "array": [
                "u8",
                32
              ]
            }
          }
        }
      ]
    },
    {
      "name": "initializeIndexedArray",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "indexedArray",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u64"
        },
        {
          "name": "owner",
          "type": "publicKey"
        },
        {
          "name": "delegate",
          "type": {
            "option": "publicKey"
          }
        }
      ]
    },
    {
      "name": "insertIntoIndexedArrays",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "registeredVerifierPda",
          "isMut": false,
          "isSigner": false,
          "isOptional": true
        }
      ],
      "args": [
        {
          "name": "elements",
          "type": {
            "vec": {
              "array": [
                "u8",
                32
              ]
            }
          }
        },
        {
          "name": "lowElementIndexes",
          "type": {
            "vec": "u16"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "groupAuthority",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "seed",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "indexedArrayAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "delegate",
            "type": "publicKey"
          },
          {
            "name": "array",
            "type": "publicKey"
          },
          {
            "name": "indexedArray",
            "type": {
              "array": [
                "u8",
                112008
              ]
            }
          }
        ]
      }
    },
    {
      "name": "concurrentMerkleTreeAccount",
      "docs": [
        "Concurrent state Merkle tree used for public compressed transactions."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "docs": [
              "Unique index."
            ],
            "type": "u64"
          },
          {
            "name": "nextMerkleTree",
            "docs": [
              "Public key of the next Merkle tree."
            ],
            "type": "publicKey"
          },
          {
            "name": "owner",
            "docs": [
              "Owner of the Merkle tree."
            ],
            "type": "publicKey"
          },
          {
            "name": "delegate",
            "docs": [
              "Delegate of the Merkle tree."
            ],
            "type": "publicKey"
          },
          {
            "name": "stateMerkleTree",
            "docs": [
              "Merkle tree for the transaction state."
            ],
            "type": {
              "array": [
                "u8",
                90368
              ]
            }
          }
        ]
      }
    },
    {
      "name": "registeredProgram",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pubkey",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Changelogs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "changelogs",
            "type": {
              "vec": {
                "defined": "ChangelogEvent"
              }
            }
          }
        ]
      }
    },
    {
      "name": "PathNode",
      "docs": [
        "Node of the Merkle path with an index representing the position in a",
        "non-sparse Merkle tree."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "node",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "index",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "ChangelogEventV1",
      "docs": [
        "Version 1 of the [`ChangelogEvent`](light_merkle_tree_program::state::ChangelogEvent)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "docs": [
              "Public key of the tree."
            ],
            "type": "publicKey"
          },
          {
            "name": "paths",
            "type": {
              "vec": {
                "vec": {
                  "defined": "PathNode"
                }
              }
            }
          },
          {
            "name": "seq",
            "docs": [
              "Number of successful operations on the on-chain tree."
            ],
            "type": "u64"
          },
          {
            "name": "index",
            "docs": [
              "Changelog event index."
            ],
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "ChangelogEvent",
      "docs": [
        "Event containing the Merkle path of the given",
        "[`StateMerkleTree`](light_merkle_tree_program::state::StateMerkleTree)",
        "change. Indexers can use this type of events to re-build a non-sparse",
        "version of state Merkle tree."
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "V1",
            "fields": [
              {
                "defined": "ChangelogEventV1"
              }
            ]
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAuthority",
      "msg": "InvalidAuthority"
    },
    {
      "code": 6001,
      "name": "InvalidVerifier",
      "msg": "InvalidVerifier"
    },
    {
      "code": 6002,
      "name": "NumberOfLeavesMismatch",
      "msg": "Leaves <> remaining accounts missmatch. The number of remaining accounts must match the number of leaves."
    },
    {
      "code": 6003,
      "name": "IntegerOverflow",
      "msg": "Integer overflow, value too large"
    },
    {
      "code": 6004,
      "name": "InvalidNoopPubkey",
      "msg": "Provided noop program public key is invalid"
    },
    {
      "code": 6005,
      "name": "EventNoChangelogEntry",
      "msg": "Emitting an event requires at least one changelog entry"
    }
  ]
};