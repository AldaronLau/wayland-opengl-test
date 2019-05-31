use super::Draw;

#[repr(C)]
pub struct OpenGL {
    
}

impl Drop for OpenGL {
    fn drop(&mut self) {

    }
}

impl Draw for OpenGL {

}

pub(super) fn new() -> Option<Box<Draw>> {
    let draw = OpenGL {
    };

    Some(Box::new(draw))
}
