use super::{
    Instruction, InstructionKind, Operation, OperationKind, Program, Test,
    TestKind,
};
use crate::interpreter::table::SymbolTable;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct ProgramDisplayer<'regs, 'prog> {
    pub target: &'prog Program,
    pub register_table: &'regs SymbolTable,
}

impl<'regs, 'prog> fmt::Display for ProgramDisplayer<'regs, 'prog> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let context = InstrContext {
            register_table: self.register_table,
            // TODO
            label_table: &SymbolTable::empty(),
        };
        for instruction in self.target.instructions.values() {
            write!(fmtr, "{}\n", context.display(instruction.clone()))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InstrContext<'regs, 'labels> {
    pub register_table: &'regs SymbolTable,
    pub label_table: &'labels SymbolTable,
}

impl<'regs, 'labels> InstrContext<'regs, 'labels> {
    pub fn display<T>(self, target: T) -> InstrDisplayer<'regs, 'labels, T> {
        InstrDisplayer { target, context: self }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InstrDisplayer<'regs, 'labels, T> {
    pub target: T,
    pub context: InstrContext<'regs, 'labels>,
}

impl<'regs, 'labels> fmt::Display
    for InstrDisplayer<'regs, 'labels, Instruction>
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "{}: {}",
            self.target.label,
            self.context.display(self.target.kind.clone())
        )
    }
}

impl<'regs, 'labels> fmt::Display
    for InstrDisplayer<'regs, 'labels, InstructionKind>
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self.target.clone() {
            InstructionKind::Operation(oper) => {
                write!(fmtr, "{}", self.context.display(oper))
            }
            InstructionKind::Test(test) => {
                write!(fmtr, "{}", self.context.display(test))
            }
        }
    }
}

impl<'regs, 'labels> fmt::Display
    for InstrDisplayer<'regs, 'labels, Operation>
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "do {} goto {}",
            self.context.display(self.target.kind.clone()),
            self.target.next,
        )
    }
}

impl<'regs, 'labels> fmt::Display
    for InstrDisplayer<'regs, 'labels, OperationKind>
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self.target.clone() {
            OperationKind::Inc(register) => write!(fmtr, "inc {}", register),
            OperationKind::Dec(register) => write!(fmtr, "dec {}", register),
            OperationKind::Clear(register) => {
                write!(fmtr, "clear ({})", register)
            }
            OperationKind::Load(register, constant) => {
                write!(fmtr, "load ({}, {})", register, constant)
            }
            OperationKind::AddConst(register, constant) => {
                write!(fmtr, "add ({}, {})", register, constant)
            }
            OperationKind::Add(reg_src, reg_dest, reg_tmp) => {
                write!(fmtr, "add ({}, {}, {})", reg_src, reg_dest, reg_tmp)
            }
            OperationKind::SubConst(register, constant) => {
                write!(fmtr, "sub ({}, {})", register, constant)
            }
            OperationKind::Sub(reg_src, reg_dest, reg_tmp) => {
                write!(fmtr, "sub ({}, {}, {})", reg_src, reg_dest, reg_tmp)
            }
        }
    }
}

impl<'regs, 'labels> fmt::Display for InstrDisplayer<'regs, 'labels, Test> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "if {} then goto {} else goto {}",
            self.context.display(self.target.kind.clone()),
            self.target.next_then,
            self.target.next_else
        )
    }
}

impl<'regs, 'labels> fmt::Display for InstrDisplayer<'regs, 'labels, TestKind> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self.target.clone() {
            TestKind::Zero(register) => write!(fmtr, "zero {}", register),
            TestKind::EqualsConst(register, constant) => {
                write!(fmtr, "equals ({}, {})", register, constant)
            }
            TestKind::Equals(reg_left, reg_right, reg_tmp) => write!(
                fmtr,
                "equals ({}, {}, {})",
                reg_left, reg_right, reg_tmp
            ),
            TestKind::LessThanConst(register, constant) => {
                write!(fmtr, "lessThan ({}, {})", register, constant)
            }
            TestKind::LessThan(reg_left, reg_right, reg_tmp) => write!(
                fmtr,
                "lessThan ({}, {}, {})",
                reg_left, reg_right, reg_tmp
            ),
        }
    }
}
