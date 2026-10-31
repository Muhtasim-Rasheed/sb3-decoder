use crate::{macros::string_id, model::sprite::RotationStyle, prelude::*};

/// The representation of a Scratch script with maybe one hat block and many action blocks.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Script {
    pub hat: Option<Hat>,
    pub actions: Vec<Action>,
}

/// The representation of a Scratch hat block.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Hat {
    // Events
    WhenFlagClicked,
    WhenKeyPressed { key: KeyField },
    WhenThisSpriteClicked,
    WhenStageClicked,
    WhenBackdropSwitchesTo { backdrop: String },

    WhenGreaterThan { loudness_timer: LoudnessTimerField, val: Reporter },

    WhenIReceive { broadcast: code::BroadcastId },

    // Control
    WhenIStartAsAClone,

    // Procedures
    DefineProcedure { prototype: ProcPrototype },
}

/// The representation of a Scratch action block.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Action {
    // Motion
    MoveSteps { steps: Reporter },
    TurnRight { deg: Reporter },
    TurnLeft { deg: Reporter },

    GoTo { loc: Reporter },
    GoToXY { x: Reporter, y: Reporter },
    GlideSecsTo { secs: Reporter, loc: Reporter },
    GlideSecsToXY { secs: Reporter, x: Reporter, y: Reporter },

    PointInDirection { dir: Reporter },
    PointTowards { loc: Reporter },

    ChangeXBy { inc: Reporter },
    SetXTo { x: Reporter },
    ChangeYBy { inc: Reporter },
    SetYTo { y: Reporter },

    IfOnEdgeBounce,

    SetRotationStyle { style: RotationStyle },

    // Looks
    SayForSeconds { content: Reporter, secs: Reporter },
    Say { content: Reporter },
    ThinkForSeconds { content: Reporter, secs: Reporter },
    Think { content: Reporter },

    SwitchCostumeTo { costume: Reporter },
    NextCostume,
    SwitchBackdropTo { backdrop: Reporter },
    SwitchBackdropToAndWait { backdrop: Reporter },
    NextBackdrop,

    ChangeSizeBy { inc: Reporter },
    SetSizeTo { size: Reporter },

    ChangeGraphicEffectBy { effect: GraphicEffectField, inc: Reporter },
    SetGraphicEffectTo { effect: GraphicEffectField, val: Reporter },
    ClearGraphicEffects,

    Show,
    Hide,

    GoToLayer { front_back: FrontBackField },
    GoLayers { forward_backward: ForwardBackwardField, num: Reporter },

    // Sound
    PlaySoundUntilDone { sound: Reporter },
    PlaySound { sound: Reporter },
    StopAllSounds,

    ChangeSoundEffectBy { effect: SoundEffectField, inc: Reporter },
    SetSoundEffectTo { effect: SoundEffectField, val: Reporter },
    ClearSoundEffects,

    ChangeVolumeBy { inc: Reporter },
    SetVolumeTo { volume: Reporter },

    // Events
    Broadcast { broadcast: Reporter },
    BroadcastAndWait { broadcast: Reporter },

    // Control
    WaitSecs { secs: Reporter },

    Repeat { times: Reporter, body: Vec<Action> },
    Forever { body: Vec<Action> },

    If { cond: Reporter, then: Vec<Action> },
    IfElse { cond: Reporter, then: Vec<Action>, otherwise: Vec<Action> },
    WaitUntil { cond: Reporter },
    RepeatUntil { cond: Reporter, body: Vec<Action> },
    While { cond: Reporter, body: Vec<Action> },

    Stop { stop: StopField },

    CreateCloneOf { menu: Reporter },
    DeleteThisClone,

    // Sensing
    AskAndWait { content: Reporter },

    SetDragMode { draggable: bool },

    ResetTimer,

    // Data
    SetVariableTo { var: code::VarId, val: Reporter },
    ChangeVariableBy { var: code::VarId, inc: Reporter },
    ShowVariable { var: code::VarId },
    HideVariable { var: code::VarId },

    AddToList { list: code::ListId, val: Reporter },

    DeleteOfList { list: code::ListId, index: Reporter },
    DeleteAllOfList { list: code::ListId },
    InsertAtOfList { list: code::ListId, val: Reporter, index: Reporter },
    ReplaceItemOfListWith { list: code::ListId, index: Reporter, val: Reporter },

    ShowList { list: code::ListId },
    HideList { list: code::ListId },

    // Procedures
    CallProcedure { proccode: String, args: Vec<Reporter> },

    // Unknown
    Unknown(UnknownBlock),
}

