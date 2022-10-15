use std::os::unix::ffi::OsStringExt;
use std::{
    env,
    ffi::{CStr, OsStr, OsString},
    mem,
    path::PathBuf,
    ptr,
};

use std::collections::HashMap;
use users::{get_current_username, get_user_by_name};

pub enum InterpolationUnit {
    String(String),
    Expression(Box<dyn Fn() -> String>),
}

pub struct Interpolator {
    units: HashMap<String, InterpolationUnit>,
}

impl Interpolator {
    pub fn new() -> Self {
        Self {
            units: Default::default(),
        }
    }

    pub fn add_unit(&mut self, name: &str, unit: InterpolationUnit) {
        if self.units.contains_key(name) {
            unimplemented!();
        }

        self.units.insert(name.to_owned(), unit);
    }

    pub fn interpolate(&self, string: &str) -> String {
        let mut interpolated_string = String::new();
        let mut current_unit = String::new();

        let mut in_unit = false;

        for char in string.chars() {
            match char {
                '<' => in_unit = true,
                '>' => {
                    in_unit = false;

                    let unit = self.units.get(&current_unit[..]);

                    if let Some(expr) = unit {
                        let result = match expr {
                            InterpolationUnit::String(item) => item.clone(),
                            InterpolationUnit::Expression(item) => item(),
                        };

                        interpolated_string.push_str(&result[..]);
                        current_unit.clear();
                    } else {
                        unimplemented!();
                    }
                },
                _ => {
                    if in_unit {
                        current_unit.push(char);
                    } else {
                        interpolated_string.push(char);
                    }
                }
            }
        }

        interpolated_string
    }
}

pub fn get_interpolator(dotfiles_dir: &PathBuf) -> Interpolator {
    let mut interpolator = Interpolator::new();

    interpolator.add_unit(
        "DOTFILES_ROOT",
        InterpolationUnit::String(dotfiles_dir.to_str().unwrap().to_owned()),
    );
    interpolator.add_unit(
        "HOME",
        InterpolationUnit::Expression(Box::new(|| {
            let user = env::var("SUDO_USER")
                .unwrap_or_else(|_| get_current_username().unwrap().into_string().unwrap());
            let user = get_user_by_name(&OsStr::new(&user)).unwrap();

            let dir = home_dir(user.uid())
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap();

            dir
        })),
    );

    interpolator
}

fn home_dir(uid: u32) -> Option<PathBuf> {
    let path: Option<OsString> = unsafe {
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = mem::zeroed();
        let mut result = ptr::null_mut();
        match libc::getpwuid_r(
            uid,
            &mut passwd,
            buf.as_mut_ptr(),
            buf.capacity(),
            &mut result,
        ) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_dir as *const _;
                let bytes = CStr::from_ptr(ptr).to_bytes();
                if bytes.is_empty() {
                    None
                } else {
                    Some(OsStringExt::from_vec(bytes.to_vec()))
                }
            }
            _ => None,
        }
    };

    path.map(PathBuf::from)
}
