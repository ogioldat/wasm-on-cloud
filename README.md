# **WASM on Cloud**  

Bachelor's Project repo.

A Kubernetes cluster running native WebAssembly workloads (this means without docker runtime).

## **System Requirements**  

Operating System: Linux, macOS, or Windows  

## **Technology Stack**  

- `Kubernetes`  
- `Docker`  
- `WebAssembly` and `WebAssembly System Interface`  
- `k3d` – a lightweight Kubernetes distribution that allows creating more than one node, unlike Docker for Desktop  
- `Rust`  
- `spin` – a tool providing an SDK (Software Development Kit) for WebAssembly application development  

## **Required Tools**  

1. **`Docker for Desktop`**  
   1. [Installation](https://www.docker.com/products/docker-desktop/)  
   2. Select the following configuration options:  

   <img src="./docker-for-desktop-opts.png"/>  

2. **`k3d`** – [Installation](https://k3d.io/v5.6.0/#releases)  
3. **`Rust`**  
   1. [Installation](https://www.rust-lang.org/tools/install)  
   2. Add WASI compilation target:  
      ```bash
      rustup target add wasm32-unknown-unknown
      ```
4. **HTTP Client**, e.g., `curl`  

## **Application Architecture**  

The `spin` tool enables the creation of application components running in WebAssembly. In this project, the listed tools facilitate handling HTTP requests and listening to events sent by a Redis database.  

![image](https://github.com/ogioldat/wasm-on-cloud/assets/46226715/8d3642f3-757d-4982-89eb-61fd02d6a6ac)  

- **`Restaurant`** – HTTP component responsible for:  
  - Returning a list of menu items  
  - Placing an order  
  - Checking the order status  

- **`Kitchen`** – listens to Redis events:  
  - When receiving an event for a new menu order, it marks the order as ready after a specified delay  

## **Cluster Architecture**  

![image](https://github.com/ogioldat/wasm-on-cloud/assets/46226715/639b7c65-e342-476d-985c-ded2cfce47e4)  

The cluster consists of **two nodes**:  

- **`k3d-bachelors-project-server-0`** – standard node with a default runtime, hosting the Redis database  
- **`k3d-bachelors-project-worker-0`** – configured to run applications in WebAssembly format  

## **Project Structure**  

- **`k8s`** – cluster configuration files  
  - `auth.yaml` – administrator account configuration  
  - `ingress.yaml` – ingress and cluster routing configuration  
  - `k3d.yaml` – `k3d` tool configuration  
  - `redis.yaml` – Redis database configuration  
  - `runtime.yaml` – `RuntimeClass` configuration for the WebAssembly runtime  

- **`kitchen`** – `kitchen` service files  
  - `src` – application source files  
  - `Cargo.lock` – dependencies and version tracking  
  - `Cargo.toml` – Rust configuration file  
  - `deploy.yaml` – Kubernetes deployment configuration  
  - `spin.toml` – `spin` tool configuration ([more info](https://developer.fermyon.com/spin/v2/writing-apps))  

- **`restaurant`** – follows the same structure as `kitchen`  

- **`dashboard.sh`** – a helper script for launching the Kubernetes dashboard  

## **Cluster Setup from Scratch**  

Assuming all required tools are installed, the following steps will create a Kubernetes cluster:  

1. **Create a cluster with a network load balancer**  
   ```bash
   k3d cluster create bachelors-project --port "8088:80@loadbalancer"
   ```  

2. **Create a worker node for running WebAssembly applications**  
   ```bash
   k3d node create bachelors-project-worker -c bachelors-project --image="ghcr.io/deislabs/containerd-wasm-shims/examples/k3d:latest"
   ```  

3. **Label the worker node**  
   ```bash
   kubectl label nodes k3d-bachelors-project-worker-0 wasm=yes spin=yes
   ```  

4. **Apply cluster configurations**  
   ```bash
   kubectl apply -Rf k8s  # Applies all configurations in the `k8s` directory
   kubectl apply -f kitchen/deploy.yaml  
   kubectl apply -f restaurant/deploy.yaml  
   ```  

**The application accepts HTTP requests at:**  
[http://localhost:8088](http://localhost:8088)  

## **Supported API Endpoints**  

1. Retrieve the menu list:  
   ```http
   GET http://localhost:8088/menu
   ```  

2. Place an order (e.g., pizza):  
   ```http
   POST http://localhost:8088/order/PIZZA
   ```  

3. Check the status of an order (e.g., pizza):  
   ```http
   GET http://localhost:8088/order/PIZZA
   ```  

Let me know if you need further refinements!
