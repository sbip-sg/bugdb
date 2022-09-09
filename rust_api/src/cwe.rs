//! Module containing detailed [CWE classification](https://cwe.mitre.org/) and
//! information of bugs in programsx.

use phf;

/// Mapping each CWE's ID to its detailed information.
///
/// Details information of all CWEs are obtained from the
/// [Mitre's](https://cwe.mitre.org/index.html) CWE website.
static CWE_INFO_DB: phf::Map<u32, &str> = phf::phf_map! {
    119u32 => "Improper Restriction of Operations within the Bounds of \
               a Memory Buffer",

    121u32 => "Stack-based Buffer Overflow",

    122u32 => "Heap-based Buffer Overflow",

    125u32 => "Out-of-bounds Read",

    190u32 => "Integer Overflow or Wraparound",

    191u32 => "Integer Underflow (Wrap or Wraparound)",

    354u32 => "Improper Validation of Integrity Check Value",

    369u32 => "Division By Zero>",

    401u32 => "Missing Release of Memory after Effective Lifetime",

    476u32 => "NULL Pointer Dereference",

    681u32 => "Incorrect Conversion between Numeric Types",

    682u32 => "Incorrect Calculation",

    704u32 => "Incorrect Type Conversion or Cast",

    754u32 => "Improper Check for Unusual or Exceptional Conditions",

    772u32 => "Missing Release of Resource after Effective Lifetime",

    787u32 => "Out-of-bounds Write",

    788u32 => "Access of Memory Location After End of Buffer",
};

/// Get detailed information of a CWE by ID
pub fn get_cwe_info(id: u32) -> String {
    match CWE_INFO_DB.get(&id) {
        Some(info) => info.to_string(),
        None => format!("CWE-{}: information not found", id),
    }
}
