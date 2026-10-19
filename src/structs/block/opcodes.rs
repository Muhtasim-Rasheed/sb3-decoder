//! This module defines the various opcodes used in Scratch 3.0 blocks.
//!
//! All opcodes are divided into 9 core categories.
//! **Note:** There are 11 more categories that aren't supported yet in this crate.

use crate::error::DecodeError;

/// This is the [`Opcode`] enum, which encompasses all the different opcodes used in Scratch 3.0
/// blocks.
///
/// You can parse an opcode from a string slice using the [`str::parse`] method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    /// Motion opcodes
    Motion(MotionOpcode),

    /// Looks opcodes
    Looks(LooksOpcode),

    /// Sound opcodes
    Sound(SoundOpcode),

    /// Events opcodes
    Events(EventsOpcode),

    /// Control opcodes
    Control(ControlOpcode),

    /// Sensing opcodes
    Sensing(SensingOpcode),

    /// Operators opcodes
    Operators(OperatorsOpcode),

    /// Data opcodes
    Data(DataOpcode),

    /// My Blocks (Procedures) opcodes
    Procedures(ProceduresOpcode),
}

impl std::str::FromStr for Opcode {
    type Err = DecodeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.splitn(2, '_').collect();
        if parts.len() != 2 {
            return Err(DecodeError::InvalidData(format!(
                "Invalid opcode format: {}",
                input
            )));
        }
        let category = parts[0];
        match category {
            "motion" => Ok(Opcode::Motion(input.parse()?)),
            "looks" => Ok(Opcode::Looks(input.parse()?)),
            "sound" => Ok(Opcode::Sound(input.parse()?)),
            "event" => Ok(Opcode::Events(input.parse()?)),
            "control" => Ok(Opcode::Control(input.parse()?)),
            "sensing" => Ok(Opcode::Sensing(input.parse()?)),
            "operator" => Ok(Opcode::Operators(input.parse()?)),
            "data" => Ok(Opcode::Data(input.parse()?)),
            "procedures" => Ok(Opcode::Procedures(input.parse()?)),
            _ => Err(DecodeError::Unknown(
                "opcode category".to_string(),
                category.to_string(),
            )),
        }
    }
}

/// Motion opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotionOpcode {
    MoveSteps,
    TurnRight,
    TurnLeft,
    GoTo,
    GoToXY,
    GlideTo,
    GlideToXY,
    PointInDirection,
    PointTowards,
    ChangeXBy,
    SetXTo,
    ChangeYBy,
    SetYTo,
    IfOnEdgeBounce,
    SetRotationStyle,
    XPosition,
    YPosition,
    Direction,
}

/// Looks opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LooksOpcode {
    SayForSeconds,
    Say,
    ThinkForSeconds,
    Think,
    SwitchCostumeTo,
    NextCostume,
    SwitchBackdropTo,
    SwitchBackdropToAndWait,
    NextBackdrop,
    ChangeSizeBy,
    SetSizeTo,
    ChangeEffectBy,
    SetEffectTo,
    ClearGraphicEffects,
    Show,
    Hide,
    GoToLayer,
    GoLayers,
    Costume,
    Backdrop,
    Size,
}

/// Sound opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundOpcode {
    PlaySoundUntilDone,
    StartSound,
    StopAllSounds,
    ChangeEffectBy,
    SetEffectTo,
    ClearSoundEffects,
    ChangeVolumeBy,
    SetVolumeTo,
    Volume,
}

/// Events opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventsOpcode {
    WhenFlagClicked,
    WhenKeyPressed,
    WhenThisSpriteClicked,
    WhenStageClicked,
    WhenBackdropSwitchesTo,
    WhenGreaterThan,
    WhenIReceive,
    Broadcast,
    BroadcastAndWait,
}

/// Control opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlOpcode {
    Wait,
    Repeat,
    Forever,
    If,
    IfElse,
    WaitUntil,
    RepeatUntil,
    Stop,
    WhenIStartAsAClone,
    CreateCloneOf,
    DeleteThisClone,
}

/// Sensing opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensingOpcode {
    Touching,
    TouchingColor,
    ColorTouchingColor,
    DistanceTo,
    AskAndWait,
    Answer,
    KeyPressed,
    MouseDown,
    MouseX,
    MouseY,
    SetDragMode,
    Loudness,
    Timer,
    ResetTimer,
    Of,
    Current,
    DaysSince2000,
    Username,
}

/// Operators opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperatorsOpcode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Random,
    GreaterThan,
    LessThan,
    Equals,
    And,
    Or,
    Not,
    Join,
    LetterOf,
    Length,
    Contains,
    Mod,
    Round,
    MathOp,
}

/// Data opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataOpcode {
    Variable,
    SetVariableTo,
    ChangeVariableBy,
    ShowVariable,
    HideVariable,
    List,
    AddToList,
    DeleteOfList,
    DeleteAllOfList,
    InsertInList,
    ReplaceItemInList,
    ItemOfList,
    ItemNumOfList,
    LengthOfList,
    ListContainsItem,
    ShowList,
    HideList,
}

/// My Blocks (Procedures) opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProceduresOpcode {
    Define,
    Call,
    ArgumentStringNumber,
    ArgumentBoolean,
}

macro_rules! impl_from_str {
    ($enum:ty, $prefix:literal { $($opcode:literal => $variant:ident),* $(,)? }) => {
        impl std::str::FromStr for $enum {
            type Err = DecodeError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match input {
                    $(
                        concat!($prefix, "_", $opcode) => Ok(<$enum>::$variant),
                    )*
                    _ => Err(DecodeError::Unknown("opcode".to_string(), input.to_string()))
                }
            }
        }
    };
}

