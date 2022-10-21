
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Window { 
    _data : [u8; 0],
    _marker : core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Surface { 
    _data : [u8; 0],
    _marker : core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Texture {
    _data : [u8; 0],
    _marker : core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
 }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Renderer { 
    _data : [u8; 0],
    _marker : core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
