whispering_oaks  (workspace crate)
│
├── bin_main (binary crate)
│   ├── src
│   │   └── bin_main.rs (main launch point)
│   └── Cargo.toml
│       [[bin]]
│       name = "wo"
│       path = "src/main.rs"
│
├── lib_misc(lib crate, miscellaneous)
│   ├── src
│   │   ├── lib.rs  (root file of the crate)
│   │   ├── gen.rs (file for generating file/data related functions)
│   │   └── env.rs (file for system environment related functions)
│   └── Cargo.toml
│
├── lib_args (lib crate, command-line arguments)
│   ├── src
│   │   ├── lib.rs  (root file of the crate)
│   │   └── match.rs (file for processing CLI arguments)
│   └── Cargo.toml
│
├── lib_cipher (lib crate, encryption/decryption)
│   ├── src
│   │   ├── lib.rs  (root file of the crate)
│   │   ├── encrypt.rs (file for all encryption related functions)
│   │   └── decrypt.rs (file for all decryption related functions)
│   └── Cargo.toml
│
├── lib_fs (lib crate, filesystem interaction and activities)
│   ├── src
│   │   ├── lib.rs  (root file of the crate)
│   │   ├── verify.rs (file for 'fs' verification(s) related functions)
│   │   ├── make.rs (file for crafting/creation related functions)
│   │   └── wreck.rs (file for destroying/removing related functions)
│   └── Cargo.toml
│
└── lib_proc (lib crate, bin execution)
    │   ├── lib.rs  (root file of the crate)
    │   └── init.rs (file to start the selected editor, commands, etc.)
    └── Cargo.toml
