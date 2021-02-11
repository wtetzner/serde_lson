
use serde::{ser, Serialize};
use crate::error::{Error, Result};
use std::io::{Write};
use std::str;

pub fn to_writer<T: Serialize, W: Write>(value: &T, writer: &mut W) -> Result<()> {
    let mut serializer = Serializer {
        indent_str: "  ".to_string(),
        indent: vec![],
        enable_indent: false,
        serializing_key: false,
        output: writer
    };
    value.serialize(&mut serializer)?;
    Ok(())
}

pub fn to_writer_pretty<T: Serialize, W: Write>(value: &T, writer: &mut W) -> Result<()> {
    let mut serializer = Serializer {
        indent_str: "  ".to_string(),
        indent: vec![],
        enable_indent: true,
        serializing_key: false,
        output: writer
    };
    value.serialize(&mut serializer)?;
    Ok(())
}

pub fn to_string<T: Serialize>(value: &T) -> Result<String> {
    let mut output: Vec<u8> = Vec::new();
    to_writer(value, &mut output)?;
    Ok(str::from_utf8(&output)?.to_string())
}

pub fn to_string_pretty<T: Serialize>(value: &T) -> Result<String> {
    let mut output: Vec<u8> = Vec::new();
    to_writer_pretty(value, &mut output)?;
    Ok(str::from_utf8(&output)?.to_string())
}

pub struct Serializer<'a, Writer: Write> {
    indent_str: String,
    indent: Vec<bool>,
    enable_indent: bool,
    serializing_key: bool,
    output: &'a mut Writer
}

impl<'a, Writer: Write> Serializer<'a, Writer> {
    #[inline]
    pub fn enable_indent(&self) -> bool {
        self.enable_indent
    }

    pub fn write_indent(&mut self) -> Result<()> {
        if !self.enable_indent() {
            return Ok(())
        }

        for _idx in 0..self.indent.len() {
            self.output.write_all(self.indent_str.as_bytes())?;
        }
        Ok(())
    }

    pub fn indent(&mut self) {
        self.indent.push(true);
    }

    pub fn dedent(&mut self) {
        if self.indent.len() > 0 {
            self.indent.pop();
        }
    }

    pub fn is_table_start(&self) -> bool {
        if self.indent.len() == 0 {
            return false;
        }
        self.indent[self.indent.len() - 1]
    }

    pub fn clear_table_start(&mut self) {
        if self.indent.len() > 0 {
            let last = self.indent.len() - 1;
            self.indent[last] = false;
        }
    }

    pub fn start_table(&mut self) -> Result<()> {
        self.output.write_all("{".as_bytes())?;
        self.indent();
        Ok(())
    }

    pub fn end_table(&mut self) -> Result<()> {
        if self.enable_indent() {
            self.write("\n")?;
        } else {
            self.write(" ")?;
        }
        self.dedent();
        self.write_indent()?;
        self.write("}")?;
        Ok(())
    }

    pub fn write(&mut self, text: &str) -> Result<()> {
        self.output.write_all(text.as_bytes())?;
        Ok(())
    }
}

fn is_identifier_char(chr: char) -> bool {
    (chr >= 'a' && chr <= 'z')
        || (chr >= 'A' && chr <= 'Z')
        || (chr >= '0' && chr <= '9')
        || chr == '_'
}

pub fn is_identifier(name: &str) -> bool {
    if name.len() == 0 {
        return false;
    }

    for chr in name.chars() {
        if !is_identifier_char(chr) {
            return false;
        }
    }

    let first = name.chars().nth(0).unwrap();
    if first >= '0' && first <= '9' {
        return false
    } else {
        match name {
            "and" | "break" | "do" | "else" | "elseif" | "end"
                | "false" | "for" | "function" | "goto"
                | "if" | "in" | "local" | "nil" | "not" | "or"
                | "repeat" | "return" | "then" | "true" | "until"
                | "while" => false,
            _ => true
        }
    }
}

