## Polkadot Runtime Releaser Docker Image
### An environment for constructing a deterministic polkadot-sdk-based runtime.
This image comprises two components: the [`Dockerfile`](Dockerfile) and [`entrypoint.sh`](entrypoint.sh).

The primary role of `entrypoint.sh` is to dynamically load the user’s `UID` and `GID`, ensuring that the generated output remains highly compatible with the current user environment.
Polkadot Runtime Releaser CLI will automatically load the current user’s `UID` and `GID` and utilize them in the container.

#### Publish Multi-Arch Docker Image
```sh
docker buildx create --use
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t ghcr.io/hack-ink/polkadot-runtime-releaser:X.Y.Z \
  -t ghcr.io/hack-ink/polkadot-runtime-releaser:latest \
  . \
  --push
```

#### Publish New Version from Existing Version
```sh
docker buildx imagetools create \
  ghcr.io/hack-ink/polkadot-runtime-releaser:X.Y.Z \
  --tag ghcr.io/hack-ink/polkadot-runtime-releaser:latest
```
