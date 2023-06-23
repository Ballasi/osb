use crate::event::*;
use crate::utils::{IntervalMap, Number, Vec2};
use crate::Layer;
use crate::Origin;

struct EventCollection {
    move_: IntervalMap<i32, Move>,
    movex_: IntervalMap<i32, MoveX>,
    movey_: IntervalMap<i32, MoveY>,
    fade_: IntervalMap<i32, Fade>,
    rotate_: IntervalMap<i32, Rotate>,
    scale_: IntervalMap<i32, Scale>,
    scalevec_: IntervalMap<i32, ScaleVec>,
    color_: IntervalMap<i32, Color>,
    hflip_: IntervalMap<i32, HFlip>,
    vflip_: IntervalMap<i32, VFlip>,
    additive_: IntervalMap<i32, Additive>,
}

/// `LoopType`s as defined in the [official osu! specifications](https://osu.ppy.sh/wiki/en/Storyboard_Scripting/Objects)
pub enum LoopType {
    /// Animation will stop on the last frame and continue displaying that last frame
    LoopOnce,
    /// Animation will loop forever
    LoopForever,
}

fn events_to_str<T>(events: &IntervalMap<i32, T>) -> String
where
    T: Event,
{
    let hs: std::collections::HashSet<_> = events
        .points
        .iter()
        .flat_map(|(_, inner_vec)| inner_vec.iter().map(|t| t.to_line() + "\n"))
        .collect();
    hs.into_iter().collect::<Vec<String>>().join("")
}

impl EventCollection {
    pub fn new() -> Self {
        Self {
            move_: IntervalMap::new(),
            movex_: IntervalMap::new(),
            movey_: IntervalMap::new(),
            fade_: IntervalMap::new(),
            rotate_: IntervalMap::new(),
            scale_: IntervalMap::new(),
            scalevec_: IntervalMap::new(),
            color_: IntervalMap::new(),
            hflip_: IntervalMap::new(),
            vflip_: IntervalMap::new(),
            additive_: IntervalMap::new(),
        }
    }

    pub fn to_str(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            events_to_str(&self.move_),
            events_to_str(&self.movex_),
            events_to_str(&self.movey_),
            events_to_str(&self.fade_),
            events_to_str(&self.rotate_),
            events_to_str(&self.scale_),
            events_to_str(&self.scalevec_),
            events_to_str(&self.color_),
            events_to_str(&self.hflip_),
            events_to_str(&self.vflip_),
            events_to_str(&self.additive_),
        )
    }
}

enum SpriteType {
    Sprite,
    Animation {
        frame_count: u32,
        frame_delay: u32,
        loop_type: LoopType,
    },
}

/// The struct corresponding to sprites
pub struct Sprite {
    events: EventCollection,
    current_depth: usize,
    path: String,
    pos: Vec2,
    layer: Layer,
    origin: Origin,
    start_time: Option<i32>,
    end_time: Option<i32>,
    type_: SpriteType,
}

// Adding an event to a sprite
macro_rules! add_event {
    ($sprite:ident, $event:ident, $events:expr) => {
        // Adjusting sprite's start and end values
        let (event_start, event_end) = ($event.get_start_time(), $event.get_end_time());
        match $sprite.start_time {
            Some(sprite_start) => {
                if event_start < sprite_start {
                    $sprite.start_time = Some(event_start)
                }
            }
            None => $sprite.start_time = Some(event_start),
        }

        match $sprite.end_time {
            Some(sprite_end) => {
                if sprite_end < event_end {
                    $sprite.end_time = Some(event_end)
                }
            }
            None => $sprite.end_time = Some(event_end),
        }

        // Pushing it to the events
        $event.set_depth($sprite.current_depth);
        $events.push(event_start..event_end, $event);
    };
}

impl Sprite {
    /// Initializes a new `Sprite` or an animation `Sprite`
    ///
    /// See [trait implementations](#trait-implementations) to see how you can create a Sprite element
    pub fn new<T>(args: T) -> Self
    where
        T: Into<Sprite>,
    {
        args.into()
    }

