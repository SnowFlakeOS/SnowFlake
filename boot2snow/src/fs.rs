// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

use alloc::vec::Vec;
use core::{mem, slice};
use uefi::boot_services::LocateSearchType;
use uefi::boot_services::protocols::{File as InnerFile, SimpleFileSystem, Protocol};
use uefi::boot_services::protocols::file::{FileInfo, FILE_MODE_READ};
use uefi::{Void, Guid, Handle, FILE_SYSTEM_GUID};
use uefi::status::Status;
use uefi::status::NOT_FOUND;

use string::wstr;
use get_boot_services;

pub struct FileSystem(pub &'static mut SimpleFileSystem);

impl Protocol for FileSystem {
    fn guid() -> Guid {
        FILE_SYSTEM_GUID
    }

	unsafe fn from_ptr(v: *const Void) -> *const Self {
		v as *const _
    }
}

impl FileSystem {
    pub fn root(&mut self) -> Result<Dir, Status> {
        let mut interface = 0 as *mut InnerFile;
        unsafe { (self.0.open_volume)(self.0, &mut interface)? };

        Ok(Dir(File(unsafe { &mut *interface })))
    }

	pub fn handle_protocol(handle: &Handle) -> Result<Self, Status> {
        let boot_services = get_boot_services();

		let mut ptr = 0 as *mut Void;
		// SAFE: Pointer cannot cause unsafety
		unsafe { (boot_services.handle_protocol)(*handle, &Self::guid(), &mut ptr) }
			.err_or_else( || unsafe { FileSystem(&mut *(ptr as *mut SimpleFileSystem)) } )
	}

    pub fn locate_handle() -> Result<Vec<Self>, Status> where Self: Sized {
        let boot_services = get_boot_services();

        let guid = Self::guid();
        let mut handles = Vec::with_capacity(256);
        let mut len = handles.capacity() * mem::size_of::<Handle>();
        unsafe { (boot_services.locate_handle)(LocateSearchType::ByProtocol, Some(&guid), 0 as *mut Void, &mut len, handles.as_mut_ptr())? };
        unsafe { handles.set_len(len / mem::size_of::<Handle>()); }

        let mut instances = Vec::new();
        for handle in handles {
            if let Ok(instance) = Self::handle_protocol(&handle) {
                instances.push(instance);
            }
        }

        Ok(instances)
    }

    pub fn all() -> Vec<Self> where Self: Sized {
        Self::locate_handle().unwrap_or(Vec::new())
    }
}

pub struct File(pub &'static mut InnerFile);

impl File {
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Status> {
        let mut len = buf.len();
        unsafe { (self.0.read)(self.0, &mut len, buf.as_mut_ptr() as *mut Void)? };
        Ok(len)
    }

    pub fn read_to_end(&mut self, vec: &mut Vec<u8>) -> Result<usize, Status> {
        let mut total = 0;
        let mut buf = [0; 8192];

        while let Some(count) = Some(self.read(&mut buf)?) {
            if count == 0 { break; }

            vec.extend(&buf[.. count]);
            total += count;
        }

        Ok(total)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Status> {
        let mut len = buf.len();
        unsafe { (self.0.write)(self.0, &mut len, buf.as_ptr() as *mut Void)? };
        Ok(len)
    }

    pub fn get_position(&mut self) -> Result<u64, Status> {
        let mut pos = 0;
		unsafe { (self.0.get_position)(self.0, &mut pos)? };
        Ok(pos)
    }

    pub fn set_position(&mut self, pos: u64) {
        unsafe { (self.0.set_position)(self.0, pos) };
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = unsafe { (self.0.close)(self.0) };
    }
}

pub struct Dir(pub File);

impl Dir {
    pub fn open(&mut self, filename: &[u16]) -> Result<File, Status> {
        let mut interface = 0 as *mut InnerFile;
        unsafe { ((self.0).0.open)((self.0).0, &mut interface, filename.as_ptr(), FILE_MODE_READ, 0)? };

        Ok(File(unsafe { &mut *interface }))
    }

    pub fn open_dir(&mut self, filename: &[u16]) -> Result<Dir, Status> {
        let file = self.open(filename)?;
        Ok(Dir(file))
    }

    pub fn read(&mut self) -> Result<Option<FileInfo>, Status> {
        let mut info = FileInfo::default();
        let buf = unsafe {
            slice::from_raw_parts_mut(
                &mut info as *mut _ as *mut u8,
                mem::size_of_val(&info)
            )
        };
        match self.0.read(buf) {
            Ok(0) => Ok(None),
            Ok(_len) => Ok(Some(info)),
            Err(err) => Err(err)
        }
    }
}

pub fn find(path: &str) -> Result<(usize, File), Status> {
    let wpath = wstr(path);

    for (i, mut fs) in FileSystem::all().iter_mut().enumerate() {
        let mut root = fs.root()?;
        match root.open(&wpath) {
            Ok(file) => {
                return Ok((i, file));
            },
            Err(err) => if err != NOT_FOUND {
                return Err(err);
            }
        }
    }

    Err(NOT_FOUND)
}

pub fn load(path: &str) -> Result<Vec<u8>, Status> {
    let (_, mut file) = find(path)?;

    let mut data = vec![];
    let _ = file.read_to_end(&mut data)?;

    Ok(data)
}