export type SquirclDid = {
  "version": "0.1.0",
  "name": "squircl_did",
  "instructions": [
    {
      "name": "createDid",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "did",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "didStr",
          "type": "string"
        },
        {
          "name": "sig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "addAddress",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "did",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "didStr",
          "type": "string"
        },
        {
          "name": "newAddressSig",
          "type": {
            "defined": "Sig"
          }
        },
        {
          "name": "controllerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "removeAddress",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "did",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "didStr",
          "type": "string"
        },
        {
          "name": "addressChain",
          "type": {
            "defined": "Chain"
          }
        },
        {
          "name": "address",
          "type": "string"
        },
        {
          "name": "removerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "issueCredential",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "credential",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "issuerDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "subjectDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "credentialId",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        },
        {
          "name": "credentialHash",
          "type": "string"
        },
        {
          "name": "isMutable",
          "type": "bool"
        },
        {
          "name": "isRevokable",
          "type": "bool"
        },
        {
          "name": "expiresAt",
          "type": {
            "option": "i64"
          }
        },
        {
          "name": "issuerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "updateCredential",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "credential",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "issuerDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "subjectDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "credentialId",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        },
        {
          "name": "credentialHash",
          "type": "string"
        },
        {
          "name": "isMutable",
          "type": "bool"
        },
        {
          "name": "isRevokable",
          "type": "bool"
        },
        {
          "name": "expiresAt",
          "type": {
            "option": "i64"
          }
        },
        {
          "name": "issuerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "revokeCredential",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "credential",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "issuerDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "subjectDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "credentialId",
          "type": "string"
        },
        {
          "name": "issuerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "credential",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "issuerDid",
            "type": "string"
          },
          {
            "name": "subjectDid",
            "type": "string"
          },
          {
            "name": "issuedAt",
            "type": "i64"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "isRevokable",
            "type": "bool"
          },
          {
            "name": "expiresAt",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "credentialId",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          },
          {
            "name": "credentialHash",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "did",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "did",
            "type": "string"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "ethAddresses",
            "type": {
              "vec": {
                "defined": "Address"
              }
            }
          },
          {
            "name": "solAddresses",
            "type": {
              "vec": {
                "defined": "Address"
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Address",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "string"
          },
          {
            "name": "addedAt",
            "type": "i64"
          },
          {
            "name": "chain",
            "type": {
              "defined": "Chain"
            }
          },
          {
            "name": "role",
            "type": {
              "defined": "Role"
            }
          }
        ]
      }
    },
    {
      "name": "EthSig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "addressBase58",
            "type": "string"
          },
          {
            "name": "sigBase58",
            "type": "string"
          },
          {
            "name": "recoveryId",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "SolSig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "addressBase58",
            "type": "string"
          },
          {
            "name": "sigBase58",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "Role",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Controller"
          },
          {
            "name": "Admin"
          },
          {
            "name": "Assertion"
          },
          {
            "name": "Authentication"
          }
        ]
      }
    },
    {
      "name": "Chain",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "EVM"
          },
          {
            "name": "SOL"
          }
        ]
      }
    },
    {
      "name": "Sig",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Eth",
            "fields": [
              {
                "name": "eth_sig",
                "type": {
                  "defined": "EthSig"
                }
              },
              {
                "name": "index",
                "type": "u8"
              },
              {
                "name": "nonce",
                "type": "i64"
              }
            ]
          },
          {
            "name": "Sol",
            "fields": [
              {
                "name": "sol_sig",
                "type": {
                  "defined": "SolSig"
                }
              },
              {
                "name": "index",
                "type": "u8"
              },
              {
                "name": "nonce",
                "type": "i64"
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
      "name": "InvalidSignatureLength",
      "msg": "Invalid signature length"
    },
    {
      "code": 6001,
      "name": "InvalidSignature",
      "msg": "Invalid signature"
    },
    {
      "code": 6002,
      "name": "InvalidAddress",
      "msg": "Invalid addres"
    },
    {
      "code": 6003,
      "name": "AddressAlreadyExists",
      "msg": "Address already exists"
    },
    {
      "code": 6004,
      "name": "AddressDoesntHaveEnoughPermissions",
      "msg": "Address doesn't have enough permissions"
    },
    {
      "code": 6005,
      "name": "CannotRemoveControllerAddress",
      "msg": "Cannot remove controller address"
    },
    {
      "code": 6006,
      "name": "ExpiryCannotBeInThePast",
      "msg": "Expiry cannot be in the past"
    },
    {
      "code": 6007,
      "name": "CredentialIsNotMutable",
      "msg": "Credential is not mutable"
    },
    {
      "code": 6008,
      "name": "CredentialIsNotRevokable",
      "msg": "Credential i not revokable"
    },
    {
      "code": 6009,
      "name": "AddressDoesNotExistInDID",
      "msg": "Address does not exist in DID"
    },
    {
      "code": 6010,
      "name": "NonceExpired",
      "msg": "Nonce expired"
    }
  ]
};

