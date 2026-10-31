#![doc = include_str!("../README.md")]

use std::{
    io::{BufRead, BufReader, Read, Split},
    marker::PhantomData,
};

struct Tokens {
    data: Vec<u8>,
    index: usize,
}

impl Tokens {
    fn new(data: Vec<u8>) -> Self {
        Self { data, index: 0 }
    }

    fn next_token(&mut self) -> Option<&[u8]> {
        if self.index >= self.data.len() {
            return None;
        }

        while self.index < self.data.len() && self.data[self.index].is_ascii_whitespace() {
            self.index += 1;
        }

        if self.index >= self.data.len() {
            return None;
        }

        let start = self.index;
        while self.index < self.data.len() && !self.data[self.index].is_ascii_whitespace() {
            self.index += 1;
        }

        Some(&self.data[start..self.index])
    }

    fn is_empty(&self) -> bool {
        let mut temp = self.index;

        while temp < self.data.len() && self.data[temp].is_ascii_whitespace() {
            temp += 1;
        }

        temp >= self.data.len()
    }
}

/// A trait for token sources that provide tokens for the [`Readable`] trait and
/// the [`input!()`] macro
pub trait InputSource<R: Read> {
    /// Creates a new input source
    fn new(source: R) -> Self;
    /// Returns the next token
    ///
    /// Returns [`None`] if there is no token left
    fn next_token(&mut self) -> Option<&[u8]>;
    /// Returns the next token
    ///
    /// # Panics
    /// Panics if there is no token left.
    fn next_token_unwrap(&mut self) -> &[u8] {
        self.next_token().expect("failed to get token")
    }
}

/// An input source that reads all tokens at once
///
/// Use for normal competitive programming input. Reads everything up front; fastest.
/// For interactive problems, use [`LineInput`] instead, which reads input line by line.
pub struct FullInput<R: Read> {
    tokens: Tokens,
    _r: PhantomData<R>,
}

impl<R: Read> InputSource<R> for FullInput<R> {
    fn new(mut source: R) -> Self {
        let mut data = Vec::new();
        source
            .read_to_end(&mut data)
            .expect("failed to read from source");
        Self {
            tokens: Tokens::new(data),
            _r: PhantomData,
        }
    }

    fn next_token(&mut self) -> Option<&[u8]> {
        self.tokens.next_token()
    }
}

/// An input source that read tokens line by line
///
/// Use for interactive problems, where input arrives line by line.
/// For normal problems, prefer [`FullInput`], which reads all input at once
pub struct LineInput<R: Read> {
    source: Split<BufReader<R>>,
    tokens: Tokens,
}

impl<R: Read> LineInput<R> {
    /// Returns the next line
    ///
    /// Returns [`None`] if there is no line left
    ///
    /// # Notes
    /// If there're tokens in the current buffer, they will be discarded away,
    /// and the next line is returned
    pub fn next_line(&mut self) -> Option<Vec<u8>> {
        self.tokens = Tokens::new(Vec::new());
        self.source.next()?.ok()
    }
}

impl<R: Read> InputSource<R> for LineInput<R> {
    fn new(source: R) -> Self {
        Self {
            source: BufReader::new(source).split(b'\n'),
            tokens: Tokens::new(Vec::new()),
        }
    }

    fn next_token(&mut self) -> Option<&[u8]> {
        loop {
            if !self.tokens.is_empty() {
                return self.tokens.next_token();
            }
            self.tokens.data = self.source.next()?.ok()?;
            self.tokens.index = 0;
        }
    }
}

/// A trait for types that can be parsed with tokens from an [`InputSource`]
pub trait Readable {
    /// Parse the type with tokens from `source`
    ///
    /// # Panics
    /// This function panics if `source` is exhausted, or the tokens cannot be parsed
    /// into the implemented type
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self;
}

macro_rules! impl_uint_readable {
    () => {};
    ($uint:ty) => {
        impl Readable for $uint {
            fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
                let mut token = source.next_token_unwrap();
                let sign_byte = *token.get(0).expect("empty token");
                if matches!(sign_byte, b'+') {
                    token = &token[1..];
                }

                let mut result: Self = 0;
                for byte in token {
                    let digit = byte.wrapping_sub(b'0') as $uint;
                    debug_assert!(digit < 10, "invalid digit while reading an integer");
                    result = result * 10 + digit;
                }

                result
            }
        }
    };
    ($uint:ty, $($rest:ty),* $(,)?) => {
        impl_uint_readable!($uint);
        impl_uint_readable!($($rest),*);
    };
}

macro_rules! impl_int_readable {
    () => {};
    ($int:ty) => {
        impl Readable for $int {
            fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
                let mut token = source.next_token_unwrap();
                let sign_byte = *token.get(0).expect("empty token");
                if matches!(sign_byte, b'+' | b'-') {
                    token = &token[1..];
                }
                let sign = match sign_byte {
                    b'-' => -1,
                    _ => 1,
                };

                let mut result: Self = 0;
                for byte in token {
                    let digit = byte.wrapping_sub(b'0') as $int;
                    debug_assert!(digit < 10, "invalid digit while reading an integer");
                    result = result * 10 + digit;
                }

                result * sign
            }
        }
    };
    ($int:ty, $($rest:ty),* $(,)?) => {
        impl_int_readable!($int);
        impl_int_readable!($($rest),*);
    };
}

impl_uint_readable!(u8, u16, u32, u64, u128, usize);
impl_int_readable!(i8, i16, i32, i64, i128, isize);

