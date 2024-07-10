//! Implements helper traits for statements.

#[cfg(not(feature = "references"))]
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};
use std::{
    marker::PhantomData,
    ops::Index,
    sync::{Arc, LockResult, Mutex, MutexGuard, PoisonError, RwLock},
};
use tree_sitter::Node;

use crate::{
    prelude::{ReadGuard, StatementInner, Statements, StatementsIter, WriteGuard},
    types::{
        CompoundSetExpression, DoBlock, Expression, FromNode, FunctionCall, GenericFor,
        GlobalFunction, IfStatement, LastStatement, LocalAssignment, LocalFunction, LuauStatement,
        NumericalFor, RepeatBlock, SetExpression, Statement, Token, TypeDefinition, WhileLoop,
    },
    utils::map_option,
};

/// Creates code that calls `try_from_node` for each of the passed element and return
/// the first one that works, or `None`.
macro_rules! __handle_statement {
    ({ $statement: ident, $code_bytes: ident }, $first_name: ident $(, $name: ident)* $(,)?) => {{
        let mut cursor = $statement.walk();

        if let Some(statement) = $first_name::try_from_node($statement, &mut cursor, $code_bytes) {
            Some(Self::$first_name(statement))
        } $(else if let Some(statement) = $name::try_from_node($statement, &mut cursor, $code_bytes) {
            Some(Self::$name(statement))
        })* else {
            None
        }
    }};
}

impl FromNode for LastStatement {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        let semicolon = map_option(node.child_by_field_name("semicolon"), |semicolon| {
            Token::from_node(semicolon?, code_bytes)
        });
        let node = node.child(0)?;

        match node.kind() {
            "break" => Some(Self::Break((
                Token::from_node(node, code_bytes)?,
                semicolon,
            ))),
            "continue" => Some(Self::Continue((
                Token::from_node(node, code_bytes)?,
                semicolon,
            ))),
            "return_statement" => Some(Self::Return {
                return_keyword: Token::from_node(node.child(0)?, code_bytes)?,
                expressions: Expression::from_nodes(
                    node.children_by_field_name("expressions", &mut node.walk()),
                    code_bytes,
                ),
                semicolon,
            }),
            _ => unreachable!(),
        }
    }
}

impl FromNode for Statement {
    fn from_node(statement: Node, code_bytes: &[u8]) -> Option<Self> {
        __handle_statement!(
            { statement, code_bytes },
            CompoundSetExpression,
            DoBlock,
            FunctionCall,
            GenericFor,
            GlobalFunction,
            IfStatement,
            LocalAssignment,
            LocalFunction,
            NumericalFor,
            RepeatBlock,
            SetExpression,
            TypeDefinition,
            WhileLoop,
        )
    }
}

impl<'a> StatementsIter<'a> {
    pub(crate) fn new(vec: ReadGuard<'a>) -> Self {
        Self {
            index: 0,
            end: vec.len(),
            vec,
        }
    }
}

impl<'a> Iterator for StatementsIter<'a> {
    type Item = &'a StatementInner;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        #[allow(unsafe_code)] // just the iterators .-.
        // Safety: we are extending the lifetime of the reference to 'a because we
        // know that the Arc will keep the Vec alive for 'a.
        self.vec
            .get(index)
            .map(|item| unsafe { &*(item as *const StatementInner) })
    }
}

impl<'a> DoubleEndedIterator for StatementsIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == 0 {
            return None;
        }
        self.end -= 1;

        #[allow(unsafe_code)] // just the iterators .-.
        // Safety: we are extending the lifetime of the reference to 'a
        // because we know that the Arc will keep the Vec alive for 'a.
        self.vec
            .get(self.end)
            .map(|item| unsafe { &*(item as *const StatementInner) })
    }
}

impl Statements {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.read().unwrap().len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&self) -> StatementsIter {
        StatementsIter::new(self.0.read().unwrap())
    }

    #[inline]
    pub fn push(&self, statement: StatementInner) {
        self.0.write().unwrap().push(statement)
    }

    #[inline]
    pub fn read(&self) -> LockResult<ReadGuard> {
        self.0.read()
    }

    #[inline]
    pub fn write(&self) -> LockResult<WriteGuard> {
        self.0.write()
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<&StatementInner> {
        self.iter().nth(i)
    }
}

#[cfg(not(feature = "references"))]
impl Hash for Statements {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Lock the mutex and handle the result
        let vec = self.0.read().unwrap();

        for (statement, token) in vec.iter() {
            statement.hash(state);
            token.hash(state);
        }
    }
}

#[cfg(not(feature = "references"))]
impl PartialEq for Statements {
    fn eq(&self, other: &Self) -> bool {
        let self_vec = self.0.read().unwrap();
        let other_vec = other.0.read().unwrap();
        *self_vec == *other_vec
    }
}

#[cfg(not(feature = "references"))]
impl Eq for Statements {}

#[cfg(not(feature = "references"))]
impl PartialOrd for Statements {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_vec = self.0.read().unwrap();
        let other_vec = other.0.read().unwrap();
        self_vec.partial_cmp(&other_vec)
    }
}

#[cfg(not(feature = "references"))]
impl Ord for Statements {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_vec = self.0.read().unwrap();
        let other_vec = other.0.read().unwrap();
        self_vec.cmp(&other_vec)
    }
}

impl Clone for Statements {
    fn clone(&self) -> Self {
        let vec = self.0.read().unwrap().clone();
        Statements(Arc::new(RwLock::new(vec)))
    }
}

impl Index<usize> for Statements {
    type Output = StatementInner;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .expect("Attempt to index Statements with an out-of-bound index.")
    }
}
