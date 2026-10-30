use std::collections::{BTreeMap, HashSet};

use serde_json::Value as Json;

use crate::{Error, decode::DecodeCtx, model::code as model, raw};

struct Lowerer<'a, 'b> {
    blocks: &'a BTreeMap<String, raw::Block>,
    ctx: &'a DecodeCtx<'b>,
    loose_reporters: Vec<model::Reporter>,
}

pub fn decode_scripts(
    raw: BTreeMap<String, raw::Block>,
    ctx: &DecodeCtx<'_>,
) -> Result<(Vec<model::Script>, Vec<model::Reporter>), Error> {
    let mut lowerer = Lowerer {
        blocks: &raw,
        ctx,
        loose_reporters: Vec::new(),
    };

    let mut tops: Vec<(&str, &raw::FullBlock)> = raw
        .iter()
        .filter_map(|(id, b)| match b {
            raw::Block::Full(f) if f.top_level => Some((id.as_str(), f)),
            _ => None,
        })
        .collect();

    tops.sort_by(|a, b| {
        a.1.y
            .unwrap_or(0.0)
            .total_cmp(&b.1.y.unwrap_or(0.0))
            .then(a.1.x.unwrap_or(0.0).total_cmp(&b.1.x.unwrap_or(0.0)))
            .then(a.0.cmp(b.0))
    });

    let loose_prims: Vec<(&str, &[Json])> = raw
        .iter()
        .filter_map(|(id, b)| match b {
            raw::Block::Primitive(p) => Some((id.as_str(), p.as_slice())),
            _ => None,
        })
        .collect();

    for (_, p) in loose_prims {
        lowerer.loose_reporters.push(primitive(p)?);
    }

    tops.into_iter()
        .filter_map(|(id, b)| lowerer.script(id, b).transpose())
        .collect::<Result<_, _>>()
        .map(|v| (v, lowerer.loose_reporters))
}

impl<'a, 'b> Lowerer<'a, 'b> {
    fn full(&self, id: &str) -> Result<&raw::FullBlock, Error> {
        match self.blocks.get(id) {
            Some(raw::Block::Full(f)) => Ok(f),
            _ => Err(Error::MissingBlock { id: id.to_string() }),
        }
    }

    fn script(&mut self, id: &str, b: &raw::FullBlock) -> Result<Option<model::Script>, Error> {
        if let Some(hat) = self.hat(b)? {
            return Ok(Some(model::Script {
                hat: Some(hat),
                actions: self.stack(b.next.as_deref(), 0)?,
            }));
        }
        if self.known_action(b, 0)?.is_none()
            && let Some(r) = self.known_reporter(b, 0)?
        {
            // loose reporter, not a script
            self.loose_reporters.push(r);
            return Ok(None);
        }
        Ok(Some(model::Script {
            hat: None,
            actions: self.stack(Some(id), 0)?,
        }))
    }

    fn stack(
        &'a self,
        mut next: Option<&'a str>,
        depth: usize,
    ) -> Result<Vec<model::Action>, Error> {
        if depth > self.ctx.max_depth {
            return Err(Error::DepthExceeded {
                limit: self.ctx.max_depth,
            });
        }
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        while let Some(id) = next {
            if !seen.insert(id) {
                return Err(Error::BlockCycle { id: id.to_string() });
            }
            let b = self.full(id)?;
            out.push(self.action(b, depth)?);
            next = b.next.as_deref();
        }
        Ok(out)
    }

    fn hat(&self, b: &raw::FullBlock) -> Result<Option<model::Hat>, Error> {
        use model::Hat as H;
        Ok(Some(match b.opcode.as_str() {
            "event_whenflagclicked" => H::WhenFlagClicked,
            "event_whenkeypressed" => H::WhenKeyPressed {
                key: field(b, "KEY_OPTION")?.parse()?,
            },
            "event_whenthisspriteclicked" => H::WhenThisSpriteClicked,
            "event_whenstageclicked" => H::WhenStageClicked,
            "event_whenbackdropswitchesto" => H::WhenBackdropSwitchesTo {
                backdrop: field(b, "BACKDROP")?,
            },
            "event_whengreaterthan" => H::WhenGreaterThan {
                loudness_timer: field(b, "WHENGREATERTHANMENU")?.parse()?,
                val: self.input(b, "VALUE", 0)?,
            },
            "event_whenbroadcastreceived" => H::WhenIReceive {
                broadcast: field_id(b, "BROADCAST_OPTION").map(model::BroadcastId::new)?,
            },
            "control_start_as_clone" => H::WhenIStartAsAClone,
            "procedures_definition" => H::DefineProcedure {
                prototype: self.prototype(b)?,
            },
            _ => return Ok(None),
        }))
    }