export const IDL: SquirclDid = {
  "version": "0.1.0",
  "name": "squircl_did",
  "instructions": [
    {
      "name": "createDid",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "did",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "didStr",
          "type": "string"
        },
        {
          "name": "sig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "addAddress",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "did",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "didStr",
          "type": "string"
        },
        {
          "name": "newAddressSig",
          "type": {
            "defined": "Sig"
          }
        },
        {
          "name": "controllerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "removeAddress",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "did",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "didStr",
          "type": "string"
        },
        {
          "name": "addressChain",
          "type": {
            "defined": "Chain"
          }
        },
        {
          "name": "address",
          "type": "string"
        },
        {
          "name": "removerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "issueCredential",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "credential",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "issuerDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "subjectDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "credentialId",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        },
        {
          "name": "credentialHash",
          "type": "string"
        },
        {
          "name": "isMutable",
          "type": "bool"
        },
        {
          "name": "isRevokable",
          "type": "bool"
        },
        {
          "name": "expiresAt",
          "type": {
            "option": "i64"
          }
        },
        {
          "name": "issuerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "updateCredential",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "credential",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "issuerDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "subjectDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "credentialId",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        },
        {
          "name": "credentialHash",
          "type": "string"
        },
        {
          "name": "isMutable",
          "type": "bool"
        },
        {
          "name": "isRevokable",
          "type": "bool"
        },
        {
          "name": "expiresAt",
          "type": {
            "option": "i64"
          }
        },
        {
          "name": "issuerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    },
    {
      "name": "revokeCredential",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "credential",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "issuerDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "subjectDid",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ixSysvar",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "credentialId",
          "type": "string"
        },
        {
          "name": "issuerSig",
          "type": {
            "defined": "Sig"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "credential",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "issuerDid",
            "type": "string"
          },
          {
            "name": "subjectDid",
            "type": "string"
          },
          {
            "name": "issuedAt",
            "type": "i64"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "isRevokable",
            "type": "bool"
          },
          {
            "name": "expiresAt",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "credentialId",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          },
          {
            "name": "credentialHash",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "did",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "did",
            "type": "string"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "ethAddresses",
            "type": {
              "vec": {
                "defined": "Address"
              }
            }
          },
          {
            "name": "solAddresses",
            "type": {
              "vec": {
                "defined": "Address"
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Address",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "string"
          },
          {
            "name": "addedAt",
            "type": "i64"
          },
          {
            "name": "chain",
            "type": {
              "defined": "Chain"
            }
          },
          {
            "name": "role",
            "type": {
              "defined": "Role"
            }
          }
        ]
      }
    },
    {
      "name": "EthSig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "addressBase58",
            "type": "string"
          },
          {
            "name": "sigBase58",
            "type": "string"
          },
          {
            "name": "recoveryId",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "SolSig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "addressBase58",
            "type": "string"
          },
          {
            "name": "sigBase58",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "Role",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Controller"
          },
          {
            "name": "Admin"
          },
          {
            "name": "Assertion"
          },
          {
            "name": "Authentication"
          }
        ]
      }
    },
    {
      "name": "Chain",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "EVM"
          },
          {
            "name": "SOL"
          }
        ]
      }
    },
    {
      "name": "Sig",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Eth",
            "fields": [
              {
                "name": "eth_sig",
                "type": {
                  "defined": "EthSig"
                }
              },
              {
                "name": "index",
                "type": "u8"
              },
              {
                "name": "nonce",
                "type": "i64"
              }
            ]
          },
          {
            "name": "Sol",
            "fields": [
              {
                "name": "sol_sig",
                "type": {
                  "defined": "SolSig"
                }
              },
              {
                "name": "index",
                "type": "u8"
              },
              {
                "name": "nonce",
                "type": "i64"
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
      "name": "InvalidSignatureLength",
      "msg": "Invalid signature length"
    },
    {
      "code": 6001,
      "name": "InvalidSignature",
      "msg": "Invalid signature"
    },
    {
      "code": 6002,
      "name": "InvalidAddress",
      "msg": "Invalid addres"
    },
    {
      "code": 6003,
      "name": "AddressAlreadyExists",
      "msg": "Address already exists"
    },
    {
      "code": 6004,
      "name": "AddressDoesntHaveEnoughPermissions",
      "msg": "Address doesn't have enough permissions"
    },
    {
      "code": 6005,
      "name": "CannotRemoveControllerAddress",
      "msg": "Cannot remove controller address"
    },
    {
      "code": 6006,
      "name": "ExpiryCannotBeInThePast",
      "msg": "Expiry cannot be in the past"
    },
    {
      "code": 6007,
      "name": "CredentialIsNotMutable",
      "msg": "Credential is not mutable"
    },
    {
      "code": 6008,
      "name": "CredentialIsNotRevokable",
      "msg": "Credential i not revokable"
    },
    {
      "code": 6009,
      "name": "AddressDoesNotExistInDID",
      "msg": "Address does not exist in DID"
    },
    {
      "code": 6010,
      "name": "NonceExpired",
      "msg": "Nonce expired"
    }
  ]
};
