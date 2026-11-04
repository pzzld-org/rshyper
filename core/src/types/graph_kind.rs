/*
    appellation: graph_kind <module>
    authors: @FL03
*/
/// [`GraphType`] is a marker trait for graph types.
///
/// **note:** This trait is sealed and cannot be implemented outside of this crate.
pub trait GraphType: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();
}

/// [`Mode`] enumerates the possible graph variants enabling dynamic dispatch features.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    #[cfg_attr(feature = "serde", serde(alias = "d", alias = "dir"))]
    Directed = 0,
    #[default]
    #[cfg_attr(feature = "serde", serde(alias = "u", alias = "un", alias = "undir"))]
    Undirected = 1,
}

impl Mode {
    /// a functional constructor for the [`Directed`](Mode::Directed) mode.
    pub const fn directed() -> Self {
        Mode::Directed
    }
    /// a functional constructor for the [`Undirected`](Mode::Undirected) mode.
    pub const fn undirected() -> Self {
        Mode::Undirected
    }
    /// returns the [`Mode`] corresponding to the given [`GraphType`], or `None` for a kind
    /// that fixes no direction at the type level, such as [`Mode`] itself: a `Mode`-kinded
    /// graph stores no runtime mode.
    pub fn from_type<T: GraphType>() -> Option<Self> {
        use core::any::TypeId;
        if TypeId::of::<T>() == TypeId::of::<Directed>() {
            Some(Mode::Directed)
        } else if TypeId::of::<T>() == TypeId::of::<Undirected>() {
            Some(Mode::Undirected)
        } else {
            None
        }
    }
    /// returns the [`GraphType`] corresponding to this mode.
    pub fn as_type(&self) -> &'static dyn GraphType {
        match self {
            Mode::Directed => &Directed,
            Mode::Undirected => &Undirected,
        }
    }
    /// returns true if the current variant matches the given graph type.
    pub fn is<T: 'static>(&self) -> bool {
        use core::any::TypeId;
        match self {
            Mode::Directed => TypeId::of::<T>() == TypeId::of::<Directed>(),
            Mode::Undirected => TypeId::of::<T>() == TypeId::of::<Undirected>(),
        }
    }
    /// returns a boxed instance of the corresponding [`GraphType`].
    pub fn boxed(self) -> Box<dyn GraphType> {
        match self {
            Mode::Directed => Box::new(Directed),
            Mode::Undirected => Box::new(Undirected),
        }
    }
}
/*
 ************* Implementations *************
*/
macro_rules! impl_kind {
    (@impl $(#[doc$($doc:tt)*])? $vis:vis enum $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize)
        )]
        pub enum $kind {}
    };
    (@impl $(#[doc$($doc:tt)*])? $vis:vis struct $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize)
        )]
        pub struct $kind;

        impl $kind {
            pub const fn new() -> Self {
                Self
            }
        }
    };
    (@display $kind:ident) => {
        impl ::core::fmt::Display for $kind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", ::core::any::type_name::<Self>())
            }
        }
    };
    (@impls $kind:ident) => {
        unsafe impl Send for $kind {}

        unsafe impl Sync for $kind {}

        impl $crate::types::graph_kind::GraphType for $kind {
            seal!();
        }
    };
    ($(
        $(#[doc$($doc:tt)*])?
        $vis:vis $itype:ident $kind:ident
    );* $(;)?) => {
        $(
            impl_kind!(@impl $(#[doc $($doc)*])? $vis $itype $kind);
            impl_kind!(@display $kind);
            impl_kind!(@impls $kind);
        )*
    };
}

impl_kind! {
    #[doc = "A marker type representing a _directed_ graph type"]
    pub struct Directed;
    #[doc = "A marker type representing an _undirected_ graph type"]
    pub struct Undirected;
}

impl_kind!(@impls Mode);