/// The representation of a Scratch reporter block.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Reporter {
    // Primitives and Empty
    Number { num: f64 },                        // 4
    PosNumber { num: f64 },                     // 5
    PosInteger { num: u64 },                    // 6
    Integer { num: i64 },                       // 7
    Angle { num: f64 },                         // 8
    Color { r: u8, g: u8, b: u8 },              // 9
    Text { text: String },                      // 10
    Broadcast { broadcast: code::BroadcastId }, // 11
    Variable { var: code::VarId },              // 12
    List { list: code::ListId },                // 13
    Empty,

    // Motion
    LocationMenu { menu: LocationMenu },

    XPosition,
    YPosition,
    Direction,

    // Looks
    CostumeMenu { costume: String },
    BackdropMenu { backdrop: String },

    CurrentCostume { number_name: NumberNameField },
    CurrentBackdrop { number_name: NumberNameField },
    Size,

    // Sound
    SoundMenu { sound: String },

    Volume,

    // Control
    CloneOfMenu { menu: CloneOfMenu },

    // Sensing
    KeyMenu { menu: KeyField },
    TouchingObjectMenu { menu: TouchingObjectMenu },
    DistanceToMenu { menu: DistanceToMenu },
    OfObjectMenu { menu: OfObjectMenu },

    TouchingObject { object: Box<Reporter> },
    TouchingColor { color: Box<Reporter> },
    ColorTouchingColor { a: Box<Reporter>, b: Box<Reporter> },
    DistanceToObject { object: Box<Reporter> },

    Answer,
    KeyPressed { key: Box<Reporter> },
    MouseDown,
    MouseX,
    MouseY,

    Loudness,
    Timer,

    PropertyOfObject { property: PropertyField, object: Box<Reporter> },

    Current { current: CurrentField },
    DaysSince2000,

    Online,
    Username,

    // Operators
    Add { a: Box<Reporter>, b: Box<Reporter> },
    Sub { a: Box<Reporter>, b: Box<Reporter> },
    Mul { a: Box<Reporter>, b: Box<Reporter> },
    Div { a: Box<Reporter>, b: Box<Reporter> },

    PickRandom { min: Box<Reporter>, max: Box<Reporter> },

    Gt { a: Box<Reporter>, b: Box<Reporter> },
    Lt { a: Box<Reporter>, b: Box<Reporter> },
    Eq { a: Box<Reporter>, b: Box<Reporter> },

    And { a: Box<Reporter>, b: Box<Reporter> },
    Or { a: Box<Reporter>, b: Box<Reporter> },
    Not { x: Box<Reporter> },

    Join { a: Box<Reporter>, b: Box<Reporter> },
    LetterOf { index: Box<Reporter>, text: Box<Reporter> },
    LengthOfText { text: Box<Reporter> },

    TextContains { haystack: Box<Reporter>, needle: Box<Reporter> },

    Mod { a: Box<Reporter>, b: Box<Reporter> },
    Round { x: Box<Reporter> },

    MathFunc { func: MathFuncField, x: Box<Reporter> },

    // Data
    ItemOfList { list: code::ListId, index: Box<Reporter> },
    ItemNumOfItemInList { list: code::ListId, val: Box<Reporter> },
    LengthOfList { list: code::ListId },
    ListContains { haystack: code::ListId, needle: Box<Reporter> },

    // Procedures
    Argument { name: String },

    // Unknown
    Unknown(UnknownBlock),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ProcPrototype {
    pub proccode: String,
    pub params: Vec<Param>,
    pub warp: bool,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Param {
    pub id: ArgId,
    pub name: String,
    pub kind: ParamKind,
    pub default: code::Value,
}

string_id! {
    /// The ID of a procedure parameter.
    ArgId
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamKind {
    StringNumber,
    Boolean,
}

/// A block this crate doesn't understand, kept in a lowered but uninterpreted form. If you think
/// this block should be added, create an issue on the repository. If you don't want to allow
/// unknown blocks, use [`Decoder::allow_unknown_blocks`].
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UnknownBlock {
    pub opcode: String,
    pub inputs: Vec<(String, UnknownInput)>,
    pub fields: Vec<(String, UnknownField)>,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum UnknownInput {
    /// A literal or reference, already lowered.
    Reporter(Reporter),
    /// A C-block substack.
    Stack(Vec<Action>),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UnknownField {
    pub value: String,
    pub id: Option<String>,
}

/// The location menu used by some motion blocks.
///
/// Specifically:
/// - `go to (menu)`
/// - `glide (secs) secs to (menu)`
/// - `point towards (menu)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LocationMenu {
    Random,
    MousePointer,
    Sprite(String),
}

/// The effect field used by some looks blocks.
///
/// Specifically:
/// - `change [graphic effect] effect by (inc)`
/// - `set [graphic effect] effect to (value)`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicEffectField {
    Color,
    Fisheye,
    Whirl,
    Pixelate,
    Mosaic,
    Brightness,
    Ghost,
}

/// The front-back field used by `go to [front-back] layer`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrontBackField {
    Front,
    Back,
}

/// The forward-backward field used by `go [forward-backward] (num) layers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForwardBackwardField {
    Forward,
    Backward,
}

