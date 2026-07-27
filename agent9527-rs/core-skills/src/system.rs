pub(crate) use agent9527_skills::install_system_skills;
pub(crate) use agent9527_skills::system_cache_root_dir;

use agent9527_utils_absolute_path::AbsolutePathBuf;

pub(crate) fn uninstall_system_skills(agent9527_home: &AbsolutePathBuf) {
    let _ = std::fs::remove_dir_all(system_cache_root_dir(agent9527_home));
}
