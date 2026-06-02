use crate::BsuiteCoreError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ExitCode {
    Success,
    Finding,
    InternalError,
    Usage,
}

impl ExitCode {
    pub const ALL: [Self; 4] = [
        Self::Success,
        Self::Finding,
        Self::InternalError,
        Self::Usage,
    ];

    pub const fn as_i32(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::Finding => 1,
            Self::InternalError => 2,
            Self::Usage => 64,
        }
    }
}

pub trait ExitCodeEmitter {
    fn emit(&self, code: ExitCode) -> Result<(), BsuiteCoreError>;
}
