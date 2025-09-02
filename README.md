# Rust Kuberhealthy Client and Example

This repository provides a minimal library and example check for reporting results back to [Kuberhealthy](https://github.com/kuberhealthy/kuberhealthy).
The reusable client library now lives in the [`kuberhealthy-client`](./kuberhealthy-client) subcrate and can be imported into your own Rust programs. The repository root contains the `kuberhealthy-example` binary, which demonstrates how to use the client in a check container.

## Using the client library

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
kuberhealthy-client = "0.1"
```

Example usage:

```rust
use kuberhealthy_client::KuberhealthyClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KuberhealthyClient::from_env()?;
    // Perform check logic here...
    client.report_success()?;
    Ok(())
}
```

The client reads the `KH_REPORTING_URL` and `KH_RUN_UUID` environment variables provided to every check pod and posts a JSON report with the `kh-run-uuid` header.

## Example check

Run without arguments to report success or with `--fail` to report a failure.

```bash
KH_REPORTING_URL="http://localhost" KH_RUN_UUID="test" cargo run --bin kuberhealthy-example
KH_REPORTING_URL="http://localhost" KH_RUN_UUID="test" cargo run --bin kuberhealthy-example -- --fail
```

## Build and push

Replace the placeholder logic in `src/main.rs` with your own check logic and then build and publish the container image:

```bash
make build          # compile the binary
IMAGE=myrepo/kuberhealthy-rust-example:latest make docker-build
IMAGE=myrepo/kuberhealthy-rust-example:latest make push
```

## Deploy with a KuberhealthyCheck

After the image is pushed, reference it from a `KuberhealthyCheck` resource:

```yaml
apiVersion: kuberhealthy.github.io/v2
kind: KuberhealthyCheck
metadata:
  name: rust-example
spec:
  runInterval: 30s
  timeout: 2m
  podSpec:
    containers:
    - name: main
      image: myrepo/kuberhealthy-rust-example:latest
      imagePullPolicy: IfNotPresent
```

Apply the resource to any cluster where Kuberhealthy is running:

```bash
kubectl apply -f khcheck.yaml
```

## Continuous integration and releases

A GitHub Actions workflow builds the example container on every change. When a release is published, another workflow publishes the `kuberhealthy-client` crate from the subdirectory and pushes the example container image to the GitHub Container Registry.

