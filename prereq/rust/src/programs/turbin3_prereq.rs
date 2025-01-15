use solana_idlgen::idlgen;

idlgen!(
  {
    "name": "turbine_prereq",
    "version": "0.1.0",
    "instructions": [
      {
        "name": "complete",
        "discriminator": [0, 77, 224, 147, 136, 25, 88, 76],
        "accounts": [
          {
            "name": "signer",
            "isMut": true,
            "isSigner": true,
            "type": "signer"
          },
          {
            "name": "prereq",
            "isMut": true,
            "isSigner": false,
            "type": "SolanaCohort5Account",
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "value": [
                    112,
                    114,
                    101,
                    114,
                    101,
                    113
                  ]
                },
                {
                  "kind": "account",
                  "path": "signer"
                }
              ]
            }
          },
          {
            "name": "system_program",
            "isMut": false,
            "isSigner": false,
            "type": "program"
          }
        ],
        "args": [
          {
            "name": "github",
            "type": "bytes"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "SolanaCohort5Account",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "github",
              "type": "bytes"
            },
            {
              "name": "key",
              "type": "pubkey"
            }
          ]
        },
        "discriminator": [167, 81, 85, 136, 32, 169, 137, 77]
      }
    ],
    "errors": [
      {
        "code": 6000,
        "name": "InvalidGithubAccount",
        "msg": "Invalid Github account"
      }
    ],
    "types": [
      {
        "name": "SolanaCohort5Account",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "github",
              "type": "bytes"
            },
            {
              "name": "key",
              "type": "pubkey"
            }
          ]
        }
      }
    ],
    "metadata": {
      "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
    }
  }
);
