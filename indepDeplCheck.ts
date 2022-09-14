import { join } from 'path';
import readYamlFile from 'read-yaml-file';
import { Pod } from './k8sTypes';

const k8sDirectory = 'k8sManifest';
const filename = 'indepDepl.yaml';
const keywords = ['sidecar', 'ambassador'];

readYamlFile(join(__dirname, k8sDirectory, filename)).then(data => {
    const containers = (data as Pod).spec.containers;
    var microserviceName: string = '';
    var indepDepl: boolean = true;

    containers.forEach(container => {
        
        var foundKeyword: boolean = false;

        keywords.forEach(k => {
            if (container.name.toLowerCase().includes(k)) foundKeyword = true;
        });

        if (!foundKeyword) {
            microserviceName === ''? microserviceName = container.name : indepDepl = false;
        }
        
    });

    if (indepDepl) {
        console.log("[❌] Independent deployability");
    } else console.log("[✅] Independent deployability");
});

/**
 * - avoid unused volumes in deployment (.yaml)
 */