    fn known_action(
        &self,
        b: &raw::FullBlock,
        depth: usize,
    ) -> Result<Option<model::Action>, Error> {
        macro_rules! i {
            ($name:ident) => {
                self.input(b, stringify!($name), depth)?
            };
        }
        macro_rules! f {
            ($name:ident) => {
                field(b, stringify!($name))?
            };
        }
        macro_rules! f_id {
            ($name:ident, $ty:ident) => {
                field_id(b, stringify!($name)).map(<model::$ty>::new)?
            };
        }
        macro_rules! s {
            ($name:ident) => {
                self.substack(b, stringify!($name), depth)?
            };
        }

        use model::Action as A;
        Ok(Some(match b.opcode.as_str() {
            "motion_movesteps" => A::MoveSteps { steps: i!(STEPS) },
            "motion_turnright" => A::TurnRight { deg: i!(DEGREES) },
            "motion_turnleft" => A::TurnLeft { deg: i!(DEGREES) },
            "motion_goto" => A::GoTo { loc: i!(TO) },
            "motion_gotoxy" => A::GoToXY { x: i!(X), y: i!(Y) },
            "motion_glideto" => A::GlideSecsTo {
                secs: i!(SECS),
                loc: i!(TO),
            },
            "motion_glidesecstoxy" => A::GlideSecsToXY {
                secs: i!(SECS),
                x: i!(X),
                y: i!(Y),
            },
            "motion_pointindirection" => A::PointInDirection { dir: i!(DIRECTION) },
            "motion_pointtowards" => A::PointTowards { loc: i!(TOWARDS) },
            "motion_changexby" => A::ChangeXBy { inc: i!(DX) },
            "motion_setx" => A::SetXTo { x: i!(X) },
            "motion_changeyby" => A::ChangeYBy { inc: i!(DY) },
            "motion_sety" => A::SetYTo { y: i!(Y) },
            "motion_ifonedgebounce" => A::IfOnEdgeBounce,
            "motion_setrotationstyle" => A::SetRotationStyle {
                style: f!(STYLE).parse()?,
            },

            "looks_sayforsecs" => A::SayForSeconds {
                content: i!(MESSAGE),
                secs: i!(SECS),
            },
            "looks_say" => A::Say {
                content: i!(MESSAGE),
            },
            "looks_thinkforsecs" => A::ThinkForSeconds {
                content: i!(MESSAGE),
                secs: i!(SECS),
            },
            "looks_think" => A::Think {
                content: i!(MESSAGE),
            },
            "looks_switchcostumeto" => A::SwitchCostumeTo {
                costume: i!(COSTUME),
            },
            "looks_nextcostume" => A::NextCostume,
            "looks_switchbackdropto" => A::SwitchBackdropTo {
                backdrop: i!(BACKDROP),
            },
            "looks_switchbackdroptoandwait" => A::SwitchBackdropToAndWait {
                backdrop: i!(BACKDROP),
            },
            "looks_nextbackdrop" => A::NextBackdrop,
            "looks_changesizeby" => A::ChangeSizeBy { inc: i!(CHANGE) },
            "looks_setsizeto" => A::SetSizeTo { size: i!(SIZE) },
            "looks_changeeffectby" => A::ChangeGraphicEffectBy {
                effect: f!(EFFECT).parse()?,
                inc: i!(CHANGE),
            },
            "looks_seteffectto" => A::SetGraphicEffectTo {
                effect: f!(EFFECT).parse()?,
                val: i!(VALUE),
            },
            "looks_cleargraphiceffects" => A::ClearGraphicEffects,
            "looks_show" => A::Show,
            "looks_hide" => A::Hide,
            "looks_gotofrontback" => A::GoToLayer {
                front_back: f!(FRONT_BACK).parse()?,
            },
            "looks_goforwardbackwardlayers" => A::GoLayers {
                forward_backward: f!(FORWARD_BACKWARD).parse()?,
                num: i!(NUM),
            },

            "sound_playuntildone" => A::PlaySoundUntilDone {
                sound: i!(SOUND_MENU),
            },
            "sound_play" => A::PlaySound {
                sound: i!(SOUND_MENU),
            },
            "sound_stopallsounds" => A::StopAllSounds,
            "sound_changeeffectby" => A::ChangeSoundEffectBy {
                effect: f!(EFFECT).parse()?,
                inc: i!(VALUE),
            },
            "sound_seteffectto" => A::SetSoundEffectTo {
                effect: f!(EFFECT).parse()?,
                val: i!(VALUE),
            },
            "sound_cleareffects" => A::ClearSoundEffects,
            "sound_changevolumeby" => A::ChangeVolumeBy { inc: i!(VOLUME) },
            "sound_setvolumeto" => A::SetVolumeTo { volume: i!(VOLUME) },

            "event_broadcast" => A::Broadcast {
                broadcast: i!(BROADCAST_INPUT),
            },
            "event_broadcastandwait" => A::BroadcastAndWait {
                broadcast: i!(BROADCAST_INPUT),
            },

            "control_wait" => A::WaitSecs { secs: i!(DURATION) },
            "control_repeat" => A::Repeat {
                times: i!(TIMES),
                body: s!(SUBSTACK),
            },
            "control_forever" => A::Forever { body: s!(SUBSTACK) },
            "control_if" => A::If {
                cond: i!(CONDITION),
                then: s!(SUBSTACK),
            },
            "control_if_else" => A::IfElse {
                cond: i!(CONDITION),
                then: s!(SUBSTACK),
                otherwise: s!(SUBSTACK2),
            },
            "control_wait_until" => A::WaitUntil {
                cond: i!(CONDITION),
            },
            "control_repeat_until" => A::RepeatUntil {
                cond: i!(CONDITION),
                body: s!(SUBSTACK),
            },
            "control_while" => A::While {
                cond: i!(CONDITION),
                body: s!(SUBSTACK),
            },
            "control_stop" => A::Stop {
                stop: f!(STOP_OPTION).parse()?,
            },
            "control_create_clone_of" => A::CreateCloneOf {
                menu: i!(CLONE_OPTION),
            },
            "control_delete_this_clone" => A::DeleteThisClone,

            "sensing_askandwait" => A::AskAndWait {
                content: i!(QUESTION),
            },
            "sensing_setdragmode" => {
                let draggable = f!(DRAG_MODE);
                A::SetDragMode {
                    draggable: match draggable.as_str() {
                        "draggable" => true,
                        "not draggable" => false,
                        _ => return Err(Error::InvalidDraggable(draggable)),
                    },
                }
            }
            "sensing_resettimer" => A::ResetTimer,

            "data_setvariableto" => A::SetVariableTo {
                var: f_id!(VARIABLE, VarId),
                val: i!(VALUE),
            },
            "data_changevariableby" => A::ChangeVariableBy {
                var: f_id!(VARIABLE, VarId),
                inc: i!(VALUE),
            },
            "data_showvariable" => A::ShowVariable {
                var: f_id!(VARIABLE, VarId),
            },
            "data_hidevariable" => A::HideVariable {
                var: f_id!(VARIABLE, VarId),
            },
            "data_addtolist" => A::AddToList {
                list: f_id!(LIST, ListId),
                val: i!(ITEM),
            },
            "data_deleteoflist" => A::DeleteOfList {
                list: f_id!(LIST, ListId),
                index: i!(INDEX),
            },
            "data_deletealloflist" => A::DeleteAllOfList {
                list: f_id!(LIST, ListId),
            },
            "data_insertatlist" => A::InsertAtOfList {
                list: f_id!(LIST, ListId),
                val: i!(ITEM),
                index: i!(INDEX),
            },
            "data_replaceitemoflist" => A::ReplaceItemOfListWith {
                list: f_id!(LIST, ListId),
                index: i!(INDEX),
                val: i!(ITEM),
            },
            "data_showlist" => A::ShowList {
                list: f_id!(LIST, ListId),
            },
            "data_hidelist" => A::HideList {
                list: f_id!(LIST, ListId),
            },

            "procedures_call" => self.call(b, depth)?,

            _ => {
                return Ok(None);
            }
        }))
    }

