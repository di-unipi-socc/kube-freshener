# k8s Freshener
This tool allows you to find and solve architectural smells inside a microservice environment driven by Kubernetes

## How to run it
To make it run just execute `cargo run` in the project folder. There's the possibility to edit the known-images (aka ignore list) using three basic commands, which are:
- `cargo run list-ignore`: shows the ignore list
- `cargo run add-ignore <name> <image> <kind>`: add an ignore item
- `cargo run delete-ignore <name>`: delete an ignore item given its name

The main `cargo run` allows you to execute the tool that will explore the manifests folder and asap it will return the analysis in terms of which are the smells found and how to solve them.