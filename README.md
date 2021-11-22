# Dependencies

To install solana + Anchor dev worksapce (M1 mac):

Solana: 
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.8.0/install)"
```

Anchor: 
```bash
cargo install --git https://github.com/project-serum/anchor --tag v0.18.2 anchor-cli --locked
```

## Check installation
```bash
anchor --version
```

### Anchor CLI commands
New project:
```bash
anchor init <new-project-name>
```

Builds programs in the workspace targeting Solana's BPF runtime and emitting IDLs in the `target/idl` directory

```bash
anchor build --verifiable
```

[More commands](https://project-serum.github.io/anchor/cli/commands.html#init)

## M1 docs (ignore if not using apple silicon)
When trying to run the solana-test-validator to create a local test node this error pops up:

```bash
dyld[50832]: Library not loaded: /usr/local/opt/openssl@1.1/lib/libssl.1.1.dylib
  Referenced from: /Users/sensei/.local/share/solana/install/releases/1.8.5/solana-release/bin/solana-test-validator
  Reason: tried: '/usr/local/opt/openssl@1.1/lib/libssl.1.1.dylib' (no such file), '/usr/local/lib/libssl.1.1.dylib' (no such file), '/usr/lib/libssl.1.1.dylib' (no such file)
zsh: abort      solana-test-validator
```

Trying [this](https://stackoverflow.com/questions/59006602/dyld-library-not-loaded-usr-local-opt-openssl-lib-libssl-1-0-0-dylib) stackoverflow solution doesn't seem to work (at least in my machine hehe)

Then found [this issue](https://github.com/rbenv/homebrew-tap/issues/1) on github which led me to [this other one](https://github.com/rbenv/homebrew-tap/issues/4) but this last one stopped working.

The last issue on this topic seemed to be [#2](https://github.com/rbenv/homebrew-tap/pull/2)

At this point (I think) the correct version of openssl is installed but still unable to run `solana-test-validator`

Other related [issue](https://github.com/solana-labs/example-helloworld/issues/200)

Last valuable resource i could find was this dev.to [post](https://dev.to/nickgarfield/how-to-install-solana-dev-tools-on-an-m1-mac-kfn)

I got to the conclusion that M1 support is not on the priority list, and that solana dev workflow is by now very Rosseta dependent which sucks because Rosseta was meant to be temporary and this people are using it as a requirement to develop on the Solana ecosystem