    /// Performs the event [`Move`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    ///
    /// // There's a `Vec2` type you can use if you wish
    /// sprite.move_((Easing::Out, 0, 1000, Vec2::from(0, 0), Vec2::from(320, 240)));
    /// // But you're not forced to! Giving pairs of integers automatically translates to a `Vec2`
    /// sprite.move_((Easing::QuadInOut, 1000, 2000, 320, 240, 100, 100));
    /// // And of course you can use a static move too
    /// sprite.move_((3000, Vec2::from(320, 240)));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn move_<T>(&mut self, args: T)
    where
        T: Into<Move>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.move_);
    }

    /// Performs the event [`MoveX`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.movex_((0, 320));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn movex_<T>(&mut self, args: T)
    where
        T: Into<MoveX>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.movex_);
    }

    /// Performs the event [`MoveY`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.movey_((0, 240));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn movey_<T>(&mut self, args: T)
    where
        T: Into<MoveY>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.movey_);
    }

    /// Performs the event [`Fade`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.fade_((0, 1));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn fade_<T>(&mut self, args: T)
    where
        T: Into<Fade>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.fade_);
    }

    /// Performs the event [`Rotate`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    /// use std::f32::consts::PI;
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.rotate_((0, PI));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn rotate_<T>(&mut self, args: T)
    where
        T: Into<Rotate>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.rotate_);
    }

    /// Performs the event [`Scale`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.scale_((0, 1));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn scale_<T>(&mut self, args: T)
    where
        T: Into<Scale>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.scale_);
    }

    /// Performs the event [`ScaleVec`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// // There's a `Vec2` type you can use if you wish
    /// sprite.scalevec_((Easing::Out, 0, 1000, Vec2::from(1, 0), Vec2::from(1, 1)));
    /// // But you're not forced to! Giving pairs of integers automatically translates to a `Vec2`
    /// sprite.scalevec_((Easing::QuadInOut, 1000, 2000, 1, 0, 1, 1));
    /// // And of course you can use a static ScaleVec too
    /// sprite.scalevec_((3000, Vec2::from(1, 0.5)));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn scalevec_<T>(&mut self, args: T)
    where
        T: Into<ScaleVec>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.scalevec_);
    }

    /// Performs the event [`Color`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Color};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// // There's a `Color` type you can use if you wish
    /// sprite.color_((Easing::Out, 0, 1000, Color::white(), Color::red()));
    /// // But you're not forced to! Giving pairs of integers automatically translates to a `Color`
    /// sprite.color_((Easing::QuadInOut, 1000, 2000, 255, 255, 255, 255, 0, 0));
    /// // And of course you can use a static Color too
    /// sprite.color_((3000, Color::green()));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn color_<T>(&mut self, args: T)
    where
        T: Into<Color>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.color_);
    }

    /// Performs the event [`HFlip`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.hflip_((0, 1000));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn hflip_<T>(&mut self, args: T)
    where
        T: Into<HFlip>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.hflip_);
    }

    /// Performs the event [`VFlip`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.vflip_((0, 1000));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn vflip_<T>(&mut self, args: T)
    where
        T: Into<VFlip>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.vflip_);
    }

    /// Performs the event [`Additive`] to a `Sprite`
    ///
    /// ```
    /// use osb::{Sprite, Easing, utils::Vec2};
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.additive_((0, 1000));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn additive_<T>(&mut self, args: T)
    where
        T: Into<Additive>,
    {
        let mut event = args.into();
        add_event!(self, event, self.events.additive_);
    }

    /// Returns the initial X position of a `Sprite`
    ///
    /// **Warning**: This does **not** return the X position in a certain time.
    ///
    /// Example:
    /// ```
    /// use osb::{utils::Number, Sprite};
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.move_((0, 100, 100));
    /// assert_eq!(sprite.get_x(), Number::Int(320));
    /// ```
    pub fn get_x(&self) -> Number {
        self.pos.x
    }

    /// Returns the initial Y position of a `Sprite`
    ///
    /// **Warning**: This does **not** return the Y position in a certain time.
    ///
    /// Example:
    /// ```
    /// use osb::{utils::Number, Sprite};
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.move_((0, 100, 100));
    /// assert_eq!(sprite.get_y(), Number::Int(240));
    /// ```
    pub fn get_y(&self) -> Number {
        self.pos.y
    }

    /// Returns the start time of the first event of a `Sprite`
    ///
    /// Example:
    /// ```
    /// use osb::Sprite;
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// assert_eq!(sprite.start_time(), None);
    ///
    /// sprite.move_((100, 200, 0, 0, 320, 240));
    /// assert_eq!(sprite.start_time(), Some(100));
    ///
    /// sprite.fade_((0, 100, 0, 1));
    /// assert_eq!(sprite.start_time(), Some(0));
    /// ```
    pub fn start_time(&self) -> Option<i32> {
        self.start_time
    }

    /// Returns the end time of the first event of a `Sprite`
    ///
    /// Example:
    /// ```
    /// use osb::Sprite;
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// assert_eq!(sprite.end_time(), None);
    ///
    /// sprite.move_((0, 100, 0, 0, 320, 240));
    /// assert_eq!(sprite.end_time(), Some(100));
    ///
    /// sprite.fade_((100, 200, 1, 0));
    /// assert_eq!(sprite.end_time(), Some(200));
    /// ```
    pub fn end_time(&self) -> Option<i32> {
        self.end_time
    }

    /// Returns the contents of the `Sprite`
    ///
    /// **Warning**: this method is not meant to be used
    pub fn to_str(&self) -> String {
        match &self.type_ {
            SpriteType::Sprite => {
                return format!(
                    "Sprite,{},{},\"{}\",{},{}\n{}",
                    self.layer,
                    self.origin,
                    self.path,
                    self.pos.x,
                    self.pos.y,
                    self.events.to_str()
                );
            }
            SpriteType::Animation {
                frame_count,
                frame_delay,
                loop_type,
            } => {
                return format!(
                    "Animation,{},{},\"{}\",{},{},{},{}{}\n{}",
                    self.layer,
                    self.origin,
                    self.path,
                    self.pos.x,
                    self.pos.y,
                    frame_count,
                    frame_delay,
                    match loop_type {
                        LoopType::LoopOnce => ",LoopOnce",
                        // defaults to LoopForever if not specified
                        LoopType::LoopForever => "",
                    },
                    self.events.to_str()
                );
            }
        }
    }

    /// Sets the [`Layer`] of the `Sprite`
    ///
    /// **Warning**: this method is not meant to be used
    pub fn set_layer(&mut self, layer: Layer) {
        self.layer = layer;
    }
}

