// use zinq_parse::{Parse, Peek, Span};
// use zinq_token::Use;

// use crate::{Node, Path, Visibility, fields::Fields, stmt::Stmt};

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct UseStmt {
//     pub span: Span,
//     pub vis: Visibility,
//     pub keyword: Use,
//     pub path: Path,
//     pub fields: Fields,
// }

// impl From<UseStmt> for Stmt {
//     fn from(value: UseStmt) -> Self {
//         Self::Use(value)
//     }
// }

// impl Node for UseStmt {
//     fn name(&self) -> &str {
//         "Syntax::Stmt::Use"
//     }

//     fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
//     where
//         Self: Sized,
//     {
//         visitor.visit(self)
//     }
// }

// impl std::fmt::Display for UseStmt {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", &self.span)
//     }
// }

// impl Peek for UseStmt {
//     fn peek(
//         cursor: &zinq_parse::Cursor,
//         parser: &zinq_parse::ZinqParser,
//     ) -> zinq_error::Result<bool> {
//         let mut fork = cursor.fork();
//         let mut fork_parser = parser.clone();

//         match fork_parser.parse::<Self>(&mut fork) {
//             Err(_) => Ok(false),
//             Ok(_) => Ok(true),
//         }
//     }
// }

// impl Parse for UseStmt {
//     fn parse(
//         cursor: &mut zinq_parse::Cursor,
//         parser: &mut zinq_parse::ZinqParser,
//     ) -> zinq_error::Result<Self> {
//         let vis = parser.parse::<Visibility>(cursor)?;
//         let keyword = parser.parse::<Use>(cursor)?;
//         let path = parser.parse::<Path>(cursor)?;
//         let fields = parser.parse::<Fields>(cursor)?;
//         let span = Span::from_bounds(vis.span(), fields.span());

//         Ok(Self {
//             span,
//             vis,
//             keyword,
//             path,
//             fields,
//         })
//     }

//     fn span(&self) -> &Span {
//         &self.span
//     }
// }
