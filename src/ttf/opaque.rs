

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TTF_Font { 
    _data : [u8; 0],
    _marker : core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}