/// Creates a `Sprite` with the path of the file
///
/// Example:
/// ```
/// use osb::Sprite;
/// let path = String::from("res/sprite.png");
/// let mut sprite = Sprite::new(path);
/// ```
impl Into<Sprite> for String {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self,
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the path of the file
///
/// Example:
/// ```
/// use osb::Sprite;
/// let path = "res/sprite.png";
/// let mut sprite = Sprite::new(path);
/// ```
impl Into<Sprite> for &str {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self),
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the origin and path of the file
///
/// Example:
/// ```
/// use osb::{Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let mut sprite = Sprite::new((origin, path));
/// ```
impl Into<Sprite> for (Origin, String) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.1,
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the origin and the path of the file
///
/// Example:
/// ```
/// use osb::{Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let mut sprite = Sprite::new((origin, path));
/// ```
impl Into<Sprite> for (Origin, &str) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = String::from("res/sprite.png");
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((path, pos));
/// ```
impl Into<Sprite> for (String, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.0,
            pos: self.1,
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = String::from("res/sprite.png");
/// let x = 320;
/// let y = 240;
/// let mut sprite = Sprite::new((path, x, y));
/// ```
impl<T, U> Into<Sprite> for (String, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.0,
            pos: Vec2::from(self.1, self.2),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = "res/sprite.png";
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((path, pos));
/// ```
impl Into<Sprite> for (&str, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.0),
            pos: self.1,
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = "res/sprite.png";
/// let x = 320;
/// let y = 240;
/// let mut sprite = Sprite::new((path, x, y));
/// ```
impl<T, U> Into<Sprite> for (&str, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.0),
            pos: Vec2::from(self.1, self.2),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((origin, path, pos));
/// ```
impl Into<Sprite> for (Origin, String, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.1,
            pos: self.2,
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let x = 320;
/// let y = 240;
/// let mut sprite = Sprite::new((origin, path, x, y));
/// ```
impl<T, U> Into<Sprite> for (Origin, String, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.1,
            pos: Vec2::from(self.2, self.3),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((origin, path, pos));
/// ```
impl Into<Sprite> for (Origin, &str, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: self.2,
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let x = 320;
/// let y = 240;
/// let mut sprite = Sprite::new((origin, path, x, y));
/// ```
impl<T, U> Into<Sprite> for (Origin, &str, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: Vec2::from(self.2, self.3),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Sprite,
        }
    }
}

/// Creates a `Sprite` animation with the path of the file
///
/// Example:
/// ```
/// use osb::{Sprite, LoopType};
/// let path = String::from("res/sprite.png");
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((path, frame_count, frame_delay, loop_type));
/// ```
impl Into<Sprite> for (String, u32, u32, LoopType) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.0,
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.1,
                frame_delay: self.2,
                loop_type: self.3,
            },
        }
    }
}

/// Creates a `Sprite` animation with the path of the file
///
/// Example:
/// ```
/// use osb::{Sprite, LoopType};
/// let path = "res/sprite.png";
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((path, frame_count, frame_delay, loop_type));
/// ```
impl Into<Sprite> for (&str, u32, u32, LoopType) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.0),
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.1,
                frame_delay: self.2,
                loop_type: self.3,
            },
        }
    }
}