impl_from_str!(MotionOpcode, "motion" {
    "movesteps" => MoveSteps,
    "turnright" => TurnRight,
    "turnleft" => TurnLeft,
    "goto" => GoTo,
    "gotoxy" => GoToXY,
    "glideto" => GlideTo,
    "glidesecstoxy" => GlideToXY,
    "pointindirection" => PointInDirection,
    "pointtowards" => PointTowards,
    "changexby" => ChangeXBy,
    "setx" => SetXTo,
    "changeyby" => ChangeYBy,
    "sety" => SetYTo,
    "ifonedgebounce" => IfOnEdgeBounce,
    "setrotationstyle" => SetRotationStyle,
    "xposition" => XPosition,
    "yposition" => YPosition,
    "direction" => Direction,
});

impl_from_str!(LooksOpcode, "looks" {
    "sayforsecs" => SayForSeconds,
    "say" => Say,
    "thinkforsecs" => ThinkForSeconds,
    "think" => Think,
    "switchcostumeto" => SwitchCostumeTo,
    "nextcostume" => NextCostume,
    "switchbackdropto" => SwitchBackdropTo,
    "switchbackdroptoandwait" => SwitchBackdropToAndWait,
    "nextbackdrop" => NextBackdrop,
    "changesizeby" => ChangeSizeBy,
    "setsizeto" => SetSizeTo,
    "changeeffectby" => ChangeEffectBy,
    "seteffectto" => SetEffectTo,
    "cleargraphiceffects" => ClearGraphicEffects,
    "show" => Show,
    "hide" => Hide,
    "gotofrontback" => GoToLayer,
    "goforwardbackwardlayers" => GoLayers,
    "costumenumbername" => Costume,
    "backdropnumbername" => Backdrop,
    "size" => Size,
});

impl_from_str!(SoundOpcode, "sound" {
    "playuntildone" => PlaySoundUntilDone,
    "play" => StartSound,
    "stopallsounds" => StopAllSounds,
    "changeeffectby" => ChangeEffectBy,
    "seteffectto" => SetEffectTo,
    "cleareffects" => ClearSoundEffects,
    "changevolumeby" => ChangeVolumeBy,
    "setvolumeto" => SetVolumeTo,
    "volume" => Volume,
});

impl_from_str!(EventsOpcode, "event" {
    "whenflagclicked" => WhenFlagClicked,
    "whenkeypressed" => WhenKeyPressed,
    "whenthisspriteclicked" => WhenThisSpriteClicked,
    "whenstageclicked" => WhenStageClicked,
    "whenbackdropswitchesto" => WhenBackdropSwitchesTo,
    "whengreaterthan" => WhenGreaterThan,
    "whenbroadcastreceived" => WhenIReceive,
    "broadcast" => Broadcast,
    "broadcastandwait" => BroadcastAndWait,
});

impl_from_str!(ControlOpcode, "control" {
    "wait" => Wait,
    "repeat" => Repeat,
    "forever" => Forever,
    "if" => If,
    "if_else" => IfElse,
    "wait_until" => WaitUntil,
    "repeat_until" => RepeatUntil,
    "stop" => Stop,
    "start_as_clone" => WhenIStartAsAClone,
    "create_clone_of" => CreateCloneOf,
    "delete_this_clone" => DeleteThisClone,
});

impl_from_str!(SensingOpcode, "sensing" {
    "touchingobject" => Touching,
    "touchingcolor" => TouchingColor,
    "coloristouchingcolor" => ColorTouchingColor,
    "distanceto" => DistanceTo,
    "askandwait" => AskAndWait,
    "answer" => Answer,
    "keypressed" => KeyPressed,
    "mousedown" => MouseDown,
    "mousex" => MouseX,
    "mousey" => MouseY,
    "setdragmode" => SetDragMode,
    "loudness" => Loudness,
    "timer" => Timer,
    "resettimer" => ResetTimer,
    "of" => Of,
    "current" => Current,
    "dayssince2000" => DaysSince2000,
    "username" => Username,
});

impl_from_str!(OperatorsOpcode, "operator" {
    "add" => Add,
    "subtract" => Subtract,
    "multiply" => Multiply,
    "divide" => Divide,
    "random" => Random,
    "gt" => GreaterThan,
    "lt" => LessThan,
    "equals" => Equals,
    "and" => And,
    "or" => Or,
    "not" => Not,
    "join" => Join,
    "letter_of" => LetterOf,
    "length" => Length,
    "contains" => Contains,
    "mod" => Mod,
    "round" => Round,
    "mathop" => MathOp,
});

impl_from_str!(DataOpcode, "data" {
    "variable" => Variable,
    "setvariableto" => SetVariableTo,
    "changevariableby" => ChangeVariableBy,
    "showvariable" => ShowVariable,
    "hidevariable" => HideVariable,
    "listcontent" => List,
    "addtolist" => AddToList,
    "deleteoflist" => DeleteOfList,
    "deletealloflist" => DeleteAllOfList,
    "insertatlist" => InsertInList,
    "replaceitemoflist" => ReplaceItemInList,
    "itemoflist" => ItemOfList,
    "itemnumoflist" => ItemNumOfList,
    "lengthoflist" => LengthOfList,
    "listcontainsitem" => ListContainsItem,
    "showlist" => ShowList,
    "hidelist" => HideList,
});

impl_from_str!(ProceduresOpcode, "procedures" {
    "definition" => Define,
    "call" => Call,
    "argument_reporter_string_number" => ArgumentStringNumber,
    "argument_reporter_boolean" => ArgumentBoolean,
});
