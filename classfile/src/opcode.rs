use alloc::vec::Vec;

use nom::number::complete::u8;
use nom_derive::{NomBE, Parse};

#[derive(NomBE, Clone)]
#[nom(Selector = "u8")]

pub enum Opcode {
    #[nom(Selector = "50")]
    Aaload,
    #[nom(Selector = "83")]
    Aastore,
    #[nom(Selector = "1")]
    AconstNull,
    #[nom(Selector = "25")]
    Aload(u8),
    #[nom(Selector = "42")]
    Aload0,
    #[nom(Selector = "43")]
    Aload1,
    #[nom(Selector = "44")]
    Aload2,
    #[nom(Selector = "45")]
    Aload3,
    #[nom(Selector = "189")]
    Anewarray(u16),
    #[nom(Selector = "176")]
    Areturn,
    #[nom(Selector = "190")]
    Arraylength,
    #[nom(Selector = "58")]
    Astore(u8),
    #[nom(Selector = "75")]
    Astore0,
    #[nom(Selector = "76")]
    Astore1,
    #[nom(Selector = "77")]
    Astore2,
    #[nom(Selector = "78")]
    Astore3,
    #[nom(Selector = "191")]
    Athrow,
    #[nom(Selector = "51")]
    Baload,
    #[nom(Selector = "84")]
    Bastore,
    #[nom(Selector = "16")]
    Bipush(i8),
    #[nom(Selector = "52")]
    Caload,
    #[nom(Selector = "85")]
    Castore,
    #[nom(Selector = "192")]
    Checkcast(u16),
    #[nom(Selector = "144")]
    D2f,
    #[nom(Selector = "142")]
    D2i,
    #[nom(Selector = "143")]
    D2l,
    #[nom(Selector = "99")]
    Dadd,
    #[nom(Selector = "49")]
    Daload,
    #[nom(Selector = "82")]
    Dastore,
    #[nom(Selector = "152")]
    Dcmpg,
    #[nom(Selector = "151")]
    Dcmpl,
    #[nom(Selector = "14")]
    Dconst0,
    #[nom(Selector = "15")]
    Dconst1,
    #[nom(Selector = "111")]
    Ddiv,
    #[nom(Selector = "24")]
    Dload(u8),
    #[nom(Selector = "38")]
    Dload0,
    #[nom(Selector = "39")]
    Dload1,
    #[nom(Selector = "40")]
    Dload2,
    #[nom(Selector = "41")]
    Dload3,
    #[nom(Selector = "107")]
    Dmul,
    #[nom(Selector = "119")]
    Dneg,
    #[nom(Selector = "115")]
    Drem,
    #[nom(Selector = "175")]
    Dreturn,
    #[nom(Selector = "57")]
    Dstore(u8),
    #[nom(Selector = "71")]
    Dstore0,
    #[nom(Selector = "72")]
    Dstore1,
    #[nom(Selector = "73")]
    Dstore2,
    #[nom(Selector = "74")]
    Dstore3,
    #[nom(Selector = "103")]
    Dsub,
    #[nom(Selector = "89")]
    Dup,
    #[nom(Selector = "90")]
    DupX1,
    #[nom(Selector = "91")]
    DupX2,
    #[nom(Selector = "92")]
    Dup2,
    #[nom(Selector = "93")]
    Dup2X1,
    #[nom(Selector = "94")]
    Dup2X2,
    #[nom(Selector = "141")]
    F2d,
    #[nom(Selector = "139")]
    F2i,
    #[nom(Selector = "140")]
    F2l,
    #[nom(Selector = "98")]
    Fadd,
    #[nom(Selector = "48")]
    Faload,
    #[nom(Selector = "81")]
    Fastore,
    #[nom(Selector = "150")]
    Fcmpg,
    #[nom(Selector = "149")]
    Fcmpl,
    #[nom(Selector = "11")]
    Fconst0,
    #[nom(Selector = "12")]
    Fconst1,
    #[nom(Selector = "13")]
    Fconst2,
    #[nom(Selector = "110")]
    Fdiv,
    #[nom(Selector = "23")]
    Fload(u8),
    #[nom(Selector = "34")]
    Fload0,
    #[nom(Selector = "35")]
    Fload1,
    #[nom(Selector = "36")]
    Fload2,
    #[nom(Selector = "37")]
    Fload3,
    #[nom(Selector = "106")]
    Fmul,
    #[nom(Selector = "118")]
    Fneg,
    #[nom(Selector = "114")]
    Frem,
    #[nom(Selector = "174")]
    Freturn,
    #[nom(Selector = "56")]
    Fstore(u8),
    #[nom(Selector = "67")]
    Fstore0,
    #[nom(Selector = "68")]
    Fstore1,
    #[nom(Selector = "69")]
    Fstore2,
    #[nom(Selector = "70")]
    Fstore3,
    #[nom(Selector = "102")]
    Fsub,
    #[nom(Selector = "180")]
    Getfield(u16),
    #[nom(Selector = "178")]
    Getstatic(u16),
    #[nom(Selector = "167")]
    Goto(i16),
    #[nom(Selector = "200")]
    GotoW(i32),
    #[nom(Selector = "145")]
    I2b,
    #[nom(Selector = "146")]
    I2c,
    #[nom(Selector = "135")]
    I2d,
    #[nom(Selector = "134")]
    I2f,
    #[nom(Selector = "133")]
    I2l,
    #[nom(Selector = "147")]
    I2s,
    #[nom(Selector = "96")]
    Iadd,
    #[nom(Selector = "46")]
    Iaload,
    #[nom(Selector = "126")]
    Iand,
    #[nom(Selector = "79")]
    Iastore,
    #[nom(Selector = "2")]
    IconstM1,
    #[nom(Selector = "3")]
    Iconst0,
    #[nom(Selector = "4")]
    Iconst1,
    #[nom(Selector = "5")]
    Iconst2,
    #[nom(Selector = "6")]
    Iconst3,
    #[nom(Selector = "7")]
    Iconst4,
    #[nom(Selector = "8")]
    Iconst5,
    #[nom(Selector = "108")]
    Idiv,
    #[nom(Selector = "165")]
    IfAcmpeq(i16),
    #[nom(Selector = "166")]
    IfAcmpne(i16),
    #[nom(Selector = "159")]
    IfIcmpeq(i16),
    #[nom(Selector = "160")]
    IfIcmpne(i16),
    #[nom(Selector = "161")]
    IfIcmplt(i16),
    #[nom(Selector = "162")]
    IfIcmpge(i16),
    #[nom(Selector = "163")]
    IfIcmpgt(i16),
    #[nom(Selector = "164")]
    IfIcmple(i16),
    #[nom(Selector = "153")]
    Ifeq(i16),
    #[nom(Selector = "154")]
    Ifne(i16),
    #[nom(Selector = "155")]
    Iflt(i16),
    #[nom(Selector = "156")]
    Ifge(i16),
    #[nom(Selector = "157")]
    Ifgt(i16),
    #[nom(Selector = "158")]
    Ifle(i16),
    #[nom(Selector = "199")]
    Ifnonnull(i16),
    #[nom(Selector = "198")]
    Ifnull(i16),
    #[nom(Selector = "132")]
    Iinc(u8, i8),
    #[nom(Selector = "21")]
    Iload(u8),
    #[nom(Selector = "26")]
    Iload0,
    #[nom(Selector = "27")]
    Iload1,
    #[nom(Selector = "28")]
    Iload2,
    #[nom(Selector = "29")]
    Iload3,
    #[nom(Selector = "104")]
    Imul,
    #[nom(Selector = "116")]
    Ineg,
    #[nom(Selector = "193")]
    Instanceof(u16),
    #[nom(Selector = "186")]
    Invokedynamic(u16, u16),
    #[nom(Selector = "185")]
    Invokeinterface(u16, u8),
    #[nom(Selector = "183")]
    Invokespecial(u16),
    #[nom(Selector = "184")]
    Invokestatic(u16),
    #[nom(Selector = "182")]
    Invokevirtual(u16),
    #[nom(Selector = "128")]
    Ior,
    #[nom(Selector = "112")]
    Irem,
    #[nom(Selector = "172")]
    Ireturn,
    #[nom(Selector = "120")]
    Ishl,
    #[nom(Selector = "122")]
    Ishr,
    #[nom(Selector = "54")]
    Istore(u8),
    #[nom(Selector = "59")]
    Istore0,
    #[nom(Selector = "60")]
    Istore1,
    #[nom(Selector = "61")]
    Istore2,
    #[nom(Selector = "62")]
    Istore3,
    #[nom(Selector = "100")]
    Isub,
    #[nom(Selector = "124")]
    Iushr,
    #[nom(Selector = "130")]
    Ixor,
    #[nom(Selector = "168")]
    Jsr(i16),
    #[nom(Selector = "201")]
    JsrW(i32),
    #[nom(Selector = "138")]
    L2d,
    #[nom(Selector = "137")]
    L2f,
    #[nom(Selector = "136")]
    L2i,
    #[nom(Selector = "97")]
    Ladd,
    #[nom(Selector = "47")]
    Laload,
    #[nom(Selector = "127")]
    Land,
    #[nom(Selector = "80")]
    Lastore,
    #[nom(Selector = "148")]
    Lcmp,
    #[nom(Selector = "9")]
    Lconst0,
    #[nom(Selector = "10")]
    Lconst1,
    #[nom(Selector = "18")]
    Ldc(u8),
    #[nom(Selector = "19")]
    LdcW(u16),
    #[nom(Selector = "20")]
    Ldc2W(u16),
    #[nom(Selector = "109")]
    Ldiv,
    #[nom(Selector = "22")]
    Lload(u8),
    #[nom(Selector = "30")]
    Lload0,
    #[nom(Selector = "31")]
    Lload1,
    #[nom(Selector = "32")]
    Lload2,
    #[nom(Selector = "33")]
    Lload3,
    #[nom(Selector = "105")]
    Lmul,
    #[nom(Selector = "117")]
    Lneg,
    #[nom(Selector = "171")]
    Lookupswitch(i32, Vec<(i32, i32)>),
    #[nom(Selector = "129")]
    Lor,
    #[nom(Selector = "113")]
    Lrem,
    #[nom(Selector = "173")]
    Lreturn,
    #[nom(Selector = "121")]
    Lshl,
    #[nom(Selector = "123")]
    Lshr,
    #[nom(Selector = "55")]
    Lstore(u8),
    #[nom(Selector = "63")]
    Lstore0,
    #[nom(Selector = "64")]
    Lstore1,
    #[nom(Selector = "65")]
    Lstore2,
    #[nom(Selector = "66")]
    Lstore3,
    #[nom(Selector = "101")]
    Lsub,
    #[nom(Selector = "125")]
    Lushr,
    #[nom(Selector = "131")]
    Lxor,
    #[nom(Selector = "194")]
    Multianewarray(u16, u8),
    #[nom(Selector = "187")]
    New(u16),
    #[nom(Selector = "188")]
    Newarray(u8),
    #[nom(Selector = "0")]
    Nop,
    #[nom(Selector = "87")]
    Pop,
    #[nom(Selector = "88")]
    Pop2,
    #[nom(Selector = "181")]
    Putfield(u16),
    #[nom(Selector = "179")]
    Putstatic(u16),
    #[nom(Selector = "169")]
    Ret(u8),
    #[nom(Selector = "177")]
    Return,
    #[nom(Selector = "53")]
    Saload,
    #[nom(Selector = "86")]
    Sastore,
    #[nom(Selector = "17")]
    Sipush(i16),
    #[nom(Selector = "95")]
    Swap,
    #[nom(Selector = "170")]
    Tableswitch(i32, i32, Vec<i32>),
    #[nom(Selector = "196")]
    Wide,
}

impl Opcode {
    pub fn parse_with_tag(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (remaining, tag) = u8(data)?;

        Opcode::parse(remaining, tag)
    }
}
