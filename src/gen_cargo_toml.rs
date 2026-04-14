use std::collections::HashMap;

use cargo_toml::{Dependency, DependencyDetail, Manifest};

// fn gen_config(extra_deps: Vec<>) {

// }

pub fn gen_cargo(
    chiyo_dep: DependencyDetail,
    extra_deps: HashMap<String, DependencyDetail>,
) -> Manifest {
    let base_manifest = toml::from_str(include_str!("../res/base_crate.toml")).unwrap();
    
    gen_config_from_base(chiyo_dep, extra_deps, base_manifest)
}

fn gen_config_from_base(
    chiyo_dep: DependencyDetail,
    extra_deps: HashMap<String, DependencyDetail>,
    mut base: Manifest,
) -> Manifest {
    let chiyo_dep = Box::new(chiyo_dep);
    base.dependencies
        .insert("chiyocore".into(), Dependency::Detailed(chiyo_dep.clone()));
    base.dependencies.insert(
        "chiyocore-companion".into(),
        Dependency::Detailed(chiyo_dep.clone()),
    );
    base.dependencies
        .insert("chiyo-hal".into(), Dependency::Detailed(chiyo_dep.clone()));

    for (dep_name, dep) in extra_deps {
        base.dependencies
            .insert(dep_name, Dependency::Detailed(Box::new(dep)));
    }

    base
}