impl<'a, 'b, W: Write> ser::Serializer for &'a mut Serializer<'b, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.write((if v { "true" } else { "false" }))?;
        Ok(())
    }

    // Lua does not distinguish between different sizes of integers, so all
    // signed integers will be serialized the same and all unsigned integers
    // will be serialized the same.
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i128(i128::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i128(i128::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i128(i128::from(v))
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.serialize_i128(i128::from(v))
    }

    fn serialize_i128(self, v: i128) -> Result<()> {
        self.write(&v.to_string())?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u128(u128::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u128(u128::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u128(u128::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_u128(u128::from(v))
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        self.write(&v.to_string())?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.write(&v.to_string())?;
        Ok(())
    }

    // Serialize a char as a single-character string.
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        if self.serializing_key && is_identifier(v) {
            self.write(v)?;
            return Ok(())
        }
        if self.serializing_key {
            self.write("[")?;
        }
        self.write("\"")?;
        for c in v.chars() { 
            match c {
                '"' => self.write("\\\"")?,
                '\'' => self.write("\\'")?,
                '\x07' => self.write("\\a")?,
                '\x08' => self.write("\\b")?,
                '\x12' => self.write("\\f")?,
                '\n' => self.write("\\n")?,
                '\r' => self.write("\\r")?,
                '\t' => self.write("\\t")?,
                '\x11' => self.write("\\v")?,
                '\\' => self.write("\\\\")?,
                '[' => self.write("\\[")?,
                ']' => self.write("\\]")?,
                _ => self.write(&c.to_string())?
            };
        }
        self.write("\"")?;
        if self.serializing_key {
            self.write("]")?;
        }
        Ok(())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let current_indent = self.enable_indent;
        self.enable_indent = false;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()?;
        self.enable_indent = current_indent;
        Ok(())
    }

    // An absent optional is represented as the LSON `nil`.
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`. Unfortunately this is typically
    // what people expect when working with LSON. Other formats are encouraged
    // to behave more intelligently if possible.
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // LSON as `nil`.
    fn serialize_unit(self) -> Result<()> {
        self.write("nil")?;
        Ok(())
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to LSON as `nil`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to LSON in externally tagged form as `{ NAME = VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output.write_all("{ ".as_bytes())?;
        if is_identifier(variant) {
            self.output.write_all(variant.as_bytes())?;
        } else {
            self.output.write_all("[".as_bytes())?;
            variant.serialize(&mut *self)?;
            self.output.write_all("]".as_bytes())?;
        }
        self.output.write_all(" = ".as_bytes())?;
        value.serialize(&mut *self)?;
        self.output.write_all(" }".as_bytes())?;
        Ok(())
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in LSON is `{`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in LSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.start_table()?;
        Ok(self)
    }

    // Tuples look just like sequences in LSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in LSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME = { DATA... } }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output.write_all("{ ".as_bytes())?;
        if is_identifier(variant) {
            self.output.write_all(variant.as_bytes())?;
        } else {
            self.output.write_all("[".as_bytes())?;
            variant.serialize(&mut *self)?;
            self.output.write_all("]".as_bytes())?;
        }

        self.output.write_all(" = ".as_bytes())?;
        self.start_table()?;
        Ok(self)
    }

    // Maps are represented in LSON as `{ K = V, K = V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.start_table()?;
        Ok(self)
    }

    // Structs look just like maps in LSON. In particular, LSON requires that we
    // serialize the field names of the struct.
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in LSON as `{ NAME = { K = V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output.write_all("{ ".as_bytes())?;
        if is_identifier(variant) {
            self.output.write_all(variant.as_bytes())?;
        } else {
            self.output.write_all("[".as_bytes())?;
            variant.serialize(&mut *self)?;
            self.output.write_all("]".as_bytes())?;
        }
        self.output.write_all(" = ".as_bytes())?;
        self.start_table()?;
        Ok(self)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a, 'b, W: Write> ser::SerializeSeq for &'a mut Serializer<'b, W> {
    type Ok = ();
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.is_table_start() {
            if self.enable_indent() {
                self.write(",\n")?;
            } else {
                self.write(", ")?;
            }
        } else {
            if self.enable_indent() {
                self.write("\n")?;
            } else {
                self.write(" ")?;
            }
            self.clear_table_start();
        }
        self.write_indent()?;
        value.serialize(&mut **self);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        self.end_table()?;
        Ok(())
    }
}


// Same thing but for tuples.
impl<'a, 'b, W: Write> ser::SerializeTuple for &'a mut Serializer<'b, W> {
    type Ok = ();
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.is_table_start() {
            if self.enable_indent() {
                self.write(",\n")?;
            } else {
                self.write(", ")?;
            }
        } else {
            if self.enable_indent() {
                self.write("\n")?;
            } else {
                self.write(" ")?;
            }
            self.clear_table_start();
        }
        self.write_indent()?;
        value.serialize(&mut **self);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        self.end_table()?;
        Ok(())
    }

}

// Same thing but for tuple structs.
impl<'a, 'b, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<'b, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.is_table_start() {
            if self.enable_indent() {
                self.write(",\n")?;
            } else {
                self.write(", ")?;
            }
        } else {
            if self.enable_indent() {
                self.write("\n")?;
            } else {
                self.write(" ")?;
            }
            self.clear_table_start();
        }
        self.write_indent()?;
        value.serialize(&mut **self);
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.end_table()?;
        self.output.write_all("}".as_bytes())?;
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a, 'b, W: Write> ser::SerializeTupleVariant for &'a mut Serializer<'b, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.is_table_start() {
            if self.enable_indent() {
                self.write(",\n")?;
            } else {
                self.write(", ")?;
            }
        } else {
            if self.enable_indent() {
                self.write("\n")?;
            } else {
                self.write(" ")?;
            }
            self.clear_table_start();
        }
        self.write_indent()?;
        value.serialize(&mut **self);
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.end_table()?;
        self.write("}")?;
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In LSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a, 'b, W: Write> ser::SerializeMap for &'a mut Serializer<'b, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.is_table_start() {
            if self.enable_indent() {
                self.write(",\n")?;
            } else {
                self.write(", ")?;
            }
        } else {
            if self.enable_indent() {
                self.write("\n")?;
            } else {
                self.write(" ")?;
            }
            self.clear_table_start();
        }

        self.write_indent()?;

        let current_indent = self.enable_indent;
        self.enable_indent = false;
        let ser_type = crate::type_ser::compute_type(&key)?;
        if ser_type == crate::type_ser::Type::String {
            self.serializing_key = true;

            key.serialize(&mut **self)?;

            self.serializing_key = false;
        } else {
            self.write("[")?;
            key.serialize(&mut **self)?;
            self.write("]")?;
        }
        self.enable_indent = current_indent;

        Ok(())
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output.write_all(" = ".as_bytes())?;
        value.serialize(&mut **self);
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.end_table()?;
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a, 'b, W: Write> ser::SerializeStruct for &'a mut Serializer<'b, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.is_table_start() {
            if self.enable_indent() {
                self.write(",\n")?;
            } else {
                self.write(", ")?;
            }
        } else {
            if self.enable_indent() {
                self.write("\n")?;
            } else {
                self.write(" ")?;
            }
            self.clear_table_start();
        }

        self.write_indent()?;

        if is_identifier(key) {
            self.output.write_all(key.as_bytes())?;
        } else {
            self.output.write_all("[".as_bytes())?;
            key.serialize(&mut **self)?;
            self.output.write_all("]".as_bytes())?;
        }

        self.output.write_all(" = ".as_bytes())?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.end_table()?;
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a, 'b, W: Write> ser::SerializeStructVariant for &'a mut Serializer<'b, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.is_table_start() {
            if self.enable_indent() {
                self.write(",\n")?;
            } else {
                self.write(", ")?;
            }
        } else {
            if self.enable_indent() {
                self.write("\n")?;
            } else {
                self.write(" ")?;
            }
            self.clear_table_start();
        }

        self.write_indent()?;

        if is_identifier(key) {
            self.output.write_all(key.as_bytes())?;
        } else {
            self.output.write_all("[".as_bytes())?;
            key.serialize(&mut **self)?;
            self.output.write_all("]".as_bytes())?;
        }

        self.output.write_all(" = ".as_bytes())?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.end_table()?;
        self.output.write_all("}".as_bytes())?;
        Ok(())
    }
}
