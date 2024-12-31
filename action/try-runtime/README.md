## Polkadot Runtime Releaser Try-Runtime Action
Provides a way to run try-runtime for the polkadot-sdk-based runtime in GitHub Action with the Polkadot Runtime Releaser (PRR) CLI.

### Example
```yaml
- name: Tru-Runtime
      uses: hack-ink/polkadot-runtime-releaser/action/override@v0.0.0
      with:
        runtime: polkadot-runtime
        features: on-chain-release-build
        toolchain-ver: 1.81.0
        try-runtime-ver: 0.8.0
        disable-spec-version-check: true
        disable-idempotency-checks: false
        tri: wss://polkadot.public.curie.radiumblock.co
```
