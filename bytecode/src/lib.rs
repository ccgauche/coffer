/*
 *     This file is part of Coffer.
 *
 *     Coffer is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU Lesser General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     Coffer is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU Lesser General Public License
 *     along with Coffer. (LICENSE.md)  If not, see <https://www.gnu.org/licenses/>.
 */
//! Coffer is a lightweight and fast library for reading and writing Java classes.
//!
//! Read and write functions are exposed via the traits [`ReadWrite`] and [`ConstantPoolReadWrite`] when the information from the constant pool is needed to get the information.
//!
//! [`ReadWrite`] uses instances of `Read` and `Write`, and [`ConstantPoolReadWrite`] uses additional parameters: instances of [`ConstantPoolWriter`] and [`ConstantPoolReader`].
//! These traits represent read and write access to the constant pool.
//!
//! Many implementors of [`ReadWrite`] and [`ConstantPoolReadWrite`] uses a derive macro internally to avoid repeating implementation for structures that just calls the trait functions of its fields.
//!
//! [`ReadWrite`]: crate::ReadWrite
//! [`ConstantPoolReadWrite`]: crate::ConstantPoolReadWrite
//! [`ConstantPoolWriter`]: crate::ConstantPoolWriter
//! [`ConstantPoolReader`]: crate::ConstantPoolReader
#![cfg_attr(any(feature = "backtrace", test), feature(backtrace))]
// #![warn(missing_docs)]
#![allow(missing_doc_code_examples)] // TODO Change these later to higher levels

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate coffer_macros;

use std::borrow::Cow;
use std::io::{Read, Write};

use prelude::*;
pub use rw::*;

pub use crate::error::Error;
pub use crate::error::Result;

pub mod annotation;
pub mod attr;
pub mod code;
pub mod constants;
pub mod cp;
pub mod dynamic;
pub mod error;
pub mod flags;

pub mod mod_utf8;
pub mod module;
pub mod member;
pub mod prelude;
pub mod ty;
pub mod signature;
pub mod loadable;

pub mod version;
pub mod rw;




#[cfg(test)]
mod tests;
pub(crate) mod insn;

#[derive(Debug, Clone)]
pub struct Class {
    pub version: JavaVersion,
    pub access: ClassFlags,
    pub name: Cow<'static, str>,
    /// The name of the super class for this class.
    /// when it is unspecifiedi in the source code, it is `"java/lang/Object"`.
    ///
    /// Although most java classes have their own superclasses,
    /// java/lang/Object has no superclass.
    pub super_name: Option<Cow<'static, str>>,
    pub interfaces: Vec<Cow<'static, str>>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<ClassAttribute>
}



#[derive(ConstantPoolReadWrite)]
struct ClassWrapper {
    #[use_normal_rw]
    pub access: ClassFlags,
    #[str_type(Class)]
    pub name: Cow<'static, str>,
    #[str_optional]
    #[str_type(Class)]
    pub super_name: Option<Cow<'static, str>>,
    #[vec_len_type(u16)]
    #[str_type(Class)]
    pub interfaces: Vec<Cow<'static, str>>,
    #[vec_len_type(u16)]
    pub fields: Vec<Field>,
    #[vec_len_type(u16)]
    pub methods: Vec<Method>,
    #[vec_len_type(u16)]
    pub attributes: Vec<ClassAttribute>
}

impl ReadWrite for Class {
    fn read_from<T: Read>(reader: &mut T) -> Result<Self> {
        match u32::read_from(reader)? {
            0xCAFEBABE => {
                let version = JavaVersion::read_from(reader)?;
                let mut cp = MapCp::read_from(reader)?;
                let c = ClassWrapper::read_from(&mut cp, reader)?;
                for attr in &c.attributes {
                    if let ClassAttribute::BootstrapMethods(b) = attr {
                        cp.bootstrap_methods(b)?;
                        break
                    }
                }
                Ok(Class {
                    version,
                    access: c.access,
                    name: c.name,
                    super_name: c.super_name,
                    interfaces: c.interfaces,
                    fields: c.fields,
                    methods: c.methods,
                    attributes: c.attributes
                })
            }
            n => Err(Error::Invalid("class header", n.to_string().into()))
        }
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> Result<()> {
        0xCAFEBABEu32.write_to(writer)?;
        self.version.write_to(writer)?;
        let mut cp = VecCp::new();
        let mut buf = vec![];
        self.access.write_to(&mut buf)?;
        cp.insert_class(self.name.clone()).write_to(&mut buf)?;
        self.super_name.as_ref().map_or(0, |n| cp.insert_class(n.clone())).write_to(&mut buf)?;
        (self.interfaces.len() as u16).write_to(&mut buf)?;
        for i in &self.interfaces {
            cp.insert_class(i.clone()).write_to(&mut buf)?;
        }
        (self.fields.len() as u16).write_to(&mut buf)?;
        for f in &self.fields {
            f.write_to(&mut cp, &mut buf)?;
        }
        (self.methods.len() as u16).write_to(&mut buf)?;
        for m in &self.methods {
            m.write_to(&mut cp, &mut buf)?;
        }
        (self.attributes.len() as u16).write_to(&mut buf)?;
        let mut bsm = vec![];
        for a in &self.attributes {
            if let ClassAttribute::BootstrapMethods(b) = a {
                bsm.extend_from_slice(b);
            } else {
                a.write_to(&mut cp, &mut buf)?;
            }
        }
        if !bsm.is_empty() {
            let mut i: u16 = 0;
            let mut buf2 = vec![];
            while !cp.bsm.is_empty() {
                let v = cp.bsm;
                cp.bsm = vec![];
                i += v.len() as u16;
                for bsm in v {
                    bsm.write_to(&mut cp, &mut buf2)?;
                }
            }
            write_to!(&Cow::Borrowed("BootstrapMethods"), &mut cp, &mut buf)?;
            (buf2.len() as u32).write_to(&mut buf)?;
            i.write_to(&mut buf)?;
            buf.write_all(&buf2)?;
        }
        cp.write_to(writer)?;
        writer.write_all(&buf)?;
        Ok(())
    }
}
