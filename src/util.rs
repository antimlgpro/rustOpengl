use std::error::Error;
use std::ffi::CString;
use std::fs;

pub fn load_to_string(path: &str) -> Result<String, Box<dyn Error>> {
	let content = fs::read_to_string(path)?;

	Ok(content)
}

pub fn to_cstring(s: String) -> Result<CString, Box<dyn Error>> {
	let cst = CString::new(s.as_bytes())?;
	Ok(cst)
}