    fn known_reporter(
        &self,
        b: &raw::FullBlock,
        depth: usize,
    ) -> Result<Option<model::Reporter>, Error> {
        use model::{Reporter as R, blocks::*};

        macro_rules! i {
            ($name:ident) => {
                Box::new(self.input(b, stringify!($name), depth)?)
            };
        }
        macro_rules! f {
            ($name:ident) => {
                field(b, stringify!($name))?
            };
        }

        Ok(Some(match b.opcode.as_str() {
            "motion_goto_menu" | "motion_glideto_menu" => R::LocationMenu {
                menu: f!(TO).parse()?,
            },
            "motion_pointtowards_menu" => R::LocationMenu {
                menu: f!(TOWARDS).parse()?,
            },
            "motion_xposition" => R::XPosition,
            "motion_yposition" => R::YPosition,
            "motion_direction" => R::Direction,

            "looks_costume" => R::CostumeMenu {
                costume: f!(COSTUME),
            },
            "looks_backdrops" => R::BackdropMenu {
                backdrop: f!(BACKDROP),
            },
            "looks_costumenumbername" => R::CurrentCostume {
                number_name: f!(NUMBER_NAME).parse()?,
            },
            "looks_backdropnumbername" => R::CurrentBackdrop {
                number_name: f!(NUMBER_NAME).parse()?,
            },
            "looks_size" => R::Size,

            "sound_sounds_menu" => R::SoundMenu {
                sound: f!(SOUND_MENU),
            },
            "sound_volume" => R::Volume,

            "event_broadcast_menu" => R::Broadcast {
                broadcast: field_id(b, "BROADCAST_OPTION").map(model::BroadcastId::new)?,
            },

            "control_create_clone_of_menu" => R::CloneOfMenu {
                menu: match f!(CLONE_OPTION).as_str() {
                    "_myself_" => CloneOfMenu::Myself,
                    s => CloneOfMenu::Sprite(s.to_owned()),
                },
            },

            "sensing_touchingobject" => R::TouchingObject {
                object: i!(TOUCHINGOBJECTMENU),
            },
            "sensing_touchingobjectmenu" => R::TouchingObjectMenu {
                menu: match f!(TOUCHINGOBJECTMENU).as_str() {
                    "_mouse_" => TouchingObjectMenu::MousePointer,
                    "_edge_" => TouchingObjectMenu::Edge,
                    s => TouchingObjectMenu::Sprite(s.to_owned()),
                },
            },
            "sensing_touchingcolor" => R::TouchingColor { color: i!(COLOR) },
            "sensing_coloristouchingcolor" => R::ColorTouchingColor {
                a: i!(COLOR),
                b: i!(COLOR2),
            },
            "sensing_distanceto" => R::DistanceToObject {
                object: i!(DISTANCETOMENU),
            },
            "sensing_distancetomenu" => R::DistanceToMenu {
                menu: match f!(DISTANCETOMENU).as_str() {
                    "_mouse_" => DistanceToMenu::MousePointer,
                    s => DistanceToMenu::Sprite(s.to_owned()),
                },
            },
            "sensing_of" => R::PropertyOfObject {
                property: f!(PROPERTY).parse()?,
                object: i!(OBJECT),
            },
            "sensing_of_object_menu" => R::OfObjectMenu {
                menu: match f!(OBJECT).as_str() {
                    "_stage_" => OfObjectMenu::Stage,
                    s => OfObjectMenu::Sprite(s.to_owned()),
                },
            },
            "sensing_answer" => R::Answer,
            "sensing_keypressed" => R::KeyPressed {
                key: i!(KEY_OPTION),
            },
            "sensing_keyoptions" => R::KeyMenu {
                menu: f!(KEY_OPTION).parse()?,
            },
            "sensing_mousedown" => R::MouseDown,
            "sensing_mousex" => R::MouseX,
            "sensing_mousey" => R::MouseY,
            "sensing_loudness" => R::Loudness,
            "sensing_timer" => R::Timer,
            "sensing_current" => R::Current {
                current: f!(CURRENTMENU).parse()?,
            },
            "sensing_dayssince2000" => R::DaysSince2000,
            "sensing_username" => R::Username,

            "operator_add" => R::Add {
                a: i!(NUM1),
                b: i!(NUM2),
            },
            "operator_subtract" => R::Sub {
                a: i!(NUM1),
                b: i!(NUM2),
            },
            "operator_multiply" => R::Mul {
                a: i!(NUM1),
                b: i!(NUM2),
            },
            "operator_divide" => R::Div {
                a: i!(NUM1),
                b: i!(NUM2),
            },
            "operator_random" => R::PickRandom {
                min: i!(FROM),
                max: i!(TO),
            },
            "operator_gt" => R::Gt {
                a: i!(OPERAND1),
                b: i!(OPERAND2),
            },
            "operator_lt" => R::Lt {
                a: i!(OPERAND1),
                b: i!(OPERAND2),
            },
            "operator_eq" => R::Eq {
                a: i!(OPERAND1),
                b: i!(OPERAND2),
            },
            "operator_and" => R::And {
                a: i!(OPERAND1),
                b: i!(OPERAND2),
            },
            "operator_or" => R::Or {
                a: i!(OPERAND1),
                b: i!(OPERAND2),
            },
            "operator_not" => R::Not { x: i!(OPERAND) },
            "operator_join" => R::Join {
                a: i!(STRING1),
                b: i!(STRING2),
            },
            "operator_letter_of" => R::LetterOf {
                index: i!(LETTER),
                text: i!(STRING),
            },
            "operator_length" => R::LengthOfText { text: i!(STRING) },
            "operator_contains" => R::TextContains {
                haystack: i!(STRING1),
                needle: i!(STRING2),
            },
            "operator_mod" => R::Mod {
                a: i!(NUM1),
                b: i!(NUM2),
            },
            "operator_round" => R::Round { x: i!(NUM) },
            "operator_mathop" => R::MathFunc {
                func: f!(OPERATOR).parse()?,
                x: i!(NUM),
            },

            "data_variable" => R::Variable {
                var: field_id(b, "VARIABLE").map(model::VarId::new)?,
            },
            "data_listcontents" => R::List {
                list: field_id(b, "LIST").map(model::ListId::new)?,
            },
            "argument_reporter_string_number" | "argument_reporter_boolean" => {
                R::Argument { name: f!(VALUE) }
            }

            _ => return Ok(None),
        }))
    }

