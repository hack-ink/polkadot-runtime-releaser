## Polkadot Runtime Releaser Inspect Action
Provides a way to inspect polkadot-sdk-based runtime in GitHub Action with the Polkadot Runtime Releaser (PRR) CLI.

### Example
```yaml
- name: Inspect
      uses: hack-ink/polkadot-runtime-releaser/action/inspect@v0.0.0
      with:
        runtime: polkadot-runtime
        no-check-version: false
        beautify: true
        verbose: false
```
