use std::collections::BTreeMap;

use serde_json::Value as Json;

use crate::{
    Error,
    decode::DecodeCtx,
    model::{self, code},
    raw,
};

pub fn decode_targets(
    raw: Vec<raw::Target>,

    globalvars_out: &mut BTreeMap<code::VarId, code::Variable>,
    globallists_out: &mut BTreeMap<code::ListId, code::List>,

    broadcasts_out: &mut BTreeMap<code::BroadcastId, code::Broadcast>,

    mut ctx: DecodeCtx<'_>,
) -> Result<(Vec<model::Sprite>, model::Stage), Error> {
    let mut sprites = Vec::new();
    let mut stage = None;

    for target in raw {
        if target.is_stage {
            if stage.is_some() {
                return Err(Error::MultipleStages);
            }
            let (scripts, loose_reporters) = super::block::decode_scripts(target.blocks, &ctx)?;
            stage = Some(model::Stage {
                scripts,
                loose_reporters,
                current_backdrop: target.current_costume,
                backdrops: target
                    .costumes
                    .into_iter()
                    .map(|c| decode_costume(c, &mut ctx))
                    .collect::<Result<_, _>>()?,
                sounds: target
                    .sounds
                    .into_iter()
                    .map(|s| decode_sound(s, &mut ctx))
                    .collect::<Result<_, _>>()?,
                volume: target.volume,
            });
            *globalvars_out = target
                .variables
                .into_iter()
                .map(|(id, var)| {
                    (
                        model::code::VarId::new(id),
                        model::code::Variable {
                            name: var.0,
                            value: json_to_value(var.1),
                            cloud: var.2,
                        },
                    )
                })
                .collect();
            *globallists_out = target
                .lists
                .into_iter()
                .map(|(id, list)| {
                    (
                        model::code::ListId::new(id),
                        model::code::List {
                            name: list.0,
                            values: list.1.into_iter().map(json_to_value).collect(),
                        },
                    )
                })
                .collect();
            *broadcasts_out = target
                .broadcasts
                .into_iter()
                .map(|(id, broadcast)| {
                    (
                        model::code::BroadcastId::new(id),
                        model::code::Broadcast { name: broadcast },
                    )
                })
                .collect();
        } else {
            let target_name = target.name.clone();
            let (scripts, loose_reporters) = super::block::decode_scripts(target.blocks, &ctx)?;
            sprites.push(model::Sprite {
                name: target.name,
                scripts,
                loose_reporters,
                x: target.x,
                y: target.y,
                direction: target.direction,
                rotation_style: target
                    .rotation_style
                    .ok_or_else(|| Error::MissingRotationStyle(target_name))
                    .and_then(|s| s.parse())?,
                draggable: target.draggable,
                current_costume: target.current_costume,
                costumes: target
                    .costumes
                    .into_iter()
                    .map(|c| decode_costume(c, &mut ctx))
                    .collect::<Result<_, _>>()?,
                visible: target.visible,
                layer: target.layer_order,
                size: target.size,
                sounds: target
                    .sounds
                    .into_iter()
                    .map(|s| decode_sound(s, &mut ctx))
                    .collect::<Result<_, _>>()?,
                volume: target.volume,
                variables: target
                    .variables
                    .into_iter()
                    .map(|(id, var)| {
                        (
                            model::code::VarId::new(id),
                            model::code::Variable {
                                name: var.0,
                                value: json_to_value(var.1),
                                cloud: var.2,
                            },
                        )
                    })
                    .collect(),
                lists: target
                    .lists
                    .into_iter()
                    .map(|(id, list)| {
                        (
                            model::code::ListId::new(id),
                            model::code::List {
                                name: list.0,
                                values: list.1.into_iter().map(json_to_value).collect(),
                            },
                        )
                    })
                    .collect(),
            })
        }
    }

    stage.ok_or(Error::NoStage).map(|stage| (sprites, stage))
}

fn decode_costume(raw: raw::Costume, ctx: &mut DecodeCtx<'_>) -> Result<model::Costume, Error> {
    use model::costume::CostumeFormat;
    Ok(model::Costume {
        name: raw.name.clone(),
        data_format: match raw.data_format.as_str() {
            "png" => CostumeFormat::Png,
            "jpg" | "jpeg" => CostumeFormat::Jpeg,
            "bmp" => CostumeFormat::Bmp,
            "svg" => CostumeFormat::Svg,
            other => return Err(Error::UnknownCostumeFormat(other.to_owned())),
        },
        rotation_center_x: raw.rotation_center_x,
        rotation_center_y: raw.rotation_center_y,
        bitmap_resolution: raw.bitmap_resolution.unwrap_or(1.0),
        data: ctx.load_asset(&raw.md5ext)?,
    })
}

fn decode_sound(raw: raw::Sound, ctx: &mut DecodeCtx<'_>) -> Result<model::Sound, Error> {
    use model::sound::SoundFormat;
    Ok(model::Sound {
        name: raw.name.clone(),
        data_format: match raw.data_format.as_str() {
            "wav" => SoundFormat::Wav,
            "mp3" => SoundFormat::Mp3,
            other => return Err(Error::UnknownSoundFormat(other.to_owned())),
        },
        data: ctx.load_asset(&raw.md5ext)?,
    })
}

fn json_to_value(v: Json) -> code::Value {
    match v {
        Json::Number(n) => code::Value::Number(n.as_f64().unwrap_or(0.0)),
        Json::String(s) => code::Value::Text(s),
        Json::Bool(b) => code::Value::Boolean(b),
        _ => code::Value::Number(0.0),
    }
}
