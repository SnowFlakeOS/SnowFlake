use uefi::boot_services::protocols;
use uefi::boot_services::BootServices;
use uefi::{CStr16,
           Status};
use uefi::status::NOT_FOUND;

use PATH_FALLBACK_KERNEL;

pub struct Configuration<'bs>
{
	pub kernel: ::uefi::borrow::Cow<'bs, 'static, CStr16>,
	//commandline: ::uefi::borrow::Cow<'bs, 'static, str>,
}

impl<'bs> Configuration<'bs>
{
	pub fn from_file(_bs: &'bs BootServices, sys_vol: &protocols::File, filename: &CStr16) -> Result<Configuration<'bs>, Status> {
		match sys_vol.open_read(filename) {
		    Ok(_cfg_file) => {
			    //panic!("TODO: Read config file (allocating strings with `bs`)");
				Ok(Configuration {
				    kernel: ::uefi::CStr16::from_slice(PATH_FALLBACK_KERNEL).into(),
				    //commandline: "".into(),
				})
			},
		    Err(NOT_FOUND) => {
			    Ok(Configuration {
				    kernel: ::uefi::CStr16::from_slice(PATH_FALLBACK_KERNEL).into(),
				    //commandline: "".into(),
				})
			},
		    Err(e) => Err(e),
		}
	}
}
