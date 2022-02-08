use gl::types::*;
use std::{
	ffi::{c_void, CStr},
	fmt,
	fmt::Display,
};

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

pub type ShaderError = String;

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
		return match_error(err);
	}
}

pub fn match_error(err: GLenum) -> Option<GLError> {
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

pub extern "system" fn error_callback(
	source_: GLenum,
	gltype_: GLenum,
	id: GLuint,
	severity_: GLenum,
	_length: GLsizei,
	message_: *const GLchar,
	_user_param: *mut c_void,
) {
	if id == 131_169 || id == 131_185 || id == 131_218 || id == 131_204 {
		// ignore these non-significant error codes
		return;
	}
	let message = unsafe { CStr::from_ptr(message_).to_string_lossy() };

	let source = match source_ {
		gl::DEBUG_SOURCE_API => "API",
		gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
		gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
		gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
		gl::DEBUG_SOURCE_APPLICATION => "Application",
		gl::DEBUG_SOURCE_OTHER => "Other",
		_ => "Unknown enum value",
	};

	let gltype = match gltype_ {
		gl::DEBUG_TYPE_ERROR => "Error",
		gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behaviour",
		gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behaviour",
		gl::DEBUG_TYPE_PORTABILITY => "Portability",
		gl::DEBUG_TYPE_PERFORMANCE => "Performance",
		gl::DEBUG_TYPE_MARKER => "Marker",
		gl::DEBUG_TYPE_PUSH_GROUP => "Push Group",
		gl::DEBUG_TYPE_POP_GROUP => "Pop Group",
		gl::DEBUG_TYPE_OTHER => "Other",
		_ => "Unknown enum value",
	};

	let severity = match severity_ {
		gl::DEBUG_SEVERITY_HIGH => "high",
		gl::DEBUG_SEVERITY_MEDIUM => "medium",
		gl::DEBUG_SEVERITY_LOW => "low",
		gl::DEBUG_SEVERITY_NOTIFICATION => "notification",
		_ => "Unknown enum value",
	};
	eprintln!(
		"Gl error: ({}), Source: {}, Type: {}, Severity: {}\nMessage: {}",
		id, source, gltype, severity, message
	);
}