    fn action(&self, b: &raw::FullBlock, depth: usize) -> Result<model::Action, Error> {
        if let Some(a) = self.known_action(b, depth)? {
            return Ok(a);
        }
        if self.ctx.allow_unknown_blocks {
            Ok(model::Action::Unknown(self.unknown(b, depth)?))
        } else {
            Err(Error::UnknownOpcode(b.opcode.clone()))
        }
    }

    fn reporter(&self, b: &raw::FullBlock, depth: usize) -> Result<model::Reporter, Error> {
        if let Some(r) = self.known_reporter(b, depth)? {
            return Ok(r);
        }
        if self.ctx.allow_unknown_blocks {
            Ok(model::Reporter::Unknown(self.unknown(b, depth)?))
        } else {
            Err(Error::UnknownOpcode(b.opcode.clone()))
        }
    }

    fn unknown(
        &self,
        b: &raw::FullBlock,
        depth: usize,
    ) -> Result<model::blocks::UnknownBlock, Error> {
        use model::blocks::UnknownInput as U;
        if depth > self.ctx.max_depth {
            return Err(Error::DepthExceeded {
                limit: self.ctx.max_depth,
            });
        }
        let mut inputs = Vec::new();
        for (name, raw::Input(_, value, _)) in &b.inputs {
            let input = match value {
                raw::InputValue::Primitive(p) => U::Reporter(primitive(p)?),
                raw::InputValue::Block(id) if name.starts_with("SUBSTACK") => {
                    U::Stack(self.stack(Some(id), depth + 1)?)
                }
                raw::InputValue::Block(id) => {
                    U::Reporter(self.reporter(self.full(id)?, depth + 1)?)
                }
                raw::InputValue::Empty => continue,
            };
            inputs.push((name.clone(), input));
        }
        let fields = b
            .fields
            .iter()
            .map(|(k, f)| {
                (
                    k.clone(),
                    model::blocks::UnknownField {
                        value: f.0.clone(),
                        id: f.1.clone(),
                    },
                )
            })
            .collect();
        Ok(model::blocks::UnknownBlock {
            opcode: b.opcode.clone(),
            inputs,
            fields,
        })
    }

