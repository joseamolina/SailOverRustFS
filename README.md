# Sail Over RustFS - Test Execution Guide

This project demonstrates the integration of **Sail** (a native Rust Spark Connect server) with **RustFS** (S3-compatible storage) running in a Kubernetes cluster (Kind).

## Prerequisites

- **Docker** and **Docker Compose**.
- **Kind** (Kubernetes in Docker).
- **kubectl**.
- **Python 3.10+** with a configured virtual environment.

## Execution Steps

### 1. Start RustFS

RustFS must be running in Docker so the Kubernetes cluster can reach it.

```bash
docker-compose up -d
```

### 2. Configure Networking (Kind to RustFS)

For the Sail workers in Kubernetes to connect to the RustFS container, we must connect the container to the Kind network and identify its IP:

```bash
# Connect RustFS to the Kind network
docker network connect kind rustsf

# Get the internal IP (usually 172.23.0.3)
docker inspect rustsf -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}'
```

> [!IMPORTANT]
> If the retrieved IP is not `172.23.0.3`, make sure to update the `k8s/sail.yml` file (both in `AWS_ENDPOINT_URL` and the `SAIL_KUBERNETES__WORKER_POD_TEMPLATE`).

### 3. Deploy Sail to Kubernetes

Apply the Kubernetes manifests to start the Sail server and configure credentials.

```bash
# Create the AWS secret (ensure values match RustFS)
kubectl create secret generic aws-creds -n sail \
  --from-literal=credentials="[default]\naws_access_key_id = rustfsadmin\naws_secret_access_key = rustfsadmin" \
  --from-literal=config="[default]\nregion = us-east-1\nendpoint_url = http://172.23.0.3:9000" \
  --dry-run=client -o yaml | kubectl apply -f -

# Apply the Deployment and Service
kubectl apply -f k8s/sail.yml

# Wait for the pod to be ready
kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=sail -n sail --timeout=60s
```

### 4. Establish Port-Forward

To connect the local PySpark client to the Kubernetes server:

```bash
kubectl -n sail port-forward service/sail-spark-server 50051:50051 &
```

### 5. Run the Test (PySpark)

Use the virtual environment to run the test script that writes data to RustFS.

```bash
# Activate virtual environment if necessary
source .venv/bin/activate

# Run the script
python main.py
```

## Result Verification

If the execution is successful, you should see the following output:
```text
Escribiendo en: s3a://testsail/prueba_sensores
Escritura realizada en: s3a://testsail/prueba_sensores
```

You can verify the generated files in your local folder mapped in `docker-compose.yaml` (default `~/data`).

## Troubleshooting

- **Credentials Error**: Verify that the `aws-creds` secret is correctly mounted at `/root/.aws` inside the pod.
- **Network Timeouts**: Ensure `rustsf` is on the `kind` network and that the IP in `sail.yml` matches the container's IP.
- **Workers**: If workers fail to start, check the server logs: `kubectl logs -n sail -l app.kubernetes.io/component=spark-server`.

## Data Governance & Lineage

We have integrated **Marquez** and **OpenLineage** to provide automated tracking of your data.

### 1. Start the Governance Stack

```bash
docker compose -f docker-compose-governance.yaml up -d
```

### 2. Configure Networking (Kind to Marquez)

Like RustFS, Marquez must be reachable by the Sail cluster.

```bash
# Verify Marquez is on the kind network
docker network inspect kind -f '{{range .Containers}}{{.Name}} {{.IPv4Address}}{{println}}{{end}}'
```

> [!IMPORTANT]
> If the Marquez IP is not `172.23.0.4`, update the `OPENLINEAGE__TRANSPORT__URL` in `k8s/sail.yml` and restart the Sail deployment.

### 3. Access the Visualization UI

- **Marquez Web UI**: Open [http://localhost:5005](http://localhost:5005) in your browser.
- **Marquez API**: Available at [http://localhost:5006](http://localhost:5006).

Here you can see the **Job Lineage**, **Datasets**, and **Data Flow** captured from Sail.

## Technical Article

For a deep dive into the architecture and the challenges overcome during this project, read the full article:
- [Modern Data Engineering with Sail and RustFS](file:///Users/josengelmolina/.gemini/antigravity/brain/2a96a557-966b-4e7f-962b-3b03e174b645/article_sail_rustfs_governance.md)

## Cleanup

To stop all services and release resources:

### 1. Stop Kubernetes Services
```bash
kubectl delete -f k8s/sail.yml
kubectl delete secret aws-creds -n sail
```

### 2. Stop Docker Services (Storage & Governance)
```bash
docker compose -f docker-compose.yaml down
docker compose -f docker-compose-governance.yaml down
```
# SailOverRustFS
