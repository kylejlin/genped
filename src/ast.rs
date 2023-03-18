#[derive(Debug, Clone)]
pub struct Spec {
    pub statements: Statements,
}

#[derive(Debug, Clone)]
pub enum Statements {
    Nil,
    Cons(Box<Statements>, Statement),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Struct(Struct),
    Enum(Enum),
    Use(Use),
    TokenTypeDecl(TokenTypeDecl),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub opt_derive_clause: OptDeriveClause,
    pub struct_kw: StructKeyword,
    pub name: Ident,
    pub body: StructBody,
}

#[derive(Debug, Clone)]
pub enum OptDeriveClause {
    None,
    Some(DeriveClause),
}

// TODO
#[derive(Debug, Clone)]
pub struct DeriveClause {}

#[derive(Debug, Clone)]
pub struct StructKeyword;

#[derive(Debug, Clone)]
pub struct LCurly;

#[derive(Debug, Clone)]
pub struct RCurly;

#[derive(Debug, Clone)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub enum StructBody {
    Curly(CurlyStructBody),
    // TODO: Support tuple-like and unit-like structs.
}

#[derive(Debug, Clone)]
pub struct CurlyStructBody {
    pub lcurly: LCurly,
    pub fields: CurlyStructFields,
    pub rcurly: RCurly,
}

#[derive(Debug, Clone)]
pub enum CurlyStructFields {
    Nil,
    Cons(Box<CurlyStructFields>, CurlyStructField),
}

#[derive(Debug, Clone)]
pub struct CurlyStructField {
    pub name: Ident,
    pub colon: Colon,
    pub ty: Type,
    pub comma: Comma,
}

#[derive(Debug, Clone)]
pub struct Colon;

#[derive(Debug, Clone)]
pub struct Comma;

#[derive(Debug, Clone)]
pub enum Type {
    Name(Ident),
    Generic(GenericType),
}

#[derive(Debug, Clone)]
pub struct GenericType {
    pub name: Ident,
    pub lt: Lt,
    pub args: CommaSeparatedTypes,
    pub opt_comma: OptComma,
    pub gt: Gt,
}

#[derive(Debug, Clone)]
pub struct Lt;

#[derive(Debug, Clone)]
pub struct Gt;

#[derive(Debug, Clone)]
pub enum CommaSeparatedTypes {
    Unit(Box<Type>),
    Cons(Box<CommaSeparatedTypes>, Comma, Box<Type>),
}

#[derive(Debug, Clone)]
pub enum OptComma {
    None,
    Some(Comma),
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub opt_derive_clause: OptDeriveClause,
    pub enum_kw: EnumKeyword,
    pub name: Ident,
    pub lcurly: LCurly,
    pub variants: EnumVariants,
    pub rcurly: RCurly,
}

#[derive(Debug, Clone)]
pub struct EnumKeyword;

#[derive(Debug, Clone)]
pub enum EnumVariants {
    Nil,
    Cons(Box<EnumVariants>, EnumVariant),
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: Ident,
    pub opt_params: OptEnumVariantParams,
    pub comma: Comma,
}

#[derive(Debug, Clone)]
pub enum OptEnumVariantParams {
    None,
    Some(EnumVariantParams),
}

#[derive(Debug, Clone)]
pub struct EnumVariantParams {
    pub lparen: LParen,
    pub fields: CommaSeparatedTypes,
    pub opt_comma: OptComma,
    pub rparen: RParen,
}

#[derive(Debug, Clone)]
pub struct LParen;

#[derive(Debug, Clone)]
pub struct RParen;

#[derive(Debug, Clone)]
pub struct Use {
    pub use_kw: UseKeyword,
    pub path: Path,
    pub semicolon: Semicolon,
}

#[derive(Debug, Clone)]
pub struct UseKeyword;

#[derive(Debug, Clone)]
pub enum Path {
    Unit(Ident),
    Cons(Box<Path>, Colon, Colon, Ident),
}

#[derive(Debug, Clone)]
pub struct Semicolon;

#[derive(Debug, Clone)]
pub struct TokenTypeDecl {
    pub dollar: Dollar,
    pub equal: Equal,
    pub enum_kw: EnumKeyword,
    pub name: Ident,
    pub lcurly: LCurly,
    pub variants: EnumVariants,
    pub rcurly: RCurly,
}

#[derive(Debug, Clone)]
pub struct Dollar;

#[derive(Debug, Clone)]
pub struct Equal;