    fn substack(
        &self,
        b: &raw::FullBlock,
        name: &str,
        depth: usize,
    ) -> Result<Vec<model::Action>, Error> {
        match b.inputs.get(name) {
            Some(raw::Input(_, raw::InputValue::Block(id), _)) => self.stack(Some(id), depth + 1),
            _ => Ok(Vec::new()), // empty C-block
        }
    }

    fn input(
        &self,
        b: &raw::FullBlock,
        name: &str,
        depth: usize,
    ) -> Result<model::Reporter, Error> {
        if depth > self.ctx.max_depth {
            return Err(Error::DepthExceeded {
                limit: self.ctx.max_depth,
            });
        }
        let Some(raw::Input(_, value, _)) = b.inputs.get(name) else {
            return Ok(model::Reporter::Empty);
        };
        match value {
            raw::InputValue::Block(id) => self.reporter(self.full(id)?, depth + 1),
            raw::InputValue::Primitive(p) => primitive(p),
            raw::InputValue::Empty => Ok(model::Reporter::Empty),
        }
    }

    fn call(&self, b: &raw::FullBlock, depth: usize) -> Result<model::Action, Error> {
        let m = b.mutation.as_ref().ok_or(Error::MissingMutation)?;
        let proccode = m.proccode.clone().ok_or(Error::MissingMutation)?;
        let ids: Vec<String> = json_list(m.argumentids.as_deref())?;
        let args = ids
            .iter()
            .map(|id| self.input(b, id, depth))
            .collect::<Result<_, _>>()?;
        Ok(model::Action::CallProcedure { proccode, args })
    }

