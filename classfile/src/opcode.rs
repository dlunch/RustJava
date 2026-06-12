use alloc::{collections::BTreeMap, vec::Vec};

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{flat_map, map, success},
    multi::count,
    number::complete::{be_i16, be_i32, be_u16, i8, u8},
};

use crate::constant_pool::{ConstantPoolItem, ConstantPoolReference};

#[derive(Clone, Debug)]
pub enum Opcode {
    Aaload,
    Aastore,
    AconstNull,
    Aload(u8),
    Anewarray(ConstantPoolReference),
    Areturn,
    Arraylength,
    Astore(u8),
    Athrow,
    Baload,
    Bastore,
    Bipush(i8),
    Caload,
    Castore,
    Checkcast(ConstantPoolReference),
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst(u8),
    Ddiv,
    Dload(u8),
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore(u8),
    Dsub,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst(u8),
    Fdiv,
    Fload(u8),
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore(u8),
    Fsub,
    Getfield(ConstantPoolReference),
    Getstatic(ConstantPoolReference),
    Goto(i16),
    GotoW(i32),
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    Iconst(i8),
    Idiv,
    IfAcmpeq(i16),
    IfAcmpne(i16),
    IfIcmpeq(i16),
    IfIcmpne(i16),
    IfIcmplt(i16),
    IfIcmpge(i16),
    IfIcmpgt(i16),
    IfIcmple(i16),
    Ifeq(i16),
    Ifne(i16),
    Iflt(i16),
    Ifge(i16),
    Ifgt(i16),
    Ifle(i16),
    Ifnonnull(i16),
    Ifnull(i16),
    Iinc(u8, i8),
    Iload(u8),
    Imul,
    Ineg,
    Instanceof(ConstantPoolReference),
    Invokedynamic(ConstantPoolReference),
    Invokeinterface(ConstantPoolReference, u8, u8),
    Invokespecial(ConstantPoolReference),
    Invokestatic(ConstantPoolReference),
    Invokevirtual(ConstantPoolReference),
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore(u8),
    Isub,
    Iushr,
    Ixor,
    Jsr(i16),
    JsrW(i32),
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst(u8),
    Ldc(ConstantPoolReference),
    LdcW(ConstantPoolReference),
    Ldc2W(ConstantPoolReference),
    Ldiv,
    Lload(u8),
    Lmul,
    Lneg,
    Lookupswitch(i32, Vec<(i32, i32)>),
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore(u8),
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray(ConstantPoolReference, u8),
    New(ConstantPoolReference),
    Newarray(u8),
    Nop,
    Pop,
    Pop2,
    Putfield(ConstantPoolReference),
    Putstatic(ConstantPoolReference),
    Ret(u8),
    Return,
    Saload,
    Sastore,
    Sipush(i16),
    Swap,
    Tableswitch(i32, Vec<(i32, i32)>),
    Wide,
}

