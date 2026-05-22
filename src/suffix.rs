use crate::ffi;

/// Numeric suffixes that AMPL attaches to variables, constraints, and objectives.
///
/// Use these with the `dbl_suffix` / `int_suffix` methods on instance types.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Numericsuffix {
    /// Current value (`.val`).
    Value,
    /// Defining equation index (`.defeqn`).
    Defeqn,
    /// Dual / shadow price (`.dual`).
    Dual,
    /// Initial value (`.init`).
    Init,
    /// Original initial value before presolve (`.init0`).
    Init0,
    /// Lower bound (`.lb`).
    Lb,
    /// Upper bound (`.ub`).
    Ub,
    /// Original lower bound before presolve (`.lb0`).
    Lb0,
    /// Original upper bound before presolve (`.ub0`).
    Ub0,
    /// Computed lower bound (`.lb1`).
    Lb1,
    /// Computed upper bound (`.ub1`).
    Ub1,
    /// Relaxed lower bound (`.lb2`).
    Lb2,
    /// Relaxed upper bound (`.ub2`).
    Ub2,
    /// Left reduced cost (`.lrc`).
    Lrc,
    /// Right reduced cost (`.urc`).
    Urc,
    /// Left slack (`.lslack`).
    Lslack,
    /// Right slack (`.uslack`).
    Uslack,
    /// Reduced cost (`.rc`).
    Rc,
    /// Slack (`.slack`).
    Slack,
    // Constraint-specific suffixes
    /// Body value of a constraint (`.body`).
    Body,
    /// Defining variable index (`.defvar`).
    Defvar,
    /// Dual initial value (`.dinit`).
    Dinit,
    /// Original dual initial value (`.dinit0`).
    Dinit0,
    /// Single-sided lower bound of a constraint (`.lbs`).
    Lbs,
    /// Single-sided upper bound of a constraint (`.ubs`).
    Ubs,
    /// Lower dual value (`.ldual`).
    Ldual,
    /// Upper dual value (`.udual`).
    Udual,
    /// Logical constraint value (`.val`).
    Val,
    // Objective-specific suffixes
    /// Solver exit code (`.exitcode`).
    Exitcode,
    /// Any numeric suffix not explicitly listed above.
    Unknown(ffi::AMPL_NUMERICSUFFIX),
}

/// String suffixes that AMPL attaches to variables, constraints, and objectives.
///
/// Use these with the `string_suffix` method on instance types.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Stringsuffix {
    /// Algebraic status (`.astatus`).
    Astatus,
    /// Solver status (`.sstatus`).
    Sstatus,
    /// Combined status (`.status`).
    Status,
    /// Solver message (`.message`).
    Message,
    /// Solve result string (`.result`).
    Result,
    /// Objective sense: `"minimize"` or `"maximize"` (`.sense`).
    Sense,
    /// Any string suffix not explicitly listed above.
    Unknown(ffi::AMPL_STRINGSUFFIX),
}

//impl From<ffi::AMPL_NUMERICSUFFIX> for Numericsuffix {
    /// Converts an `AMPL_NUMERICSUFFIX` value to a `Numericsuffix` enum variant.
    //fn from(val: ffi::AMPL_NUMERICSUFFIX) -> Self {
        //match val {
            //ffi::AMPL_NUMERICSUFFIX_VALUE => Numericsuffix::Value,
            //ffi::AMPL_NUMERICSUFFIX_DEFEQN => Numericsuffix::Defeqn,
            //ffi::AMPL_NUMERICSUFFIX_DUAL => Numericsuffix::Dual,
            //ffi::AMPL_NUMERICSUFFIX_INIT => Numericsuffix::Init,
            //ffi::AMPL_NUMERICSUFFIX_INIT0 => Numericsuffix::Init0,
            //ffi::AMPL_NUMERICSUFFIX_LB => Numericsuffix::Lb,
            //ffi::AMPL_NUMERICSUFFIX_UB => Numericsuffix::Ub,
            //ffi::AMPL_NUMERICSUFFIX_LB0 => Numericsuffix::Lb0,
            //ffi::AMPL_NUMERICSUFFIX_UB0 => Numericsuffix::Ub0,
            //ffi::AMPL_NUMERICSUFFIX_LB1 => Numericsuffix::Lb1,
            //ffi::AMPL_NUMERICSUFFIX_UB1 => Numericsuffix::Ub1,
            //ffi::AMPL_NUMERICSUFFIX_LB2 => Numericsuffix::Lb2,
            ///ffi::AMPL_NUMERICSUFFIX_UB2 => Numericsuffix::Ub2,
            //ffi::AMPL_NUMERICSUFFIX_LRC => Numericsuffix::Lrc,
            //ffi::AMPL_NUMERICSUFFIX_URC => Numericsuffix::Urc,
            //ffi::AMPL_NUMERICSUFFIX_LSLACK => Numericsuffix::Lslack,
            //ffi::AMPL_NUMERICSUFFIX_USLACK => Numericsuffix::Uslack,
            //ffi::AMPL_NUMERICSUFFIX_RC => Numericsuffix::Rc,
            //ffi::AMPL_NUMERICSUFFIX_SLACK => Numericsuffix::Slack,
            // CONSTRAINTS
            //ffi::AMPL_NUMERICSUFFIX_BODY => Numericsuffix::Body,
            //ffi::AMPL_NUMERICSUFFIX_DEFVAR => Numericsuffix::Defvar,
            //ffi::AMPL_NUMERICSUFFIX_DINIT => Numericsuffix::Dinit,
            //ffi::AMPL_NUMERICSUFFIX_DINIT0 => Numericsuffix::Dinit0,
            //ffi::AMPL_NUMERICSUFFIX_LBS => Numericsuffix::Lbs,
            //ffi::AMPL_NUMERICSUFFIX_UBS => Numericsuffix::Ubs,
            //ffi::AMPL_NUMERICSUFFIX_LDUAL => Numericsuffix::Ldual,
            //ffi::AMPL_NUMERICSUFFIX_UDUAL => Numericsuffix::Udual,
            //ffi::AMPL_NUMERICSUFFIX_VAL => Numericsuffix::Val,  // for logical constraints
            // OBJECTIVES
            //ffi::AMPL_NUMERICSUFFIX_EXITCODE => Numericsuffix::Exitcode,
            //val => Numericsuffix::Unknown(val),
        //}
    //}