    fn prototype(&self, def: &raw::FullBlock) -> Result<model::blocks::ProcPrototype, Error> {
        let Some(raw::Input(_, raw::InputValue::Block(pid), _)) = def.inputs.get("custom_block")
        else {
            return Err(Error::MissingMutation);
        };
        let m = self
            .full(pid)?
            .mutation
            .as_ref()
            .ok_or(Error::MissingMutation)?;
        let proccode = m.proccode.clone().ok_or(Error::MissingMutation)?;

        let ids: Vec<String> = json_list(m.argumentids.as_deref())?;
        let names: Vec<String> = json_list(m.argumentnames.as_deref())?;
        let defaults: Vec<Json> = json_list(m.argumentdefaults.as_deref())?;
        if names.len() != ids.len() {
            return Err(Error::InvalidMutation);
        }

        let kinds = proccode.split_whitespace().filter_map(|t| match t {
            "%s" | "%n" => Some(model::blocks::ParamKind::StringNumber),
            "%b" => Some(model::blocks::ParamKind::Boolean),
            _ => None,
        });

        let params = ids
            .into_iter()
            .zip(names)
            .zip(kinds)
            .enumerate()
            .map(|(i, ((id, name), kind))| {
                let d = defaults.get(i);
                let default = match (kind, d) {
                    (model::blocks::ParamKind::Boolean, Some(Json::String(s))) => {
                        model::Value::Boolean(s == "true")
                    }
                    (model::blocks::ParamKind::Boolean, Some(Json::Bool(b))) => {
                        model::Value::Boolean(*b)
                    }
                    (_, Some(v)) => json_to_value(v),
                    (_, None) => model::Value::Text(String::new()),
                };
                model::blocks::Param {
                    id: model::blocks::ArgId::new(id),
                    name,
                    kind,
                    default,
                }
            })
            .collect();

        Ok(model::blocks::ProcPrototype {
            proccode,
            params,
            warp: m.warp.as_deref() == Some("true"),
        })
    }
}