// Gonna use `str::parse` for floats since problems with floats are usually rare
// and float parsing is very tricky

impl Readable for f32 {
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        std::str::from_utf8(source.next_token_unwrap())
            .expect("invalid float while reading")
            .parse()
            .expect("invalid float while reading")
    }
}

impl Readable for f64 {
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        str::parse(
            std::str::from_utf8(source.next_token_unwrap()).expect("invalid float while reading"),
        )
        .expect("invalid float while reading")
    }
}

impl Readable for String {
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        let token = source.next_token_unwrap();
        std::str::from_utf8(token)
            .expect("invalid utf8 string")
            .to_owned()
    }
}

impl Readable for Vec<u8> {
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        source.next_token_unwrap().to_owned()
    }
}

impl<T: Readable, U: Readable> Readable for (T, U) {
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        (T::read(source), U::read(source))
    }
}

impl<T: Readable, U: Readable, V: Readable> Readable for (T, U, V) {
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        (T::read(source), U::read(source), V::read(source))
    }
}

impl<T: Readable, U: Readable, V: Readable, W: Readable> Readable for (T, U, V, W) {
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        (
            T::read(source),
            U::read(source),
            V::read(source),
            W::read(source),
        )
    }
}

impl<T: Readable, U: Readable, V: Readable, W: Readable, X: Readable> Readable for (T, U, V, W, X) {
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        (
            T::read(source),
            U::read(source),
            V::read(source),
            W::read(source),
            X::read(source),
        )
    }
}

impl<T: Readable, U: Readable, V: Readable, W: Readable, X: Readable, Y: Readable> Readable
    for (T, U, V, W, X, Y)
{
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        (
            T::read(source),
            U::read(source),
            V::read(source),
            W::read(source),
            X::read(source),
            Y::read(source),
        )
    }
}

impl<T: Readable, const N: usize> Readable for [T; N] {
    fn read<R: Read, I: InputSource<R>>(source: &mut I) -> Self {
        std::array::from_fn(|_| T::read(source))
    }
}

/// Reads values from an input source into variables.
///
/// # Examples
/// ```
/// use std::io::Cursor;
/// use cpproio::{FullInput, InputSource, input};
///
/// let cursor = Cursor::new(b"3 2 1 2 3 4 5 6 2 2");
/// let mut source = FullInput::new(cursor);
///
/// input! {
///     source,
///     n: usize,
///     m: usize,
///     mut arr: Vec<i32>[n; m],
///     pair: (u8, u8),
/// }
/// arr[0][0] = 0;
///
/// assert_eq!(n, 3);
/// assert_eq!(m, 2);
/// assert_eq!(arr, vec![vec![0, 2], vec![3, 4], vec![5, 6]]);
/// assert_eq!(pair, (2, 2));
/// ```
///
/// Supporting types implementing [`Readable`], including:
/// - All integer and float types
/// - Tuples (up to 6 elements)
/// - [`String`]
/// - [`Vec<u8>`] (bytes)
/// - `[T; N]` (where `T` implements [`Readable`])
///
/// Vectors with runtime length are handled differently as `Vec<T>[N1; N2; N3; ...]`,
/// which would read `N1 * N2 * N3 * ...` `T` into a nested [`Vec`] with dimensions
/// `(N1, N2, N3, ...)`.
#[macro_export]
macro_rules! input {
    ($source:expr $(,)?) => {};
    (@vec $source:expr, Vec< $ty:ty >[ $size:tt ]) => {
        (0..$size).map(|_| <$ty as $crate::Readable>::read(&mut $source)).collect::<Vec<_>>()
    };
    (@vec $source:expr, Vec< $ty:ty >[ $size:expr; $($rest:tt);* ] ) => {
        (0..$size).map(|_| input!(@vec $source, Vec< $ty >[ $($rest);* ] )).collect::<Vec<_>>()
    };
    ($source:expr, $name:ident: Vec< $ty:ty >[ $($sizes:tt);* ]) => {
        let $name = input!(@vec $source, Vec< $ty >[ $($sizes);* ]);
    };
    ($source:expr, mut $name:ident: Vec< $ty:ty >[ $($sizes:tt);* ]) => {
        let mut $name = input!(@vec $source, Vec< $ty >[ $($sizes);* ]);
    };
    ($source:expr, $name:ident: Vec< $ty:ty >[ $($sizes:tt);* ], $($rest:tt)*) => {
        let $name = input!(@vec $source, Vec< $ty >[ $($sizes);* ]);
        input!($source, $($rest)*);
    };
    ($source:expr, mut $name:ident: Vec< $ty:ty >[ $($sizes:tt);* ], $($rest:tt)*) => {
        let mut $name = input!(@vec $source, Vec< $ty >[ $($sizes);* ]);
        input!($source, $($rest)*);
    };

    ($source:expr, $name:ident: $ty:ty) => {
        let $name = <$ty as $crate::Readable>::read(&mut $source);
    };
    ($source:expr, mut $name:ident: $ty:ty) => {
        let mut $name = <$ty as $crate::Readable>::read(&mut $source);
    };
    ($source:expr, $name:ident: $ty:ty, $($rest:tt)*) => {
        input!($source, $name:$ty);
        input!($source, $($rest)*);
    };
    ($source:expr, mut $name:ident: $ty:ty, $($rest:tt)*) => {
        input!($source, mut $name:$ty);
        input!($source, $($rest)*);
    };
}
