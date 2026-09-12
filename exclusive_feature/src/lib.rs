
#[cfg(feature = "int_are_32_bits")]
mod integer_typedef32
{
    /// Default unsigned integer type
    #[allow(non_camel_case_types)]
    pub type uint = u32;

    /// Default signed integer type
    #[allow(non_camel_case_types)]
    pub type int = i32;
}

#[cfg(feature = "int_are_32_bits")]
pub use integer_typedef32::*;


#[cfg(feature = "int_are_64_bits")]
mod integer_typedef64
{
    /// Default unsigned integer type
    #[allow(non_camel_case_types)]
    pub type uint = u64;

    /// Default signed integer type
    #[allow(non_camel_case_types)]
    pub type int = i64;
}

#[cfg(feature = "int_are_64_bits")]
pub use integer_typedef64::*;



// None enabled
#[cfg(not(any(
    feature = "int_are_32_bits",
    feature = "int_are_64_bits",
)))]
compile_error!(
    "Missing one of the following features: \
    `int_are_32_bits`, or `int_are_64_bits`."
);

// More than one enabled
#[cfg(any(
    all(feature = "int_are_32_bits", feature = "int_are_64_bits"),
))]
compile_error!(
    "Multiple int size features enabled. Please enable only one of: \
     `int_are_32_bits`, or `int_are_64_bits`."
);