# KubeFreshener

## About

`KubeFreshener` is a prototype tool enabling you to discover microservices' [architectural smells](http://dx.doi.org/10.1007/s00450-019-00407-8) by analyzing their deployment in Kubernetes. 
The methodology implemented by `KubeFreshener` has been first presented in the paper:
> J. Soldani, M. Marinò, A. Brogi. "Semi-automated Smell Resolution in Kubernetes Deployed Microservices", CLOSER 2023 [accepted for publication].

## Running KubeFreshener
`KubeFreshener` can be configured to analyze a microservices application' deployment in Kubernetes by placing the manifest files specifying such deployment in a newly created `manifests` folder. 

`KubeFreshener` can be further configured by editing the available `config.yaml` file to specify 
1. `invoked_services` - list of microservices that are invoked by other microservices
2. `ignore_smells` - list of architectural smells that should not be checked on given microservices.

Once all the configuration has been provided, `KubeFreshener` can be run by executing the command 
```cargo run analyze [-s]``` 
which runs the analysis and returns an output like that below. If the option `-s` is set, `KubeFreshener` will also update the files in the `manifest` by providing the *refactoring templates* to be completed to resolve the occurrence of identified smells.
![](data/figures/screen.png)

## Examples
The necessary inputs (config and manifest files) for running examples of analyses are available in the [data/examples](data/examples) folder, together with the generated refactoring templates. 