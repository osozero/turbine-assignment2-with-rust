use solana_idlgen::idlgen;

// idlgen!({
//     "version": "0.1.0",
//     "name": "wba_prereq",
//     "metadata": {
//       "address": "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1"
//     },
//     "instructions": [
//       {
//         "name": "complete",
//         "accounts": [
//           {
//             "name": "signer",
//             "isMut": true,
//             "isSigner": true
//           },
//           {
//             "name": "prereq",
//             "isMut": true,
//             "isSigner": false
//           },
//           {
//             "name": "systemProgram",
//             "isMut": false,
//             "isSigner": false
//           }
//         ],
//         "args": [
//           {
//             "name": "github",
//             "type": "bytes"
//           }
//         ]
//       },
//       {
//         "name": "update",
//         "accounts": [
//           {
//             "name": "signer",
//             "isMut": true,
//             "isSigner": true
//           },
//           {
//             "name": "prereq",
//             "isMut": true,
//             "isSigner": false
//           },
//           {
//             "name": "systemProgram",
//             "isMut": false,
//             "isSigner": false
//           }
//         ],
//         "args": [
//           {
//             "name": "github",
//             "type": "bytes"
//           }
//         ]
//       }
//     ],
//     "accounts": [
//       {
//         "name": "PrereqAccount",
//         "type": {
//           "kind": "struct",
//           "fields": [
//             {
//               "name": "github",
//               "type": "bytes"
//             },
//             {
//               "name": "key",
//               "type": "publicKey"
//             }
//           ]
//         }
//       }
//     ],
//     "errors": [
//       {
//         "code": 6000,
//         "name": "InvalidGithubAccount",
//         "msg": "Invalid Github account"
//       }
//     ]
//   });

idlgen!(
    {   "version": "0.1.0",
        "name": "turbine_prereq",
        "spec": "0.1.0",
        "description": "Created with Anchor",
        "metadata": {
        "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
        },
        "instructions": [
          {
            "name": "complete",
            "discriminator": [
              0,
              77,
              224,
              147,
              136,
              25,
              88,
              76
            ],
            "accounts": [
              {
                "name": "signer",
                "isMut": true,
                "isSigner": true
              },
              {
                "name": "prereq",
                "isMut": true,
                "isSigner": false,
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
                "address": "11111111111111111111111111111111",
                "isMut": false,
                "isSigner": false
              }
            ],
            "args": [
              {
                "name": "github",
                "type": "bytes"
              }
            ]
          },
          {
            "name": "update",
            "discriminator": [
              219,
              200,
              88,
              176,
              158,
              63,
              253,
              127
            ],
            "accounts": [
              {
                "name": "signer",
                "isMut": true,
                "isSigner": true
              },
              {
                "name": "prereq",
                "isMut": true,
                "isSigner": false
              },
              {
                "name": "system_program",
                "isSigner": false,
                "isMut": false,
                "address": "11111111111111111111111111111111"
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
            "name": "SolanaCohort5Account",
            "discriminator": [
              167,
              81,
              85,
              136,
              32,
              169,
              137,
              77
            ]
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
        ]
      }
);