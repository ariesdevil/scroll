use crate::types::ProverType;
use halo2_proofs::{halo2curves::bn256::Bn256, poly::kzg::commitment::ParamsKZG};
use std::{
    collections::BTreeMap,
    sync::{Arc, OnceLock},
};

static PARAMS_MAP: OnceLock<Arc<BTreeMap<u32, ParamsKZG<Bn256>>>> = OnceLock::new();

pub fn get_params_map_instance<'a, F>(load_params_func: F) -> &'a BTreeMap<u32, ParamsKZG<Bn256>>
where
    F: FnOnce() -> BTreeMap<u32, ParamsKZG<Bn256>>,
{
    PARAMS_MAP.get_or_init(|| {
        let params_map = load_params_func();
        Arc::new(params_map)
    })
}

pub fn get_degrees<F>(prover_types: &std::collections::HashSet<ProverType>, f: F) -> Vec<u32>
where
    F: FnMut(&ProverType) -> Vec<u32>,
{
    prover_types
        .iter()
        .flat_map(f)
        .collect::<std::collections::HashSet<u32>>()
        .into_iter()
        .collect()
}