/// The number-name field used by some looks blocks.
///
/// Specifically:
/// - `(costume [number-name])`
/// - `(backdrop [number-name])`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberNameField {
    Number,
    Name,
}

/// The effect field used by some sound blocks.
///
/// Specifically:
/// - `change [sound effect] effect by (inc)`
/// - `set [sound effect] effect to (value)`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SoundEffectField {
    Pitch,
    PanLeftRight,
}

/// The key field used by `when [key] key pressed`. Additionally, [`Reporter::KeyMenu`] also uses
/// this enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum KeyField {
    Space,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Any,
    Letter(char),
    Digit(u8),
    Other(String),
}

/// The loudness-timer field used by `when [loudness-timer] > (val)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LoudnessTimerField {
    Loudness,
    Timer,
}

/// The stop field used by `stop [stop]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StopField {
    All,
    ThisScript,
    OtherScriptsInSprite,
}

/// The object menu used by `<touching (menu)?>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TouchingObjectMenu {
    MousePointer,
    Edge,
    Sprite(String),
}

/// The object menu used by `(distance to (menu))`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DistanceToMenu {
    MousePointer,
    Sprite(String),
}

/// The object menu used by `([property] of (object))`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OfObjectMenu {
    Stage,
    Sprite(String),
}

/// The property field used by `([property] of (object))`.
///
/// Not every property is meaningful for every object (for example,
/// `BackdropNumber` only applies to the stage); Scratch itself does not enforce this.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PropertyField {
    // Sprite properties
    XPosition,
    YPosition,
    Direction,
    CostumeNumber,
    CostumeName,
    Size,

    // Stage properties
    BackdropNumber,
    BackdropName,

    // Both
    Volume,

    /// Any other string is a variable name on the target.
    Variable(String),
}

/// The current field used by `(current [current])`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrentField {
    Year,
    Month,
    Date,
    DayOfWeek,
    Hour,
    Minute,
    Second,
}

/// The clone of menu used by `create clone of [menu]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CloneOfMenu {
    Myself,
    Sprite(String),
}

/// The math function field used by `([math-func] of (x))`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MathFuncField {
    Abs,
    Floor,
    Ceiling,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Ln,
    Log,
    EToPower,
    TenToPower,
}
