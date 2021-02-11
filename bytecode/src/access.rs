/*
    This file is part of Coffer.

    Coffer is free software: you can redistribute it and/or modify
    it under the terms of the GNU Lesser General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Coffer is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with Coffer. (LICENSE.md)  If not, see <https://www.gnu.org/licenses/>.
*/
use crate::ReadWrite;
use crate::Result;
use std::io::{Read, Write};

bitflags! {
    #[doc = "Access flags for classes."]
    pub struct ClassFlags: u16 {
        #[doc = "This class may be accessed from outside its package."]
        const ACC_PUBLIC       = 0b0000_0000_0000_0001;
        #[doc = "This class can't have subclasses."]
        const ACC_FINAL        = 0b0000_0000_0001_0000;
        #[doc = "Treat superclass methods specially.

                 This flag has no effect in Java 8 and above."]
        const ACC_SUPER        = 0b0000_0000_0010_0000;
        #[doc = "This is an interface, not a class."]
        const ACC_INTERFACE    = 0b0000_0010_0000_0000;
        #[doc = "This class is declared `abstract` and may not be instantiated."]
        const ACC_ABSTRACT     = 0b0000_0100_0000_0000;
        #[doc = "This class is synthetic and is not present in the source code."]
        const ACC_SYNTHETIC    = 0b0001_0000_0000_0000;
        #[doc = "This class is an annotation class."]
        const ACC_ANNOTATION   = 0b0010_0000_0000_0000;
        #[doc = "This class is an enum type."]
        const ACC_ENUM         = 0b0100_0000_0000_0000;
        #[doc = "This is a module, not a class or interface."]
        const ACC_MODULE       = 0b1000_0000_0000_0000;
    }
}

bitflags! {
    #[doc = "Access flags for fields."]
    pub struct FieldFlags: u16 {
        #[doc = "This field may be accessed from outside its package."]
        const ACC_PUBLIC       = 0b0000_0000_0000_0001;
        #[doc = "This field is only accessible within the defining class and other classes belonging to the same nest"]
        const ACC_PRIVATE      = 0b0000_0000_0000_0010;
        #[doc = "This field may be accessed from subclasses."]
        const ACC_PROTECTED    = 0b0000_0000_0000_0100;
        #[doc = "This field is static."]
        const ACC_STATIC       = 0b0000_0000_0000_1000;
        #[doc = "This field may not be assigned again after object construction."]
        const ACC_FINAL        = 0b0000_0000_0001_0000;
        #[doc = "This field is volatile and cannot be cached."]
        const ACC_VOLATILE     = 0b0000_0000_0100_0000;
        #[doc = "This field not written or read by a persistent object manager i.e. not serialized."]
        const ACC_TRANSIENT    = 0b0000_0000_1000_0000;
        #[doc = "This field is synthetic and is not present in the source code."]
        const ACC_SYNTHETIC    = 0b0001_0000_0000_0000;
        #[doc = "This field is an element of an enum type."]
        const ACC_ENUM         = 0b0100_0000_0000_0000;
    }
}

bitflags! {
    #[doc = "Access flags for methods."]
    pub struct MethodFlags: u16 {
        #[doc = "This method may be accessed from outside its package."]
        const ACC_PUBLIC       = 0b0000_0000_0000_0001;
        #[doc = "This method is only accessible within the defining class and other classes belonging to the same nest"]
        const ACC_PRIVATE      = 0b0000_0000_0000_0010;
        #[doc = "This method may be accessed from subclasses."]
        const ACC_PROTECTED    = 0b0000_0000_0000_0100;
        #[doc = "This method is static."]
        const ACC_STATIC       = 0b0000_0000_0000_1000;
        #[doc = "This method may not be overridden."]
        const ACC_FINAL        = 0b0000_0000_0001_0000;
        #[doc = "This method will be wrapped by a monitor use."]
        const ACC_SYNCHRONIZED = 0b0000_0000_0010_0000;
        #[doc = "This method is a bridge method generated by the compiler."]
        const ACC_BRIDGE       = 0b0000_0000_0100_0000;
        #[doc = "This method takes a variable number of arguments at the source code level"]
        const ACC_VARARGS      = 0b0000_0000_1000_0000;
        #[doc = "This method is native and implemented in another language."]
        const ACC_NATIVE       = 0b0000_0001_0000_0000;
        #[doc = "No implementation is provided."]
        const ACC_ABSTRACT     = 0b0000_0100_0000_0000;
        #[doc = "Floating point mode is FP-strict."]
        const ACC_STRICT       = 0b0000_1000_0000_0000;
        #[doc = "This method is synthetic and is not present in the source code."]
        const ACC_SYNTHETIC    = 0b0001_0000_0000_0000;
    }
}

