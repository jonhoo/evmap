/// Sealed trait for types in [`std`] that are known to implement
/// `Hash` and `Eq` deterministically.
pub trait StableHashEq: Hash + Eq + sealed_hash_eq::Sealed {}

mod sealed_hash_eq {
    pub trait Sealed {}
}

macro_rules! stable_hash_eq {
    ($(
        $({$($a:lifetime),*$(,)?$($T:ident$(:?$Sized:ident)?),*$(,)?}
        $({$($manual_bounds:tt)*})?)? $Type:ty,
    )*) => {
        stable_hash_eq!{#
            $(
                $({$($a)*$($T$(:?$Sized$Sized)?)*})? $($({where $($manual_bounds)*})?
                {
                    where $(
                        $T: StableHashEq,
                    )*
                })?
                $Type,
            )*
        }
    };
    (#$(
        $({$($a:lifetime)*$($T:ident$(:?Sized$Sized:ident)?)*}
        {$($where_bounds:tt)*}$({$($_t:tt)*})?)? $Type:ty,
    )*) => {
        $(
            impl$(<$($a,)*$($T$(:?$Sized)?,)*>)? StableHashEq for $Type
            $($($where_bounds)*)? {}
            impl$(<$($a,)*$($T$(:?$Sized)?,)*>)? sealed_hash_eq::Sealed for $Type
            $($($where_bounds)*)? {}
        )*
    };
}

use std::{
    any::TypeId,
    borrow::Cow,
    cell::{Ref, RefMut},
    cmp::{self, Reverse},
    collections::{binary_heap::PeekMut, BTreeMap, BTreeSet, LinkedList, VecDeque},
    convert::Infallible,
    ffi::{CStr, CString, OsStr, OsString},
    fmt,
    fs::FileType,
    hash::Hash,
    io::{ErrorKind, IoSlice, IoSliceMut},
    marker::{PhantomData, PhantomPinned},
    mem::{Discriminant, ManuallyDrop},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Wrapping,
    },
    ops::{Bound, Deref, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    panic::AssertUnwindSafe,
    path::{Component, Path, PathBuf, Prefix, PrefixComponent},
    pin::Pin,
    ptr::NonNull,
    rc::Rc,
    sync::{atomic, Arc, MutexGuard, RwLockReadGuard, RwLockWriteGuard},
    task::Poll,
    thread::ThreadId,
    time::{Duration, Instant, SystemTime},
};

