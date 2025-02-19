// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Deployment;
#[cfg(any(feature = "v2016_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2016_4")))]
use crate::DeploymentUnlockedState;
#[cfg(any(feature = "v2017_7", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_7")))]
use crate::Repo;
#[cfg(any(feature = "v2020_7", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_7")))]
use crate::SysrootDeployTreeOpts;
use crate::SysrootSimpleWriteDeploymentFlags;
#[cfg(any(feature = "v2017_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_4")))]
use crate::SysrootWriteDeploymentsOpts;
use glib::object::IsA;
#[cfg(any(feature = "v2017_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_10")))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v2017_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_10")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2017_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_10")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
#[cfg(any(feature = "v2017_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_10")))]
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "OstreeSysroot")]
    pub struct Sysroot(Object<ffi::OstreeSysroot>);

    match fn {
        type_ => || ffi::ostree_sysroot_get_type(),
    }
}

impl Sysroot {
    #[doc(alias = "ostree_sysroot_new")]
    pub fn new<P: IsA<gio::File>>(path: Option<&P>) -> Sysroot {
        unsafe {
            from_glib_full(ffi::ostree_sysroot_new(path.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sysroot_new_default")]
    pub fn new_default() -> Sysroot {
        unsafe {
            from_glib_full(ffi::ostree_sysroot_new_default())
        }
    }

    #[doc(alias = "ostree_sysroot_cleanup")]
    pub fn cleanup<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_cleanup(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //#[cfg(any(feature = "v2018_6", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_6")))]
    //#[doc(alias = "ostree_sysroot_cleanup_prune_repo")]
    //pub fn cleanup_prune_repo<P: IsA<gio::Cancellable>>(&self, options: /*Ignored*/&mut RepoPruneOptions, cancellable: Option<&P>) -> Result<(i32, i32, u64), glib::Error> {
    //    unsafe { TODO: call ffi:ostree_sysroot_cleanup_prune_repo() }
    //}

    #[cfg(any(feature = "v2018_5", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_5")))]
    #[doc(alias = "ostree_sysroot_deploy_tree")]
    pub fn deploy_tree<P: IsA<gio::Cancellable>>(&self, osname: Option<&str>, revision: &str, origin: Option<&glib::KeyFile>, provided_merge_deployment: Option<&Deployment>, override_kernel_argv: &[&str], cancellable: Option<&P>) -> Result<Deployment, glib::Error> {
        unsafe {
            let mut out_new_deployment = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_deploy_tree(self.to_glib_none().0, osname.to_glib_none().0, revision.to_glib_none().0, origin.to_glib_none().0, provided_merge_deployment.to_glib_none().0, override_kernel_argv.to_glib_none().0, &mut out_new_deployment, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(out_new_deployment)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2020_7", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_7")))]
    #[doc(alias = "ostree_sysroot_deploy_tree_with_options")]
    pub fn deploy_tree_with_options<P: IsA<gio::Cancellable>>(&self, osname: Option<&str>, revision: &str, origin: Option<&glib::KeyFile>, provided_merge_deployment: Option<&Deployment>, opts: Option<&SysrootDeployTreeOpts>, cancellable: Option<&P>) -> Result<Deployment, glib::Error> {
        unsafe {
            let mut out_new_deployment = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_deploy_tree_with_options(self.to_glib_none().0, osname.to_glib_none().0, revision.to_glib_none().0, origin.to_glib_none().0, provided_merge_deployment.to_glib_none().0, mut_override(opts.to_glib_none().0), &mut out_new_deployment, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(out_new_deployment)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_deployment_set_kargs")]
    pub fn deployment_set_kargs<P: IsA<gio::Cancellable>>(&self, deployment: &Deployment, new_kargs: &[&str], cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_deployment_set_kargs(self.to_glib_none().0, deployment.to_glib_none().0, new_kargs.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_deployment_set_kargs_in_place")]
    pub fn deployment_set_kargs_in_place<P: IsA<gio::Cancellable>>(&self, deployment: &Deployment, kargs_str: &str, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_deployment_set_kargs_in_place(self.to_glib_none().0, deployment.to_glib_none().0, kargs_str.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_deployment_set_mutable")]
    pub fn deployment_set_mutable<P: IsA<gio::Cancellable>>(&self, deployment: &Deployment, is_mutable: bool, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_deployment_set_mutable(self.to_glib_none().0, deployment.to_glib_none().0, is_mutable.into_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2018_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_3")))]
    #[doc(alias = "ostree_sysroot_deployment_set_pinned")]
    pub fn deployment_set_pinned(&self, deployment: &Deployment, is_pinned: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_deployment_set_pinned(self.to_glib_none().0, deployment.to_glib_none().0, is_pinned.into_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2016_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2016_4")))]
    #[doc(alias = "ostree_sysroot_deployment_unlock")]
    pub fn deployment_unlock<P: IsA<gio::Cancellable>>(&self, deployment: &Deployment, unlocked_state: DeploymentUnlockedState, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_deployment_unlock(self.to_glib_none().0, deployment.to_glib_none().0, unlocked_state.into_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_ensure_initialized")]
    pub fn ensure_initialized<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_ensure_initialized(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_get_booted_deployment")]
    #[doc(alias = "get_booted_deployment")]
    pub fn booted_deployment(&self) -> Option<Deployment> {
        unsafe {
            from_glib_none(ffi::ostree_sysroot_get_booted_deployment(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sysroot_get_bootversion")]
    #[doc(alias = "get_bootversion")]
    pub fn bootversion(&self) -> i32 {
        unsafe {
            ffi::ostree_sysroot_get_bootversion(self.to_glib_none().0)
        }
    }

    #[doc(alias = "ostree_sysroot_get_deployment_directory")]
    #[doc(alias = "get_deployment_directory")]
    pub fn deployment_directory(&self, deployment: &Deployment) -> Option<gio::File> {
        unsafe {
            from_glib_full(ffi::ostree_sysroot_get_deployment_directory(self.to_glib_none().0, deployment.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sysroot_get_deployment_dirpath")]
    #[doc(alias = "get_deployment_dirpath")]
    pub fn deployment_dirpath(&self, deployment: &Deployment) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::ostree_sysroot_get_deployment_dirpath(self.to_glib_none().0, deployment.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sysroot_get_deployments")]
    #[doc(alias = "get_deployments")]
    pub fn deployments(&self) -> Vec<Deployment> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::ostree_sysroot_get_deployments(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sysroot_get_fd")]
    #[doc(alias = "get_fd")]
    pub fn fd(&self) -> i32 {
        unsafe {
            ffi::ostree_sysroot_get_fd(self.to_glib_none().0)
        }
    }

    #[doc(alias = "ostree_sysroot_get_merge_deployment")]
    #[doc(alias = "get_merge_deployment")]
    pub fn merge_deployment(&self, osname: Option<&str>) -> Option<Deployment> {
        unsafe {
            from_glib_full(ffi::ostree_sysroot_get_merge_deployment(self.to_glib_none().0, osname.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sysroot_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::ostree_sysroot_get_path(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2018_5", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_5")))]
    #[doc(alias = "ostree_sysroot_get_staged_deployment")]
    #[doc(alias = "get_staged_deployment")]
    pub fn staged_deployment(&self) -> Option<Deployment> {
        unsafe {
            from_glib_none(ffi::ostree_sysroot_get_staged_deployment(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sysroot_get_subbootversion")]
    #[doc(alias = "get_subbootversion")]
    pub fn subbootversion(&self) -> i32 {
        unsafe {
            ffi::ostree_sysroot_get_subbootversion(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2016_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2016_4")))]
    #[doc(alias = "ostree_sysroot_init_osname")]
    pub fn init_osname<P: IsA<gio::Cancellable>>(&self, osname: &str, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_init_osname(self.to_glib_none().0, osname.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2020_1", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_1")))]
    #[doc(alias = "ostree_sysroot_initialize")]
    pub fn initialize(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_initialize(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2020_1", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_1")))]
    #[doc(alias = "ostree_sysroot_is_booted")]
    pub fn is_booted(&self) -> bool {
        unsafe {
            from_glib(ffi::ostree_sysroot_is_booted(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sysroot_load")]
    pub fn load<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_load(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2016_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2016_4")))]
    #[doc(alias = "ostree_sysroot_load_if_changed")]
    pub fn load_if_changed<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> Result<bool, glib::Error> {
        unsafe {
            let mut out_changed = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_load_if_changed(self.to_glib_none().0, out_changed.as_mut_ptr(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            let out_changed = out_changed.assume_init();
            if error.is_null() { Ok(from_glib(out_changed)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_lock")]
    pub fn lock(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_lock(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_lock_async")]
    pub fn lock_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn lock_async_trampoline<Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_lock_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lock_async_trampoline::<Q>;
        unsafe {
            ffi::ostree_sysroot_lock_async(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    pub fn lock_async_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.lock_async(
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "ostree_sysroot_origin_new_from_refspec")]
    pub fn origin_new_from_refspec(&self, refspec: &str) -> Option<glib::KeyFile> {
        unsafe {
            from_glib_full(ffi::ostree_sysroot_origin_new_from_refspec(self.to_glib_none().0, refspec.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_sysroot_prepare_cleanup")]
    pub fn prepare_cleanup<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_prepare_cleanup(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2017_7", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_7")))]
    #[doc(alias = "ostree_sysroot_query_deployments_for")]
    pub fn query_deployments_for(&self, osname: Option<&str>) -> (Option<Deployment>, Option<Deployment>) {
        unsafe {
            let mut out_pending = ptr::null_mut();
            let mut out_rollback = ptr::null_mut();
            ffi::ostree_sysroot_query_deployments_for(self.to_glib_none().0, osname.to_glib_none().0, &mut out_pending, &mut out_rollback);
            (from_glib_full(out_pending), from_glib_full(out_rollback))
        }
    }

    #[cfg(any(feature = "v2017_7", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_7")))]
    #[doc(alias = "ostree_sysroot_repo")]
    pub fn repo(&self) -> Option<Repo> {
        unsafe {
            from_glib_none(ffi::ostree_sysroot_repo(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2021_1", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2021_1")))]
    #[doc(alias = "ostree_sysroot_require_booted_deployment")]
    pub fn require_booted_deployment(&self) -> Result<Deployment, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ostree_sysroot_require_booted_deployment(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2020_1", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_1")))]
    #[doc(alias = "ostree_sysroot_set_mount_namespace_in_use")]
    pub fn set_mount_namespace_in_use(&self) {
        unsafe {
            ffi::ostree_sysroot_set_mount_namespace_in_use(self.to_glib_none().0);
        }
    }

    #[doc(alias = "ostree_sysroot_simple_write_deployment")]
    pub fn simple_write_deployment<P: IsA<gio::Cancellable>>(&self, osname: Option<&str>, new_deployment: &Deployment, merge_deployment: Option<&Deployment>, flags: SysrootSimpleWriteDeploymentFlags, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_simple_write_deployment(self.to_glib_none().0, osname.to_glib_none().0, new_deployment.to_glib_none().0, merge_deployment.to_glib_none().0, flags.into_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2020_7", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_7")))]
    #[doc(alias = "ostree_sysroot_stage_overlay_initrd")]
    pub fn stage_overlay_initrd<P: IsA<gio::Cancellable>>(&self, fd: i32, cancellable: Option<&P>) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut out_checksum = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_stage_overlay_initrd(self.to_glib_none().0, fd, &mut out_checksum, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(out_checksum)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2018_5", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_5")))]
    #[doc(alias = "ostree_sysroot_stage_tree")]
    pub fn stage_tree<P: IsA<gio::Cancellable>>(&self, osname: Option<&str>, revision: &str, origin: Option<&glib::KeyFile>, merge_deployment: Option<&Deployment>, override_kernel_argv: &[&str], cancellable: Option<&P>) -> Result<Deployment, glib::Error> {
        unsafe {
            let mut out_new_deployment = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_stage_tree(self.to_glib_none().0, osname.to_glib_none().0, revision.to_glib_none().0, origin.to_glib_none().0, merge_deployment.to_glib_none().0, override_kernel_argv.to_glib_none().0, &mut out_new_deployment, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(out_new_deployment)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2020_7", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2020_7")))]
    #[doc(alias = "ostree_sysroot_stage_tree_with_options")]
    pub fn stage_tree_with_options<P: IsA<gio::Cancellable>>(&self, osname: Option<&str>, revision: &str, origin: Option<&glib::KeyFile>, merge_deployment: Option<&Deployment>, opts: &SysrootDeployTreeOpts, cancellable: Option<&P>) -> Result<Deployment, glib::Error> {
        unsafe {
            let mut out_new_deployment = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_stage_tree_with_options(self.to_glib_none().0, osname.to_glib_none().0, revision.to_glib_none().0, origin.to_glib_none().0, merge_deployment.to_glib_none().0, mut_override(opts.to_glib_none().0), &mut out_new_deployment, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(out_new_deployment)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_try_lock")]
    pub fn try_lock(&self) -> Result<bool, glib::Error> {
        unsafe {
            let mut out_acquired = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_try_lock(self.to_glib_none().0, out_acquired.as_mut_ptr(), &mut error);
            let out_acquired = out_acquired.assume_init();
            if error.is_null() { Ok(from_glib(out_acquired)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_unload")]
    pub fn unload(&self) {
        unsafe {
            ffi::ostree_sysroot_unload(self.to_glib_none().0);
        }
    }

    #[doc(alias = "ostree_sysroot_unlock")]
    pub fn unlock(&self) {
        unsafe {
            ffi::ostree_sysroot_unlock(self.to_glib_none().0);
        }
    }

    #[doc(alias = "ostree_sysroot_write_deployments")]
    pub fn write_deployments<P: IsA<gio::Cancellable>>(&self, new_deployments: &[Deployment], cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_write_deployments(self.to_glib_none().0, new_deployments.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2017_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_4")))]
    #[doc(alias = "ostree_sysroot_write_deployments_with_options")]
    pub fn write_deployments_with_options<P: IsA<gio::Cancellable>>(&self, new_deployments: &[Deployment], opts: &SysrootWriteDeploymentsOpts, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_write_deployments_with_options(self.to_glib_none().0, new_deployments.to_glib_none().0, mut_override(opts.to_glib_none().0), cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_write_origin_file")]
    pub fn write_origin_file<P: IsA<gio::Cancellable>>(&self, deployment: &Deployment, new_origin: Option<&glib::KeyFile>, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sysroot_write_origin_file(self.to_glib_none().0, deployment.to_glib_none().0, new_origin.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_sysroot_get_deployment_origin_path")]
    #[doc(alias = "get_deployment_origin_path")]
    pub fn deployment_origin_path<P: IsA<gio::File>>(deployment_path: &P) -> Option<gio::File> {
        unsafe {
            from_glib_full(ffi::ostree_sysroot_get_deployment_origin_path(deployment_path.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2017_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2017_10")))]
    #[doc(alias = "journal-msg")]
    pub fn connect_journal_msg<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn journal_msg_trampoline<F: Fn(&Sysroot, &str) + 'static>(this: *mut ffi::OstreeSysroot, msg: *mut libc::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &glib::GString::from_glib_borrow(msg))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"journal-msg\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(journal_msg_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Sysroot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Sysroot")
    }
}
