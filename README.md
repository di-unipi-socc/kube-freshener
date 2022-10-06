# k8s Freshener
This tool allows you to find and solve architectural smells inside a microservice environment driven by Kubernetes

## How to run it
To make it run just execute `cargo run` in the project folder. There's the possibility to edit the known-images (aka ignore list) using three basic commands, which are:
- `cargo run list-known-images`: shows the well-known docker images
- `cargo run list-manifest-ignore`: shows the manifests file the tool have to ignore
- `cargo run add-known-image <name> <image> <kind>`: add a known image with kind in {sidecar, mr}
- `cargo run add-manifest-ignore <name>`: add a manifest to ignore
- `cargo run delete-known-image <name>`: delete a known docker image
- `cargo run delete-manifest-ignore <name>`: stop ignoring a manifest previously added

The main `cargo run` allows you to execute the tool that will explore the manifests folder and asap it will return the analysis in terms of which are the smells found and how to solve them.

![](https://github.com/di-unipi-socc/k8s-freshener/blob/main/imgs/screen.png)