//}

impl From<Numericsuffix> for ffi::AMPL_NUMERICSUFFIX {
    fn from(value: Numericsuffix) -> Self {
        match value {
            Numericsuffix::Value => ffi::AMPL_NUMERICSUFFIX_AMPL_VALUE,
            Numericsuffix::Defeqn => ffi::AMPL_NUMERICSUFFIX_AMPL_DEFEQN,
            Numericsuffix::Dual => ffi::AMPL_NUMERICSUFFIX_AMPL_DUAL,
            Numericsuffix::Init => ffi::AMPL_NUMERICSUFFIX_AMPL_INIT,
            Numericsuffix::Init0 => ffi::AMPL_NUMERICSUFFIX_AMPL_INIT0,
            Numericsuffix::Lb => ffi::AMPL_NUMERICSUFFIX_AMPL_LB,
            Numericsuffix::Ub => ffi::AMPL_NUMERICSUFFIX_AMPL_UB,
            Numericsuffix::Lb0 => ffi::AMPL_NUMERICSUFFIX_AMPL_LB0,
            Numericsuffix::Ub0 => ffi::AMPL_NUMERICSUFFIX_AMPL_UB0,
            Numericsuffix::Lb1 => ffi::AMPL_NUMERICSUFFIX_AMPL_LB1,
            Numericsuffix::Ub1 => ffi::AMPL_NUMERICSUFFIX_AMPL_UB1,
            Numericsuffix::Lb2 => ffi::AMPL_NUMERICSUFFIX_AMPL_LB2,
            Numericsuffix::Ub2 => ffi::AMPL_NUMERICSUFFIX_AMPL_UB2,
            Numericsuffix::Lrc => ffi::AMPL_NUMERICSUFFIX_AMPL_LRC,
            Numericsuffix::Urc => ffi::AMPL_NUMERICSUFFIX_AMPL_URC,
            Numericsuffix::Lslack => ffi::AMPL_NUMERICSUFFIX_AMPL_LSLACK,
            Numericsuffix::Uslack => ffi::AMPL_NUMERICSUFFIX_AMPL_USLACK,
            Numericsuffix::Rc => ffi::AMPL_NUMERICSUFFIX_AMPL_RC,
            Numericsuffix::Slack => ffi::AMPL_NUMERICSUFFIX_AMPL_SLACK,
            Numericsuffix::Body => ffi::AMPL_NUMERICSUFFIX_AMPL_BODY,
            Numericsuffix::Defvar => ffi::AMPL_NUMERICSUFFIX_AMPL_DEFVAR,
            Numericsuffix::Dinit => ffi::AMPL_NUMERICSUFFIX_AMPL_DINIT,
            Numericsuffix::Dinit0 => ffi::AMPL_NUMERICSUFFIX_AMPL_DINIT0,
            Numericsuffix::Lbs => ffi::AMPL_NUMERICSUFFIX_AMPL_LBS,
            Numericsuffix::Ubs => ffi::AMPL_NUMERICSUFFIX_AMPL_UBS,
            Numericsuffix::Ldual => ffi::AMPL_NUMERICSUFFIX_AMPL_LDUAL,
            Numericsuffix::Udual => ffi::AMPL_NUMERICSUFFIX_AMPL_UDUAL,
            Numericsuffix::Val => ffi::AMPL_NUMERICSUFFIX_AMPL_VAL,
            Numericsuffix::Exitcode => ffi::AMPL_NUMERICSUFFIX_AMPL_EXITCODE,
            Numericsuffix::Unknown(val) => val,
        }
    }
}

impl From<Stringsuffix> for ffi::AMPL_STRINGSUFFIX {
    fn from(value: Stringsuffix) -> Self {
        match value {
            Stringsuffix::Astatus => ffi::AMPL_STRINGSUFFIX_AMPL_ASTATUS,
            Stringsuffix::Sstatus => ffi::AMPL_STRINGSUFFIX_AMPL_SSTATUS,
            Stringsuffix::Status => ffi::AMPL_STRINGSUFFIX_AMPL_STATUS,
            Stringsuffix::Message => ffi::AMPL_STRINGSUFFIX_AMPL_MESSAGE,
            Stringsuffix::Result => ffi::AMPL_STRINGSUFFIX_AMPL_RESULT,
            Stringsuffix::Sense => ffi::AMPL_STRINGSUFFIX_AMPL_SENSE,
            Stringsuffix::Unknown(val) => val,
        }
    }
}
