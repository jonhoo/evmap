use std::hash::Hash;
/// Types from the standard library that are known to implement `Hash` and `Eq`
/// deterministically.
pub trait StableHashEq: Hash + Eq + sealed::Sealed {}

macro_rules! stable_hash_eq {
    ({$($imports:tt)*}
    $(
        $({$($a:lifetime),*$(,)?$($T:ident$(:?$Sized:ident)?),*$(,)?}$({$($manual_bounds:tt)*})?)? $Type:ty,
    )*) => {
        stable_hash_eq!{#
            {$($imports)*}
            $(
                $({$($a)*$($T$(:?$Sized$Sized)?)*})? $($({$($manual_bounds)*})?
                {
                    where $(
                        $T: StableHashEq,
                    )*
                })?
                $Type,
            )*
        }
    };
    (#{$($imports:tt)*}
    $(
        $({$($a:lifetime)*$($T:ident$(:?Sized$Sized:ident)?)*}{$($where_bounds:tt)*}$({$($_t:tt)*})?)? $Type:ty,
    )*) => {
        $($imports)*
        $(
            impl$(<$($a,)*$($T$(:?$Sized)?,)*>)? StableHashEq for $Type
            $($($where_bounds)*)? {}
        )*
        mod sealed {
            $($imports)*
            use super::StableHashEq;
            pub trait Sealed {}
            $(
                impl$(<$($a,)*$($T$(:?$Sized)?,)*>)? Sealed for $Type
                $($($where_bounds)*)? {}
            )*
        }
    };
}

stable_hash_eq! {
    {
        use std::collections::{BTreeMap, BTreeSet, VecDeque};
    }
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    bool, char, String,
    {'a, T} &'a [T],
    {'a, T: ?Sized} &'a T,
    {'a} &'a str,
    {T} Vec<T>,
    {T} VecDeque<T>,
    {T} BTreeSet<T>,
    {K, V} BTreeMap<K, V>,
    (),
    {T1: ?Sized} (T1,),
    {T1, T2: ?Sized} (T1, T2),
    {T1, T2, T3: ?Sized} (T1, T2, T3),
    {T1, T2, T3, T4: ?Sized} (T1, T2, T3, T4),
    {T1, T2, T3, T4, T5: ?Sized} (T1, T2, T3, T4, T5),
    {T1, T2, T3, T4, T5, T6: ?Sized} (T1, T2, T3, T4, T5, T6),
    {T1, T2, T3, T4, T5, T6, T7: ?Sized} (T1, T2, T3, T4, T5, T6, T7),
    {T1, T2, T3, T4, T5, T6, T7, T8: ?Sized} (T1, T2, T3, T4, T5, T6, T7, T8),
    {T1, T2, T3, T4, T5, T6, T7, T8, T9: ?Sized}
    (T1, T2, T3, T4, T5, T6, T7, T8, T9),
    {T1, T2, T3, T4, T5, T6, T7, T8, T9, T10: ?Sized}
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
    {T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11: ?Sized}
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    {T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12: ?Sized}
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
    {T} [T; 0], {T} [T; 1], {T} [T; 2], {T} [T; 3], {T} [T; 4],
    {T} [T; 5], {T} [T; 6], {T} [T; 7], {T} [T; 8], {T} [T; 9],
    {T} [T; 10], {T} [T; 11], {T} [T; 12], {T} [T; 13], {T} [T; 14],
    {T} [T; 15], {T} [T; 16], {T} [T; 17], {T} [T; 18], {T} [T; 19],
    {T} [T; 20], {T} [T; 21], {T} [T; 22], {T} [T; 23], {T} [T; 24],
    {T} [T; 25], {T} [T; 26], {T} [T; 27], {T} [T; 28], {T} [T; 29],
    {T} [T; 30], {T} [T; 31], {T} [T; 32],
}
