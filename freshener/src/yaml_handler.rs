pub fn unpack(manifest: &String) -> Vec<String> {
    /*
        Let's split the manifest using "---"
        as a separator
    */
    let mut split = manifest.split("---");
    let vec = split
        .map(|x| x.to_owned())
        .collect();

    vec
}