impl std::str::FromStr for model::blocks::KeyField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use model::blocks::KeyField as K;
        Ok(match s {
            "space" => K::Space,
            "up arrow" => K::UpArrow,
            "down arrow" => K::DownArrow,
            "right arrow" => K::RightArrow,
            "left arrow" => K::LeftArrow,
            "any" => K::Any,
            _ => {
                let mut chars = s.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) if c.is_ascii_lowercase() => K::Letter(c),
                    (Some(c), None) if c.is_ascii_digit() => K::Digit(c as u8 - b'0'),
                    _ => K::Other(s.to_owned()),
                }
            }
        })
    }
}

impl std::str::FromStr for model::blocks::LoudnessTimerField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use model::blocks::LoudnessTimerField as LT;

        match s {
            "LOUDNESS" => Ok(LT::Loudness),
            "TIMER" => Ok(LT::Timer),
            _ => Err(Error::InvalidLoudnessTimer(s.to_string())),
        }
    }
}

impl std::str::FromStr for model::blocks::GraphicEffectField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use model::blocks::GraphicEffectField as G;

        match s {
            "COLOR" => Ok(G::Color),
            "FISHEYE" => Ok(G::Fisheye),
            "WHIRL" => Ok(G::Whirl),
            "PIXELATE" => Ok(G::Pixelate),
            "MOSAIC" => Ok(G::Mosaic),
            "BRIGHTNESS" => Ok(G::Brightness),
            "GHOST" => Ok(G::Ghost),
            _ => Err(Error::InvalidGraphicEffect(s.to_string())),
        }
    }
}

impl std::str::FromStr for model::blocks::FrontBackField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use model::blocks::FrontBackField as FB;

        match s {
            "front" => Ok(FB::Front),
            "back" => Ok(FB::Back),
            _ => Err(Error::InvalidFrontBack(s.to_string())),
        }
    }
}

impl std::str::FromStr for model::blocks::ForwardBackwardField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use model::blocks::ForwardBackwardField as FB;

        match s {
            "forward" => Ok(FB::Forward),
            "backward" => Ok(FB::Backward),
            _ => Err(Error::InvalidForwardBackward(s.to_string())),
        }
    }
}

impl std::str::FromStr for model::blocks::SoundEffectField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use model::blocks::SoundEffectField as S;

        match s {
            "PITCH" => Ok(S::Pitch),
            "PAN" => Ok(S::PanLeftRight),
            _ => Err(Error::InvalidSoundEffect(s.to_string())),
        }
    }
}

impl std::str::FromStr for model::blocks::StopField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use model::blocks::StopField as S;

        match s {
            "all" => Ok(S::All),
            "this script" => Ok(S::ThisScript),
            "other scripts in sprite" => Ok(S::OtherScriptsInSprite),
            _ => Err(Error::InvalidStop(s.to_string())),
        }
    }
}

impl std::str::FromStr for model::blocks::LocationMenu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use model::blocks::LocationMenu as L;

        Ok(match s {
            "_mouse_" => L::MousePointer,
            "_random_" => L::Random,
            _ => L::Sprite(s.to_string()),
        })
    }
}

impl std::str::FromStr for model::blocks::NumberNameField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use model::blocks::NumberNameField as N;

        match s {
            "number" => Ok(N::Number),
            "name" => Ok(N::Name),
            _ => Err(Error::InvalidNumberName(s.to_string())),
        }
    }
}

impl std::str::FromStr for model::blocks::PropertyField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use model::blocks::PropertyField as P;

        Ok(match s {
            "x position" => P::XPosition,
            "y position" => P::YPosition,
            "direction" => P::Direction,
            "costume #" => P::CostumeNumber,
            "costume name" => P::CostumeName,
            "size" => P::Size,
            "volume" => P::Volume,
            "backdrop #" => P::BackdropNumber,
            "backdrop name" => P::BackdropName,
            other => P::Variable(other.to_owned()),
        })
    }
}

impl std::str::FromStr for model::blocks::CurrentField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        use model::blocks::CurrentField as C;

        match s {
            "YEAR" => Ok(C::Year),
            "MONTH" => Ok(C::Month),
            "DATE" => Ok(C::Date),
            "DAYOFWEEK" => Ok(C::DayOfWeek),
            "HOUR" => Ok(C::Hour),
            "MINUTE" => Ok(C::Minute),
            "SECOND" => Ok(C::Second),
            _ => Err(Error::InvalidCurrent(s.to_string())),
        }
    }
}

