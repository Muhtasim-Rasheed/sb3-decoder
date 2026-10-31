use std::collections::BTreeMap;

use crate::{Error, decode::DecodeCtx, model, raw};

pub fn decode_project(raw: raw::Project, ctx: DecodeCtx<'_>) -> Result<model::Project, Error> {
    let mut global_variables = BTreeMap::new();
    let mut global_lists = BTreeMap::new();
    let mut broadcasts = BTreeMap::new();
    let (sprites, stage) = super::target::decode_targets(
        raw.targets,
        &mut global_variables,
        &mut global_lists,
        &mut broadcasts,
        ctx,
    )?;
    let extensions = raw
        .extensions
        .into_iter()
        .map(|e| {
            use model::project::Extension as E;
            match e.as_str() {
                "pen" => Ok(E::Pen),
                _ => Err(Error::InvalidExtension(e)),
            }
        })
        .collect::<Result<_, _>>()?;
    Ok(model::Project {
        sprites,
        stage,
        global_variables,
        global_lists,
        broadcasts,
        extensions,
    })
}