impl Opcode {
    pub fn parse<'a>(data: &'a [u8], offset: usize, constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        flat_map(u8, |x| move |i| Self::parse_opcode(x, offset, i, constant_pool)).parse(data)
    }

    fn parse_opcode<'a>(opcode: u8, offset: usize, data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        match opcode {
            0x32 => success(Opcode::Aaload).parse(data),
            0x53 => success(Opcode::Aastore).parse(data),
            0x01 => success(Opcode::AconstNull).parse(data),
            0x19 => map(u8, Opcode::Aload).parse(data),
            0x2a => success(Opcode::Aload(0)).parse(data),
            0x2b => success(Opcode::Aload(1)).parse(data),
            0x2c => success(Opcode::Aload(2)).parse(data),
            0x2d => success(Opcode::Aload(3)).parse(data),
            0xbd => map(be_u16, |x| {
                Opcode::Anewarray(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0xb0 => success(Opcode::Areturn).parse(data),
            0xbe => success(Opcode::Arraylength).parse(data),
            0x3a => map(u8, Opcode::Astore).parse(data),
            0x4b => success(Opcode::Astore(0)).parse(data),
            0x4c => success(Opcode::Astore(1)).parse(data),
            0x4d => success(Opcode::Astore(2)).parse(data),
            0x4e => success(Opcode::Astore(3)).parse(data),
            0xbf => success(Opcode::Athrow).parse(data),
            0x33 => success(Opcode::Baload).parse(data),
            0x54 => success(Opcode::Bastore).parse(data),
            0x10 => map(i8, Opcode::Bipush).parse(data),
            0x34 => success(Opcode::Caload).parse(data),
            0x55 => success(Opcode::Castore).parse(data),
            0xc0 => map(be_u16, |x| {
                Opcode::Checkcast(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0x90 => success(Opcode::D2f).parse(data),
            0x8e => success(Opcode::D2i).parse(data),
            0x8f => success(Opcode::D2l).parse(data),
            0x63 => success(Opcode::Dadd).parse(data),
            0x31 => success(Opcode::Daload).parse(data),
            0x52 => success(Opcode::Dastore).parse(data),
            0x98 => success(Opcode::Dcmpg).parse(data),
            0x97 => success(Opcode::Dcmpl).parse(data),
            0x0e => success(Opcode::Dconst(0)).parse(data),
            0x0f => success(Opcode::Dconst(1)).parse(data),
            0x6f => success(Opcode::Ddiv).parse(data),
            0x18 => map(u8, Opcode::Dload).parse(data),
            0x26 => success(Opcode::Dload(0)).parse(data),
            0x27 => success(Opcode::Dload(1)).parse(data),
            0x28 => success(Opcode::Dload(2)).parse(data),
            0x29 => success(Opcode::Dload(3)).parse(data),
            0x6b => success(Opcode::Dmul).parse(data),
            0x77 => success(Opcode::Dneg).parse(data),
            0x73 => success(Opcode::Drem).parse(data),
            0xaf => success(Opcode::Dreturn).parse(data),
            0x39 => map(u8, Opcode::Dstore).parse(data),
            0x47 => success(Opcode::Dstore(0)).parse(data),
            0x48 => success(Opcode::Dstore(1)).parse(data),
            0x49 => success(Opcode::Dstore(2)).parse(data),
            0x4a => success(Opcode::Dstore(3)).parse(data),
            0x67 => success(Opcode::Dsub).parse(data),
            0x59 => success(Opcode::Dup).parse(data),
            0x5a => success(Opcode::DupX1).parse(data),
            0x5b => success(Opcode::DupX2).parse(data),
            0x5c => success(Opcode::Dup2).parse(data),
            0x5d => success(Opcode::Dup2X1).parse(data),
            0x5e => success(Opcode::Dup2X2).parse(data),
            0x8d => success(Opcode::F2d).parse(data),
            0x8b => success(Opcode::F2i).parse(data),
            0x8c => success(Opcode::F2l).parse(data),
            0x62 => success(Opcode::Fadd).parse(data),
            0x30 => success(Opcode::Faload).parse(data),
            0x51 => success(Opcode::Fastore).parse(data),
            0x96 => success(Opcode::Fcmpg).parse(data),
            0x95 => success(Opcode::Fcmpl).parse(data),
            0x0b => success(Opcode::Fconst(0)).parse(data),
            0x0c => success(Opcode::Fconst(1)).parse(data),
            0x0d => success(Opcode::Fconst(2)).parse(data),
            0x6e => success(Opcode::Fdiv).parse(data),
            0x17 => map(u8, Opcode::Fload).parse(data),
            0x22 => success(Opcode::Fload(0)).parse(data),
            0x23 => success(Opcode::Fload(1)).parse(data),
            0x24 => success(Opcode::Fload(2)).parse(data),
            0x25 => success(Opcode::Fload(3)).parse(data),
            0x6a => success(Opcode::Fmul).parse(data),
            0x76 => success(Opcode::Fneg).parse(data),
            0x72 => success(Opcode::Frem).parse(data),
            0xae => success(Opcode::Freturn).parse(data),
            0x38 => map(u8, Opcode::Fstore).parse(data),
            0x43 => success(Opcode::Fstore(0)).parse(data),
            0x44 => success(Opcode::Fstore(1)).parse(data),
            0x45 => success(Opcode::Fstore(2)).parse(data),
            0x46 => success(Opcode::Fstore(3)).parse(data),
            0x66 => success(Opcode::Fsub).parse(data),
            0xb4 => map(be_u16, |x| {
                Opcode::Getfield(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0xb2 => map(be_u16, |x| {
                Opcode::Getstatic(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0xa7 => map(be_i16, Opcode::Goto).parse(data),
            0xc8 => map(be_i32, Opcode::GotoW).parse(data),
            0x91 => success(Opcode::I2b).parse(data),
            0x92 => success(Opcode::I2c).parse(data),
            0x87 => success(Opcode::I2d).parse(data),
            0x86 => success(Opcode::I2f).parse(data),
            0x85 => success(Opcode::I2l).parse(data),
            0x93 => success(Opcode::I2s).parse(data),
            0x60 => success(Opcode::Iadd).parse(data),
            0x2e => success(Opcode::Iaload).parse(data),
            0x7e => success(Opcode::Iand).parse(data),
            0x4f => success(Opcode::Iastore).parse(data),
            0x02 => success(Opcode::Iconst(-1)).parse(data),
            0x03 => success(Opcode::Iconst(0)).parse(data),
            0x04 => success(Opcode::Iconst(1)).parse(data),
            0x05 => success(Opcode::Iconst(2)).parse(data),
            0x06 => success(Opcode::Iconst(3)).parse(data),
            0x07 => success(Opcode::Iconst(4)).parse(data),
            0x08 => success(Opcode::Iconst(5)).parse(data),
            0x6c => success(Opcode::Idiv).parse(data),
            0xa5 => map(be_i16, Opcode::IfAcmpeq).parse(data),
            0xa6 => map(be_i16, Opcode::IfAcmpne).parse(data),
            0x9f => map(be_i16, Opcode::IfIcmpeq).parse(data),
            0xa0 => map(be_i16, Opcode::IfIcmpne).parse(data),
            0xa1 => map(be_i16, Opcode::IfIcmplt).parse(data),
            0xa2 => map(be_i16, Opcode::IfIcmpge).parse(data),
            0xa3 => map(be_i16, Opcode::IfIcmpgt).parse(data),
            0xa4 => map(be_i16, Opcode::IfIcmple).parse(data),
            0x99 => map(be_i16, Opcode::Ifeq).parse(data),
            0x9a => map(be_i16, Opcode::Ifne).parse(data),
            0x9b => map(be_i16, Opcode::Iflt).parse(data),
            0x9c => map(be_i16, Opcode::Ifge).parse(data),
            0x9d => map(be_i16, Opcode::Ifgt).parse(data),
            0x9e => map(be_i16, Opcode::Ifle).parse(data),
            0xc7 => map(be_i16, Opcode::Ifnonnull).parse(data),
            0xc6 => map(be_i16, Opcode::Ifnull).parse(data),
            0x84 => map((u8, i8), |(index, constant)| Opcode::Iinc(index, constant)).parse(data),
            0x15 => map(u8, Opcode::Iload).parse(data),
            0x1a => success(Opcode::Iload(0)).parse(data),
            0x1b => success(Opcode::Iload(1)).parse(data),
            0x1c => success(Opcode::Iload(2)).parse(data),
            0x1d => success(Opcode::Iload(3)).parse(data),
            0x68 => success(Opcode::Imul).parse(data),
            0x74 => success(Opcode::Ineg).parse(data),
            0xc1 => map(be_u16, |x| {
                Opcode::Instanceof(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0xba => map((be_u16, be_u16), |(x, _)| {
                Opcode::Invokedynamic(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0xb9 => map((be_u16, u8, u8), |(x, count, zero)| {
                Opcode::Invokeinterface(ConstantPoolReference::from_constant_pool(constant_pool, x as _), count, zero)
            })
            .parse(data),
            0xb7 => map(be_u16, |x| {
                Opcode::Invokespecial(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0xb8 => map(be_u16, |x| {
                Opcode::Invokestatic(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0xb6 => map(be_u16, |x| {
                Opcode::Invokevirtual(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0x80 => success(Opcode::Ior).parse(data),
            0x70 => success(Opcode::Irem).parse(data),
            0xac => success(Opcode::Ireturn).parse(data),
            0x78 => success(Opcode::Ishl).parse(data),
            0x7a => success(Opcode::Ishr).parse(data),
            0x36 => map(u8, Opcode::Istore).parse(data),
            0x3b => success(Opcode::Istore(0)).parse(data),
            0x3c => success(Opcode::Istore(1)).parse(data),
            0x3d => success(Opcode::Istore(2)).parse(data),
            0x3e => success(Opcode::Istore(3)).parse(data),
            0x64 => success(Opcode::Isub).parse(data),
            0x7c => success(Opcode::Iushr).parse(data),
            0x82 => success(Opcode::Ixor).parse(data),
            0xa8 => map(be_i16, Opcode::Jsr).parse(data),
            0xc9 => map(be_i32, Opcode::JsrW).parse(data),
            0x8a => success(Opcode::L2d).parse(data),
            0x89 => success(Opcode::L2f).parse(data),
            0x88 => success(Opcode::L2i).parse(data),
            0x61 => success(Opcode::Ladd).parse(data),
            0x2f => success(Opcode::Laload).parse(data),
            0x7f => success(Opcode::Land).parse(data),
            0x50 => success(Opcode::Lastore).parse(data),
            0x94 => success(Opcode::Lcmp).parse(data),
            0x09 => success(Opcode::Lconst(0)).parse(data),
            0x0a => success(Opcode::Lconst(1)).parse(data),
            0x12 => map(u8, |x| Opcode::Ldc(ConstantPoolReference::from_constant_pool(constant_pool, x as _))).parse(data),
            0x13 => map(be_u16, |x| Opcode::LdcW(ConstantPoolReference::from_constant_pool(constant_pool, x as _))).parse(data),
            0x14 => map(be_u16, |x| {
                Opcode::Ldc2W(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0x6d => success(Opcode::Ldiv).parse(data),
            0x16 => map(u8, Opcode::Lload).parse(data),
            0x1e => success(Opcode::Lload(0)).parse(data),
            0x1f => success(Opcode::Lload(1)).parse(data),
            0x20 => success(Opcode::Lload(2)).parse(data),
            0x21 => success(Opcode::Lload(3)).parse(data),
            0x69 => success(Opcode::Lmul).parse(data),
            0x75 => success(Opcode::Lneg).parse(data),
            0xab => flat_map((take((4 - (offset + 1) % 4) % 4), be_i32, be_i32), |(_, default, npairs)| {
                move |x| map(count((be_i32, be_i32), npairs as _), |offsets| Opcode::Lookupswitch(default, offsets)).parse(x)
            })
            .parse(data),
            0x81 => success(Opcode::Lor).parse(data),
            0x71 => success(Opcode::Lrem).parse(data),
            0xad => success(Opcode::Lreturn).parse(data),
            0x79 => success(Opcode::Lshl).parse(data),
            0x7b => success(Opcode::Lshr).parse(data),
            0x37 => map(u8, Opcode::Lstore).parse(data),
            0x3f => success(Opcode::Lstore(0)).parse(data),
            0x40 => success(Opcode::Lstore(1)).parse(data),
            0x41 => success(Opcode::Lstore(2)).parse(data),
            0x42 => success(Opcode::Lstore(3)).parse(data),
            0x65 => success(Opcode::Lsub).parse(data),
            0x7d => success(Opcode::Lushr).parse(data),
            0x83 => success(Opcode::Lxor).parse(data),
            0xc2 => success(Opcode::Monitorenter).parse(data),
            0xc3 => success(Opcode::Monitorexit).parse(data),
            0xc5 => map((be_u16, u8), |(index, dimensions)| {
                Opcode::Multianewarray(ConstantPoolReference::from_constant_pool(constant_pool, index as _), dimensions)
            })
            .parse(data),
            0xbb => map(be_u16, |x| Opcode::New(ConstantPoolReference::from_constant_pool(constant_pool, x as _))).parse(data),
            0xbc => map(u8, Opcode::Newarray).parse(data),
            0x00 => success(Opcode::Nop).parse(data),
            0x57 => success(Opcode::Pop).parse(data),
            0x58 => success(Opcode::Pop2).parse(data),
            0xb5 => map(be_u16, |x| {
                Opcode::Putfield(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0xb3 => map(be_u16, |x| {
                Opcode::Putstatic(ConstantPoolReference::from_constant_pool(constant_pool, x as _))
            })
            .parse(data),
            0xa9 => map(u8, Opcode::Ret).parse(data),
            0xb1 => success(Opcode::Return).parse(data),
            0x35 => success(Opcode::Saload).parse(data),
            0x56 => success(Opcode::Sastore).parse(data),
            0x11 => map(be_i16, Opcode::Sipush).parse(data),
            0x5f => success(Opcode::Swap).parse(data),
            0xaa => flat_map((take((4 - (offset + 1) % 4) % 4), be_i32, be_i32, be_i32), |(_, default, low, high)| {
                move |x| {
                    map(count(be_i32, ((high - low) + 1) as _), |offsets| {
                        Opcode::Tableswitch(default, (low..=high).zip(offsets).collect())
                    })
                    .parse(x)
                }
            })
            .parse(data),
            0xc4 => success(Opcode::Wide).parse(data),
            _ => panic!("Unknown opcode: {:02x}", opcode),
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::{collections::BTreeMap, string::ToString, sync::Arc};

    use super::Opcode;
    use crate::constant_pool::ConstantPoolItem;

    fn constant_pool() -> BTreeMap<u16, ConstantPoolItem> {
        [
            (1, ConstantPoolItem::Utf8(Arc::new("Foo".to_string()))),
            (2, ConstantPoolItem::Class { name_index: 1 }),
            (3, ConstantPoolItem::Utf8(Arc::new("bar".to_string()))),
            (4, ConstantPoolItem::Utf8(Arc::new("()V".to_string()))),
            (
                5,
                ConstantPoolItem::NameAndType {
                    name_index: 3,
                    descriptor_index: 4,
                },
            ),
            (
                6,
                ConstantPoolItem::InterfaceMethodref {
                    class_index: 2,
                    name_and_type_index: 5,
                },
            ),
            (
                7,
                ConstantPoolItem::Methodref {
                    class_index: 2,
                    name_and_type_index: 5,
                },
            ),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn test_invokeinterface_consumes_count_and_zero() {
        let (remaining, opcode) = Opcode::parse(&[0xb9, 0x00, 0x06, 0x01, 0x00], 0, &constant_pool()).unwrap();

        assert!(remaining.is_empty());
        assert!(matches!(opcode, Opcode::Invokeinterface(_, 1, 0)));
    }

    #[test]
    fn test_invokedynamic_consumes_reserved_bytes() {
        let (remaining, opcode) = Opcode::parse(&[0xba, 0x00, 0x07, 0x00, 0x00], 0, &constant_pool()).unwrap();

        assert!(remaining.is_empty());
        assert!(matches!(opcode, Opcode::Invokedynamic(_)));
    }
}