impl std::str::FromStr for model::blocks::MathFuncField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use model::blocks::MathFuncField as M;

        match s {
            "abs" => Ok(M::Abs),
            "floor" => Ok(M::Floor),
            "ceiling" => Ok(M::Ceiling),
            "sqrt" => Ok(M::Sqrt),
            "sin" => Ok(M::Sin),
            "cos" => Ok(M::Cos),
            "tan" => Ok(M::Tan),
            "asin" => Ok(M::Asin),
            "acos" => Ok(M::Acos),
            "atan" => Ok(M::Atan),
            "ln" => Ok(M::Ln),
            "log" => Ok(M::Log),
            "e ^" => Ok(M::EToPower),
            "10 ^" => Ok(M::TenToPower),
            _ => Err(Error::InvalidMathFunc(s.to_string())),
        }
    }
}

fn field(b: &raw::FullBlock, name: &str) -> Result<String, Error> {
    b.fields
        .get(name)
        .map(|v| v.0.clone())
        .ok_or_else(|| Error::MissingField {
            field: name.to_string(),
        })
}

fn field_id(b: &raw::FullBlock, name: &str) -> Result<String, Error> {
    b.fields
        .get(name)
        .and_then(|v| v.1.clone())
        .ok_or_else(|| Error::MissingIDOfField {
            field: name.to_string(),
        })
}

fn json_list<T: serde::de::DeserializeOwned>(s: Option<&str>) -> Result<Vec<T>, Error> {
    serde_json::from_str(s.unwrap_or("[]")).map_err(Error::json)
}

fn json_to_value(v: &Json) -> model::Value {
    match v {
        Json::Number(n) => model::Value::Number(n.as_f64().unwrap_or(0.0)),
        Json::String(s) => model::Value::Text(s.clone()),
        Json::Bool(b) => model::Value::Boolean(*b),
        _ => model::Value::Number(0.0),
    }
}

fn primitive(p: &[Json]) -> Result<model::Reporter, Error> {
    use model::Reporter as R;

    let kind = p
        .first()
        .and_then(Json::as_u64)
        .ok_or(Error::InvalidPrimitive("missing type code"))?;
    let value = p.get(1).ok_or(Error::InvalidPrimitive("missing value"))?;

    Ok(match kind {
        4 => R::Number {
            num: number(value)?,
        },
        5 => R::PosNumber {
            num: number(value)?,
        },
        6 => R::PosInteger {
            num: number(value)? as u64,
        },
        7 => R::Integer {
            num: number(value)? as i64,
        },
        8 => R::Angle {
            num: number(value)?,
        },
        9 => {
            let (r, g, b) = color(value)?;
            R::Color { r, g, b }
        }
        10 => R::Text { text: text(value)? },
        11 => R::Broadcast {
            broadcast: model::BroadcastId::new(id(p)?),
        },
        12 => R::Variable {
            var: model::VarId::new(id(p)?),
        },
        13 => R::List {
            list: model::ListId::new(id(p)?),
        },
        _ => return Err(Error::InvalidPrimitive("unknown primitive type code")),
    })
}

fn number(v: &Json) -> Result<f64, Error> {
    let n = match v {
        Json::Number(n) => n.as_f64(),
        Json::String(s) if s.trim().is_empty() => Some(0.0),
        Json::String(s) => s.trim().parse::<f64>().ok(),
        _ => None,
    };

    n.filter(|n| n.is_finite())
        .ok_or(Error::InvalidPrimitive("invalid number"))
}

fn text(v: &Json) -> Result<String, Error> {
    match v {
        Json::String(s) => Ok(s.clone()),
        Json::Number(n) => Ok(n.to_string()),
        _ => Err(Error::InvalidPrimitive("invalid text")),
    }
}

fn color(v: &Json) -> Result<(u8, u8, u8), Error> {
    let bad = || Error::InvalidPrimitive("invalid color");
    let s = v.as_str().ok_or_else(bad)?;
    let hex = s.strip_prefix('#').unwrap_or(s);
    if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(bad());
    }
    let ch = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| bad());
    Ok((ch(0)?, ch(2)?, ch(4)?))
}

fn id(p: &[Json]) -> Result<&str, Error> {
    p.get(2)
        .and_then(Json::as_str)
        .ok_or(Error::InvalidPrimitive("missing id"))
}
