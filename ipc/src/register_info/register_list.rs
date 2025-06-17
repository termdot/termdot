use super::IRegisterInfo;
use common::typedef::RegisterInfoId;
use std::{marker::PhantomData, mem::MaybeUninit};
use tmui::tipc::{
    mem::BuildType,
    parking_lot::{Mutex, MutexGuard},
    shared_memory::{Shmem, ShmemConf, ShmemError},
};

#[repr(C)]
struct _RegisterList<const SIZE: usize, T: IRegisterInfo> {
    len: usize,
    inner: [MaybeUninit<T>; SIZE],
    init_flags: [bool; SIZE],
}

impl<const SIZE: usize, T: IRegisterInfo> _RegisterList<SIZE, T> {
    #[inline]
    fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }

    fn add(&mut self, t: T) {
        if self.len == SIZE {
            return;
        }

        let mut idx = 0;

        for (i, init_flag) in self.init_flags.iter().enumerate() {
            if *init_flag {
                continue;
            }

            idx = i;
            break;
        }

        self.inner[idx] = MaybeUninit::new(t);
        self.init_flags[idx] = true;
        self.len += 1;
    }

    fn remove(&mut self, id: RegisterInfoId) {
        for (idx, maybe_uninit) in self.inner.iter().enumerate() {
            if !self.init_flags[idx] {
                continue;
            }

            unsafe {
                if maybe_uninit.assume_init_ref().id() == id {
                    self.init_flags[idx] = false;
                    self.len -= 1;
                }
            }

            break;
        }
    }

    fn has(&self, id: RegisterInfoId) -> bool {
        let mut visit = 0;
        for (idx, maybe_uninit) in self.inner.iter().enumerate() {
            if visit == self.len {
                break;
            }

            if !self.init_flags[idx] {
                continue;
            }
            visit += 1;

            unsafe {
                let mut_ref = maybe_uninit.assume_init_ref();
                if mut_ref.id() == id {
                    return true;
                }
            }
        }

        false
    }

    fn get_ref(&self, id: RegisterInfoId) -> Option<&T> {
        let mut visit = 0;
        for (idx, maybe_uninit) in self.inner.iter().enumerate() {
            if visit == self.len {
                break;
            }

            if !self.init_flags[idx] {
                continue;
            }
            visit += 1;

            unsafe {
                let mut_ref = maybe_uninit.assume_init_ref();
                if mut_ref.id() == id {
                    return Some(mut_ref);
                }
            }
        }

        None
    }

    fn get_mut(&mut self, id: RegisterInfoId) -> Option<&mut T> {
        let mut visit = 0;
        for (idx, maybe_uninit) in self.inner.iter_mut().enumerate() {
            if visit == self.len {
                break;
            }

            if !self.init_flags[idx] {
                continue;
            }
            visit += 1;

            unsafe {
                let mut_ref = maybe_uninit.assume_init_mut();
                if mut_ref.id() == id {
                    return Some(mut_ref);
                }
            }
        }

        None
    }

    fn for_each<F: Fn(&mut T)>(&mut self, f: F) {
        let mut visit = 0;

        for (idx, maybe_uninit) in self.inner.iter_mut().enumerate() {
            if visit == self.len {
                break;
            }

            if !self.init_flags[idx] {
                continue;
            }

            unsafe { f(maybe_uninit.assume_init_mut()) }
            visit += 1;
        }
    }

    fn check_valid(&mut self) {
        for (idx, maybe_uninit) in self.inner.iter().enumerate() {
            if !self.init_flags[idx] {
                continue;
            }

            unsafe {
                if !maybe_uninit.assume_init_ref().is_valid() {
                    self.init_flags[idx] = false;
                    self.len -= 1;
                }
            }
        }
    }
}

pub struct RegisterList<const SIZE: usize, T: IRegisterInfo> {
    shmem: Shmem,
    _type_holder: PhantomData<T>,
    mutex: Mutex<()>,
}
impl<const SIZE: usize, T: IRegisterInfo> RegisterList<SIZE, T> {
    pub fn create() -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new()
            .size(size_of::<_RegisterList<SIZE, T>>())
            .create()?;

        Ok(Self {
            shmem,
            _type_holder: PhantomData,
            mutex: Mutex::new(()),
        })
    }

    pub fn create_with_os_id(os_id: &str) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new()
            .size(size_of::<_RegisterList<SIZE, T>>())
            .os_id(os_id)
            .create()?;

        Ok(Self {
            shmem,
            _type_holder: PhantomData,
            mutex: Mutex::new(()),
        })
    }

    pub fn open(os_id: &str) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new().os_id(os_id).open()?;

        Ok(Self {
            shmem,
            _type_holder: PhantomData,
            mutex: Mutex::new(()),
        })
    }

    #[inline]
    pub fn os_id(&self) -> &str {
        self.shmem.get_os_id()
    }

    #[inline]
    pub fn clear(&mut self) {
        let _guard = self.mutex.lock();
        self.list_mut().clear();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        let _guard = self.mutex.lock();
        self.list_mut().is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        let _guard = self.mutex.lock();
        self.list_mut().len()
    }

    #[inline]
    pub fn add(&mut self, t: T) {
        let _guard = self.mutex.lock();
        self.list_mut().add(t);
    }

    #[inline]
    pub fn remove(&mut self, id: RegisterInfoId) {
        let _guard = self.mutex.lock();
        self.list_mut().remove(id);
    }

    #[inline]
    pub fn has(&self, id: RegisterInfoId) -> bool {
        let _guard = self.mutex.lock();
        self.list_mut().has(id)
    }

    #[inline]
    pub fn get_ref(&self, id: RegisterInfoId) -> Option<(&T, MutexGuard<()>)> {
        let guard = self.mutex.lock();
        self.list_mut().get_ref(id).map(|info| (info, guard))
    }

    #[inline]
    pub fn get_mut(&mut self, id: RegisterInfoId) -> Option<(&mut T, MutexGuard<()>)> {
        let guard = self.mutex.lock();
        self.list_mut().get_mut(id).map(|info| (info, guard))
    }

    #[inline]
    pub fn for_each<F: Fn(&mut T)>(&mut self, f: F) {
        let _guard = self.mutex.lock();
        self.list_mut().for_each(f);
    }

    #[inline]
    pub fn check_valid(&mut self) {
        let _guard = self.mutex.lock();
        self.list_mut().check_valid();
    }

    #[inline]
    fn list_mut(&self) -> &'static mut _RegisterList<SIZE, T> {
        unsafe {
            (self.shmem.as_ptr() as *mut _RegisterList<SIZE, T>)
                .as_mut()
                .unwrap()
        }
    }
}

#[derive(Default)]
pub struct RegisterListBuilder {
    build_type: BuildType,
    os_id: Option<String>,
}

impl RegisterListBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn build_type(mut self, build_type: BuildType) -> Self {
        self.build_type = build_type;
        self
    }

    #[inline]
    pub fn os_id<P: ToString>(mut self, os_id: P) -> Self {
        self.os_id = Some(os_id.to_string());
        self
    }

    pub fn build<const SIZE: usize, T: IRegisterInfo>(
        self,
    ) -> Result<RegisterList<SIZE, T>, ShmemError> {
        match self.build_type {
            BuildType::Create => {
                if let Some(ref os_id) = self.os_id {
                    RegisterList::create_with_os_id(os_id)
                } else {
                    RegisterList::create()
                }
            }
            BuildType::Open => {
                if let Some(ref os_id) = self.os_id {
                    RegisterList::open(os_id)
                } else {
                    panic!("`Open` MemQueue must assign the os_id")
                }
            }
        }
    }
}
