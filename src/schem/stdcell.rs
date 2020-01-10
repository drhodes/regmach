// the std objects used for
// button, circle with x through it.
// use lyon to render out to array buffers
use crate::schem::types::*;

impl Button for Entity {
    struct glstuff {
        vbo,
        vao,
        vertices,
        color stuff,        
    }

    // append vertices onto big vao for glBindBuffer
    // use shader for translation and positioning, rotation?
    // sub-modules are gonna need terminals...
    // but this will work well for layers, if layout is ever a thing.
}

