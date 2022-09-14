type Container = {
    name: string;
    image: string;
}

type Spec = {
    initContainers: Object[][];
    containers: Container[];
    volumes: Object[][];
}

type Pod = {
    apiVersion: string;
    kind: string;
    metadata: string;
    spec: Spec;
}

export { Container, Spec, Pod };