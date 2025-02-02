# Rust CLI Calculator: Docker Setup and Usage

. This project is a simple rust-based command line calculator which evaluate mathematical expresions. So in this documentation I will guide you through the process of building and running the project inside a docker container.
. Ensure you have docker and git installed within your operating system
. Once you have docker and git install, clone the repository using the command
```
git clone <repository_url>

```

. Once the repository is cloned into your local machine cd into it using:
```
cd <repository_url>

```

. Once inside the cloned repository, run the following build command for a Docker image: 
```
docker build -t rust-cli-calculator

```

. One the image is successfully built, run the command below to execute the rust cli calculator in orderto start the docker container:
```
docker run -it --rm rust-cli-calculator

```

. Once the container has started, you can now go ahead and evaluate any simple mathematical expression

