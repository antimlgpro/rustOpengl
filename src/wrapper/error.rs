use std::{fmt, fmt::Display};

#[derive(Debug)]
pub enum GLError {
	InvalidEnum,
	InvalidValue,
	InvalidOperation,
	StackOverflow,
	StackUnderflow,
	OutOfMemory,
	InvalidFramebufferOperation,
	ContextLost,
}

impl Display for GLError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			GLError::InvalidEnum => write!(f, "InvalidEnum"),
			GLError::InvalidValue => write!(f, "InvalidValue"),
			GLError::InvalidOperation => write!(f, "InvalidOperation"),
			GLError::StackOverflow => write!(f, "StackOverflow"),
			GLError::StackUnderflow => write!(f, "StackUnderflow"),
			GLError::OutOfMemory => write!(f, "OutOfMemory"),
			GLError::InvalidFramebufferOperation => write!(f, "InvalidFramebufferOperation"),
			GLError::ContextLost => write!(f, "ContextLost"),
		}
	}
}

pub fn get_error() -> Option<GLError> {
	unsafe {
		let err = gl::GetError();

		match err {
			gl::INVALID_ENUM => return Some(GLError::InvalidEnum),
			gl::INVALID_VALUE => return Some(GLError::InvalidValue),
			gl::INVALID_OPERATION => return Some(GLError::InvalidOperation),
			gl::STACK_OVERFLOW => return Some(GLError::StackOverflow),
			gl::STACK_UNDERFLOW => return Some(GLError::StackUnderflow),
			gl::OUT_OF_MEMORY => return Some(GLError::OutOfMemory),
			gl::INVALID_FRAMEBUFFER_OPERATION => return Some(GLError::InvalidFramebufferOperation),
			gl::CONTEXT_LOST => return Some(GLError::ContextLost),
			_ => return None,
		}
	}
}