bitflags! {
    #[doc = "Access flags for inner classes."]
    pub struct InnerClassFlags: u16 {
        #[doc = "This class may be accessed from outside its package."]
        const ACC_PUBLIC       = 0b0000_0000_0000_0001;
        #[doc = "This class is only accessible within the defining class and other classes belonging to the same nest"]
        const ACC_PRIVATE      = 0b0000_0000_0000_0010;
        #[doc = "This class may be accessed from subclasses."]
        const ACC_PROTECTED    = 0b0000_0000_0000_0100;
        #[doc = "This class is static."]
        const ACC_STATIC       = 0b0000_0000_0000_1000;
        #[doc = "This class can't have subclasses."]
        const ACC_FINAL        = 0b0000_0000_0001_0000;
        #[doc = "This is an interface, not a class."]
        const ACC_INTERFACE    = 0b0000_0010_0000_0000;
        #[doc = "This class is declared `abstract` and may not be instantiated."]
        const ACC_ABSTRACT     = 0b0000_0100_0000_0000;
        #[doc = "This class is synthetic and is not present in the source code."]
        const ACC_SYNTHETIC    = 0b0001_0000_0000_0000;
        #[doc = "This class is an annotation class."]
        const ACC_ANNOTATION   = 0b0010_0000_0000_0000;
        #[doc = "This class is an enum type."]
        const ACC_ENUM         = 0b0100_0000_0000_0000;
    }
}

bitflags! {
    #[doc = "Access flags for method parameters."]
    pub struct MethodParameterFlags: u16 {
        #[doc = "This method parameter was declared `final`."]
        const ACC_FINAL        = 0b0000_0000_0001_0000;
        #[doc = "This method parameter is synthetic and is not present in the source code."]
        const ACC_SYNTHETIC    = 0b0001_0000_0000_0000;
        #[doc = "This method parameter is mandated by a language specification. So all compilers for the language must emit it."]
        const ACC_MANDATED     = 0b1000_0000_0000_0000;
    }
}

bitflags! {
    #[doc = "Flags for modules."]
    pub struct ModuleFlags: u16 {
        #[doc = "This module is open."]
        const ACC_OPEN         = 0b0000_0000_0010_0000;
        #[doc = "This module was not explicitly or implicitly declared."]
        const ACC_SYNTHETIC    = 0b0001_0000_0000_0000;
        #[doc = "This module was implicitly declared."]
        const ACC_MANDATED     = 0b1000_0000_0000_0000;
    }
}

bitflags! {
    #[doc = "Flags for module requires."]
    pub struct RequireFlags: u16 {
        #[doc = "Any module which depends on the current module, implicitly declares a dependence on the module indicated by this entry."]
        const ACC_TRANSITIVE   = 0b0000_0000_0010_0000;
        #[doc = "This dependence is mandatory in the static phase, i.e., at compile time, but is optional in the dynamic phase, i.e., at run time. "]
        const ACC_STATIC_PHASE = 0b0000_0000_0100_0000;
        #[doc = "This dependence was not explicitly or implicitly declared."]
        const ACC_SYNTHETIC    = 0b0001_0000_0000_0000;
        #[doc = "This dependence was implicitly declared."]
        const ACC_MANDATED     = 0b1000_0000_0000_0000;
    }
}

bitflags! {
    #[doc = "Flags for module exports and opens."]
    pub struct ExOpFlags: u16 {
        #[doc = "This export/opening was not explicitly or implicitly declared."]
        const ACC_SYNTHETIC    = 0b0001_0000_0000_0000;
        #[doc = "This export/opening was implicitly declared."]
        const ACC_MANDATED     = 0b1000_0000_0000_0000;
    }
}

macro_rules! rw_impls {
    ($($ty:ty),*) => {
        $(
            impl ReadWrite for $ty {
                    fn read_from<T: Read>(reader: &mut T) -> Result<$ty> { Ok(<$ty>::from_bits(u16::read_from(reader)?).unwrap()) }
                    fn write_to<T: Write>(&self, writer: &mut T) -> Result<()> { self.bits().write_to(writer) }
            }
        )*
    };
}

rw_impls!(ClassFlags, ExOpFlags, RequireFlags, ModuleFlags, MethodParameterFlags, InnerClassFlags, MethodFlags, FieldFlags);