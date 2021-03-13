use std::error::Error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn Render(template_body: GoString, json_data: GoString) -> *const c_char;
}

#[repr(C)]
struct GoString {
    a: *const c_char,
    b: i64,
}

pub fn render(template_body: &str, json_data: &str) -> Result<String, Box<dyn Error>> {
    // Transform input values into Go strings
    let template_body_cstring = CString::new(template_body)?;
    let template_body_ptr = template_body_cstring.as_ptr();
    let template_body_gostring = GoString {
        a: template_body_ptr,
        b: template_body_cstring.as_bytes().len() as i64,
    };
    let json_data_cstring = CString::new(json_data)?;
    let json_data_ptr = json_data_cstring.as_ptr();
    let json_data_gostring = GoString {
        a: json_data_ptr,
        b: json_data_cstring.as_bytes().len() as i64,
    };

    // Render the Go template using FFI
    let rendered_ptr = unsafe { Render(template_body_gostring, json_data_gostring) };
    let rendered_cstr = unsafe { CStr::from_ptr(rendered_ptr) };
    let rendered = rendered_cstr.to_str()?;

    // Check the result for error strings and return
    match rendered.is_empty() || rendered.starts_with("error") {
        true => Err(rendered.into()),
        false => Ok(rendered.to_string()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