stable_hash_eq! {
    cmp::Ordering,
    Infallible,
    ErrorKind,
    IpAddr,
    SocketAddr,
    atomic::Ordering,
    bool, char,
    i8, i16, i32, i64, i128,
    isize,
    str,
    u8, u16, u32, u64, u128,
    (),
    usize,
    TypeId,
    CStr,
    CString,
    OsStr,
    OsString,
    fmt::Error,
    FileType,
    PhantomPinned,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddrV4,
    SocketAddrV6,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    RangeFull,
    Path,
    PathBuf,
    String,
    ThreadId,
    Duration,
    Instant,
    SystemTime,
    {'a} PrefixComponent<'a>,
    {'a} Cow<'a, str>,
    {'a} Cow<'a, CStr>,
    {'a} Cow<'a, OsStr>,
    {'a} Cow<'a, Path>,
    {'a, T}{T: Clone + StableHashEq} Cow<'a, [T]>,
    {'a, T}{T: Clone + StableHashEq} Cow<'a, T>,
    {'a, T: ?Sized} &'a T,
    {'a, T: ?Sized} &'a mut T,
    {'a} Component<'a>,
    {'a} Prefix<'a>,
    {A: ?Sized} (A,),
    {T} VecDeque<T>,
    {A, B: ?Sized} (A, B),
    {A, B, C: ?Sized} (A, B, C),
    {A, B, C, D: ?Sized} (A, B, C, D),
    {A, B, C, D, E: ?Sized} (A, B, C, D, E),
    {A, B, C, D, E, F: ?Sized} (A, B, C, D, E, F),
    {A, B, C, D, E, F, G: ?Sized} (A, B, C, D, E, F, G),
    {A, B, C, D, E, F, G, H: ?Sized} (A, B, C, D, E, F, G, H),
    {A, B, C, D, E, F, G, H, I: ?Sized} (A, B, C, D, E, F, G, H, I),
    {A, B, C, D, E, F, G, H, I, J: ?Sized} (A, B, C, D, E, F, G, H, I, J),
    {A, B, C, D, E, F, G, H, I, J, K: ?Sized} (A, B, C, D, E, F, G, H, I, J, K),
    {A, B, C, D, E, F, G, H, I, J, K, L: ?Sized} (A, B, C, D, E, F, G, H, I, J, K, L),
    {Idx} Range<Idx>,
    {Idx} RangeFrom<Idx>,
    {Idx} RangeInclusive<Idx>,
    {Idx} RangeTo<Idx>,
    {Idx} RangeToInclusive<Idx>,
    {K, V} BTreeMap<K, V>,
    {P}{P: StableDeref, P::Target: StableHashEq} Pin<P>,
}
macro_rules! stable_hash_eq_fn {
    ($({$($($A:ident),+)?})*) => {
        stable_hash_eq!{
            $(
                {Ret$(, $($A),+)?}{} fn($($($A),+)?) -> Ret,
                {Ret$(, $($A),+)?}{} extern "C" fn($($($A),+)?) -> Ret,
                $({Ret, $($A),+}{} extern "C" fn($($A),+, ...) -> Ret,)?
                {Ret$(, $($A),+)?}{} unsafe fn($($($A),+)?) -> Ret,
                {Ret$(, $($A),+)?}{} unsafe extern "C" fn($($($A),+)?) -> Ret,
                $({Ret, $($A),+}{} unsafe extern "C" fn($($A),+, ...) -> Ret,)?
            )*
        }
    };
}
stable_hash_eq_fn! {
    {}
    {A}
    {A, B}
    {A, B, C}
    {A, B, C, D}
    {A, B, C, D, E}
    {A, B, C, D, E, F}
    {A, B, C, D, E, F, G}
    {A, B, C, D, E, F, G, H}
    {A, B, C, D, E, F, G, H, I}
    {A, B, C, D, E, F, G, H, I, J}
    {A, B, C, D, E, F, G, H, I, J, K}
    {A, B, C, D, E, F, G, H, I, J, K, L}
}
stable_hash_eq! {
    {T} Bound<T>,
    {T} Option<T>,
    {T} Poll<T>,
    {T: ?Sized}{} *const T,
    {T: ?Sized}{} *mut T,
    {T} [T],
    {T: ?Sized} Box<T>,
    {T} Reverse<T>,
    {T} BTreeSet<T>,
    {T} LinkedList<T>,
    {T: ?Sized}{} PhantomData<T>,
    {T}{} Discriminant<T>,
    {T} ManuallyDrop<T>,
    {T} Wrapping<T>,
    {T: ?Sized}{} NonNull<T>,
    {T: ?Sized} Rc<T>,
    {T: ?Sized} Arc<T>,
    {T} Vec<T>,
    {T, E} Result<T, E>,
    {T} [T; 0], {T} [T; 1], {T} [T; 2], {T} [T; 3], {T} [T; 4],
    {T} [T; 5], {T} [T; 6], {T} [T; 7], {T} [T; 8], {T} [T; 9],
    {T} [T; 10], {T} [T; 11], {T} [T; 12], {T} [T; 13], {T} [T; 14],
    {T} [T; 15], {T} [T; 16], {T} [T; 17], {T} [T; 18], {T} [T; 19],
    {T} [T; 20], {T} [T; 21], {T} [T; 22], {T} [T; 23], {T} [T; 24],
    {T} [T; 25], {T} [T; 26], {T} [T; 27], {T} [T; 28], {T} [T; 29],
    {T} [T; 30], {T} [T; 31], {T} [T; 32],
}

/// Sealed trait for types in [`std`] that are known to implement
/// `Deref` deterministically.
pub trait StableDeref: Deref + sealed_deref::Sealed {}
mod sealed_deref {
    pub trait Sealed {}
}

macro_rules! stable_deref {
    ($(
        $({$($a:lifetime),*$(,)?$($T:ident$(:?$Sized:ident)?),*$(,)?}
        $({$($manual_bounds:tt)*})?)? $Type:ty,
    )*) => {
        stable_deref!{#
            $(
                $({$($a)*$($T$(:?$Sized$Sized)?)*})? $($({where $($manual_bounds)*})?
                {
                    /* without default constraints, unlike stable_hash_eq macro */
                })?
                $Type,
            )*
        }
    };
    (#$(
        $({$($a:lifetime)*$($T:ident$(:?Sized$Sized:ident)?)*}
        {$($where_bounds:tt)*}$({$($_t:tt)*})?)? $Type:ty,
    )*) => {
        $(
            impl$(<$($a,)*$($T$(:?$Sized)?,)*>)? StableDeref for $Type
            $($($where_bounds)*)? {}
            impl$(<$($a,)*$($T$(:?$Sized)?,)*>)? sealed_deref::Sealed for $Type
            $($($where_bounds)*)? {}
        )*
    };
}

stable_deref! {
    CString,
    OsString,
    PathBuf,
    String,
    {'a} Cow<'a, str>,
    {'a} Cow<'a, CStr>,
    {'a} Cow<'a, OsStr>,
    {'a} Cow<'a, Path>,
    {'a, T}{T: Clone} Cow<'a, [T]>,
    {'a, T}{T: Clone} Cow<'a, T>,
    {'a, T: ?Sized} &'a T,
    {'a, T: ?Sized} &'a mut T,
    {'a, T: ?Sized} Ref<'a, T>,
    {'a, T: ?Sized} RefMut<'a, T>,
    // the `Ord` impl is not used when dereferencing PeekMut
    {'a, T}{T: Ord} PeekMut<'a, T>,
    {'a} IoSlice<'a>,
    {'a} IoSliceMut<'a>,
    {P}{P: StableDeref} Pin<P>,
    {T: ?Sized} Box<T>,
    {T: ?Sized} ManuallyDrop<T>,
    {T} AssertUnwindSafe<T>,
    {T: ?Sized} Rc<T>,
    {T: ?Sized} Arc<T>,
    {T} Vec<T>,
    {'a, T: ?Sized} MutexGuard<'a, T>,
    {'a, T: ?Sized} RwLockReadGuard<'a, T>,
    {'a, T: ?Sized} RwLockWriteGuard<'a, T>,
}
