## Polkadot Runtime Releaser Build Action
Provides a way to build polkadot-sdk-based runtime in GitHub Action with the Polkadot Runtime Releaser (PPR) CLI.

### Example
```yaml
- name: Build
      uses: hack-ink/polkadot-runtime-releaser/action/build@v0.0.0
      with:
        runtime: polkadot-runtime
        features: force-debug,runtime-metrics
        toolchain-ver: 1.81.0
        workdir: .
        output-dir: .
```
