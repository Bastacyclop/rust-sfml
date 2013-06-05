
use graphics::vertex::Vertex;
use core::libc::c_uint;
use graphics::rect::FloatRect;
use graphics::primitive_type;
use graphics::primitive_type::PrimitiveType;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_uint, c_void};
    use graphics::vertex;
    use graphics::rect::FloatRect;
    
    pub type sfPrimitiveType = c_uint;
    pub static sfPoints : sfPrimitiveType = 0;
    pub static sfLines : sfPrimitiveType = 1;
    pub static sfLinesStrip : sfPrimitiveType = 2;
    pub static sfTriangles : sfPrimitiveType = 3;
    pub static sfTrianglesStrip : sfPrimitiveType = 4;
    pub static sfTrianglesFan : sfPrimitiveType = 5;
    pub static sfQuads : sfPrimitiveType = 6;

    pub struct sfVertexArray {
        This : *c_void
    }

    pub extern "C" {
        fn sfVertexArray_create() -> *sfVertexArray;
        fn sfVertexArray_copy(vertexArray : *sfVertexArray) -> *sfVertexArray;
        fn sfVertexArray_destroy(vertexArray : *sfVertexArray) -> ();
        fn sfVertexArray_getVertexCount(vertexArray : *sfVertexArray) -> c_uint;
        //fn sfVertexArray_getVertex(vertexArray : *sfVertexArray, index : c_uint) -> *csfml::sfVertex;
        fn sfVertexArray_clear(vertexArray : *sfVertexArray) -> ();
        fn sfVertexArray_resize(vertexArray : *sfVertexArray, vertexCount : c_uint) -> ();
        fn sfVertexArray_append(vertexArray : *sfVertexArray, vertex : vertex::Vertex) -> ();
        fn sfVertexArray_setPrimitiveType(vertexArray : *sfVertexArray, stype : sfPrimitiveType) -> ();
        fn sfVertexArray_getPrimitiveType(vertexArray : *sfVertexArray) -> sfPrimitiveType;
        fn sfVertexArray_getBounds(vertexArray : *sfVertexArray) -> FloatRect;
    }
}

#[doc(hidden)]
pub struct VertexArray {
    priv vertexArray : *csfml::sfVertexArray
}

impl VertexArray {
    pub fn new() -> VertexArray {
        unsafe {
            VertexArray { vertexArray : csfml::sfVertexArray_create()}
        }
    }

    pub fn new_copy(vertexArray : &VertexArray) -> VertexArray {
        unsafe {
            VertexArray { vertexArray : csfml::sfVertexArray_copy(vertexArray.unwrap())}
        }
    }

    pub fn get_vertex_count(&self) -> uint {
        unsafe {
            csfml::sfVertexArray_getVertexCount(self.vertexArray) as uint
        }
    }

    pub fn clear(&self) -> () {
        unsafe {
            csfml::sfVertexArray_clear(self.vertexArray)
        }
    }
    
    pub fn resize(&self, vertexCount : uint) -> () {
        unsafe {
            csfml::sfVertexArray_resize(self.vertexArray, vertexCount as c_uint)
        }
    }

    pub fn append(&self, vertex : &Vertex) -> () {
        unsafe {
            csfml::sfVertexArray_append(self.vertexArray, *vertex)
        }
    }

    pub fn get_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfVertexArray_getBounds(self.vertexArray)
        }
    }
    
    pub fn set_primitive_type(&self, primitiveType : PrimitiveType) -> () {
        match primitiveType {
            primitive_type::Points              => unsafe {csfml::sfVertexArray_setPrimitiveType(self.vertexArray, csfml::sfPoints)},
            primitive_type::Lines               => unsafe {csfml::sfVertexArray_setPrimitiveType(self.vertexArray, csfml::sfLines)},
            primitive_type::LinesStrip          => unsafe {csfml::sfVertexArray_setPrimitiveType(self.vertexArray, csfml::sfLinesStrip)},
            primitive_type::Triangles           => unsafe {csfml::sfVertexArray_setPrimitiveType(self.vertexArray, csfml::sfTriangles)},
            primitive_type::TrianglesStrip      => unsafe {csfml::sfVertexArray_setPrimitiveType(self.vertexArray, csfml::sfTrianglesStrip)},
            primitive_type::TrianglesFan        => unsafe {csfml::sfVertexArray_setPrimitiveType(self.vertexArray, csfml::sfTrianglesFan)},
            primitive_type::Quads               => unsafe {csfml::sfVertexArray_setPrimitiveType(self.vertexArray, csfml::sfQuads)}
        }
    }
    
    pub fn get_primitive_type(&self) -> PrimitiveType {
        match unsafe {csfml::sfVertexArray_getPrimitiveType(self.vertexArray)} {
            csfml::sfPoints             => primitive_type::Points,
            csfml::sfLines              => primitive_type::Lines,
            csfml::sfLinesStrip         => primitive_type::LinesStrip,
            csfml::sfTriangles          => primitive_type::Triangles,
            csfml::sfTrianglesStrip     => primitive_type::TrianglesStrip,
            csfml::sfTrianglesFan       => primitive_type::TrianglesFan,
            csfml::sfQuads              => primitive_type::Quads,
            _                           => primitive_type::Points   
        }
    }

    pub fn wrap(vertexArray : *csfml::sfVertexArray) -> VertexArray {
        VertexArray {vertexArray : vertexArray}
    }

    pub fn unwrap(&self) -> *csfml::sfVertexArray {
        self.vertexArray
    }
}

impl Drop for VertexArray {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfVertexArray_destroy(self.vertexArray)
        }
    }
}