/// Creates a `Sprite` animation with the origin and path of the file
///
/// Example:
/// ```
/// use osb::{Origin, Sprite, LoopType};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((origin, path, frame_count, frame_delay, loop_type));
/// ```
impl Into<Sprite> for (Origin, String, u32, u32, LoopType) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.1,
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.2,
                frame_delay: self.3,
                loop_type: self.4,
            },
        }
    }
}

/// Creates a `Sprite` animation with the origin and the path of the file
///
/// Example:
/// ```
/// use osb::{Origin, Sprite, LoopType};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((origin, path, frame_count, frame_delay, loop_type));
/// ```
impl Into<Sprite> for (Origin, &str, u32, u32, LoopType) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.2,
                frame_delay: self.3,
                loop_type: self.4,
            },
        }
    }
}

/// Creates a `Sprite` animation with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite, LoopType};
/// let path = String::from("res/sprite.png");
/// let pos = Vec2::from(320, 240);
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((path, pos, frame_count, frame_delay, loop_type));
/// ```
impl Into<Sprite> for (String, Vec2, u32, u32, LoopType) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.0,
            pos: self.1,
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.2,
                frame_delay: self.3,
                loop_type: self.4,
            },
        }
    }
}

/// Creates a `Sprite` animation with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite, LoopType};
/// let path = String::from("res/sprite.png");
/// let x = 320;
/// let y = 240;
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((path, x, y, frame_count, frame_delay, loop_type));
/// ```
impl<T, U> Into<Sprite> for (String, T, U, u32, u32, LoopType)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.0,
            pos: Vec2::from(self.1, self.2),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.3,
                frame_delay: self.4,
                loop_type: self.5,
            },
        }
    }
}

/// Creates a `Sprite` animation with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite, LoopType};
/// let path = "res/sprite.png";
/// let pos = Vec2::from(320, 240);
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((path, pos, frame_count, frame_delay, loop_type));
/// ```
impl Into<Sprite> for (&str, Vec2, u32, u32, LoopType) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.0),
            pos: self.1,
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.2,
                frame_delay: self.3,
                loop_type: self.4,
            },
        }
    }
}

/// Creates a `Sprite` animation with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite, LoopType};
/// let path = "res/sprite.png";
/// let x = 320;
/// let y = 240;
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((path, x, y, frame_count, frame_delay, loop_type));
/// ```
impl<T, U> Into<Sprite> for (&str, T, U, u32, u32, LoopType)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.0),
            pos: Vec2::from(self.1, self.2),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.3,
                frame_delay: self.4,
                loop_type: self.5,
            },
        }
    }
}

/// Creates a `Sprite` animation with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite, LoopType};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let pos = Vec2::from(320, 240);
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((origin, path, pos, frame_count, frame_delay, loop_type));
/// ```
impl Into<Sprite> for (Origin, String, Vec2, u32, u32, LoopType) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.1,
            pos: self.2,
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.3,
                frame_delay: self.4,
                loop_type: self.5,
            },
        }
    }
}

/// Creates a `Sprite` animation with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite, LoopType};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let x = 320;
/// let y = 240;
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((origin, path, x, y, frame_count, frame_delay, loop_type));
/// ```
impl<T, U> Into<Sprite> for (Origin, String, T, U, u32, u32, LoopType)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.1,
            pos: Vec2::from(self.2, self.3),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.4,
                frame_delay: self.5,
                loop_type: self.6,
            },
        }
    }
}

/// Creates a `Sprite` animation with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite, LoopType};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let pos = Vec2::from(320, 240);
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((origin, path, pos, frame_count, frame_delay, loop_type));
/// ```
impl Into<Sprite> for (Origin, &str, Vec2, u32, u32, LoopType) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: self.2,
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.3,
                frame_delay: self.4,
                loop_type: self.5,
            },
        }
    }
}

/// Creates a `Sprite` animation with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite, LoopType};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let x = 320;
/// let y = 240;
/// let frame_count = 20;
/// let frame_delay = 100;
/// let loop_type = LoopType::LoopForever;
/// let mut sprite = Sprite::new((origin, path, x, y, frame_count, frame_delay, loop_type));
/// ```
impl<T, U> Into<Sprite> for (Origin, &str, T, U, u32, u32, LoopType)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: Vec2::from(self.2, self.3),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
            type_: SpriteType::Animation {
                frame_count: self.4,
                frame_delay: self.5,
                loop_type: self.6,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LoopType, Sprite};

    #[test]
    fn animation() {
        let sprite = Sprite::new(("sb/sprite.jpg", 10, 10, LoopType::LoopOnce));
        assert_eq!(
            "Animation,Background,Centre,\"sb/sprite.jpg\",320,240,10,10,LoopOnce\n",
            sprite.to_str()
        );
    }
}
