use super::graph::{Node, SignalRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FilterMode {
    LowPass2,
    HighPass2,
    BandPass2,
    LowPass4,
}

macro_rules! count_inputs {
    () => (0usize);
    ($head:ident $($tail:ident)*) => (1usize + count_inputs!($($tail)*));
}

macro_rules! op {
    ($name:ident, [], []) => {
        #[derive(Debug)]
        pub struct $name;
        impl Node for $name {
            fn inputs(&self) -> &[SignalRef] {
                &[]
            }
        }
    };
    ($name:ident, [], [$($pname:ident: $ptype:ty),*]) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $pname: $ptype),*
        }
        impl Node for $name {
            fn inputs(&self) -> &[SignalRef] {
                &[]
            }
        }
    };
    ($name:ident, [$($input:ident),*], [$($pname:ident: $ptype:ty),*]) => {
        #[derive(Debug)]
        pub struct $name {
            pub inputs: [SignalRef; count_inputs!($($input)*)],
            $(pub $pname: $ptype),*
        }
        impl Node for $name {
            fn inputs(&self) -> &[SignalRef] {
                &self.inputs[..]
            }
        }
    };
    ($name:ident, [$($input:ident),*], [$($pname:ident: $ptype:ty),*],) => {
        op!($name, [$($input),*], [$($pname: $ptype),*]);
    };
    ($name:ident, [$($input:ident),*],) => {
        op!($name, [$($input),*], []);
    };
    ($name:ident, [$($input:ident),*]) => {
        op!($name, [$($input),*], []);
    };
}

// Oscillators and generators
op!(Oscillator, [frequency]);
op!(Sawtooth, [phase]);
op!(Sine, [phase]);
op!(Noise, []);

// Filters
op!(HighPass, [input], [frequency: f64]);
op!(
    StateVariableFilter,
    [input, frequency],
    [mode: FilterMode, q: f64],
);

// Distortion
op!(Saturate, [input]);
op!(Rectify, [input]);

// Envelopes
#[derive(Debug, Clone, Copy)]
pub enum EnvelopeSegment {
    Set(f64),
    Lin(f64, f64),
    Exp(f64, f64),
    Delay(f64),
    Gate,
    Stop,
}

#[derive(Debug)]
pub struct Envelope(pub Box<[EnvelopeSegment]>);

impl Node for Envelope {
    fn inputs(&self) -> &[SignalRef] {
        &[]
    }
}

// Utilities
op!(Multiply, [x, y]);
op!(Constant, [], [value: f64]);
op!(Frequency, [input]);
op!(Mix, [base, input], [gain: f64]);
op!(Zero, []);
op!(ScaleInt, [input], [scale: i32]);

/*
// Parameter references
op!(Parameter, 0, 0, 1); // -> deref and derefcopy
op!(Note, 1, 0, 1);
 */
op!(Note, [], [offset: i32]);