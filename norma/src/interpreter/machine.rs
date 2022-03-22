#[cfg(test)]
mod test;

use super::table::{self, SymbolTable};
use indexmap::IndexMap;
use num_bigint::BigUint;
use num_traits::identities::Zero;
use std::{cmp::Ordering, slice};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegisterId {
    pub index: usize,
}

impl RegisterId {
    pub const X: Self = Self { index: 0 };
    pub const Y: Self = Self { index: 1 };
}

impl From<usize> for RegisterId {
    fn from(index: usize) -> Self {
        Self { index }
    }
}

impl From<RegisterId> for usize {
    fn from(id: RegisterId) -> Self {
        id.index
    }
}

pub type RegisterTable = SymbolTable<String, RegisterId>;

#[cold]
#[inline(never)]
fn inconsistent_register_table() -> ! {
    panic!("Register table is inconsistent")
}

/// Um registrador da norma (sendo um  número natural arbitrário).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Register {
    /// Valor do registrador em número natural (tradicional da Norma).
    value: BigUint,
}

impl Register {
    /// Cria um novo registrador com o valor desejado
    fn new(number: BigUint) -> Register {
        Register { value: number }
    }

    /// Incrementa o valor do registrador.
    fn inc(&mut self) {
        self.value += 1u8
    }

    /// Decrementa o valor do registrador (caso seja maior que 0).
    fn dec(&mut self) {
        if !self.is_zero() {
            self.value -= 1u8
        }
    }

    /// Verifica se o valor do registrador é zero.
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    /// Limpa o valor do registrador (define-o para zero).
    fn clear(&mut self) {
        self.value.set_zero();
    }

    /// Adiciona uma constante ao registrador.
    fn add(&mut self, constant: &BigUint) {
        self.value += constant;
    }

    /// Subtrai uma constante do registrador. A subtração satura no zero, caso a
    /// constante seja maior que o valor armazenado.
    fn sub(&mut self, constant: &BigUint) {
        if self.value <= *constant {
            self.value.set_zero();
        } else {
            self.value -= constant;
        }
    }

    /// Compara o registrador a uma constante e retorna se o valor armazenado é
    /// menor, igual ou maior à ela.
    fn cmp(&self, constant: &BigUint) -> Ordering {
        self.value.cmp(constant)
    }

    /// Retorna o valor do registrador.
    fn value(&self) -> BigUint {
        self.value.clone()
    }

    /// Define o valor do registrador.
    fn set_value(&mut self, value: BigUint) {
        self.value = value;
    }
}

/// Banco de registradores da Norma.
#[derive(Debug, Clone)]
pub struct Machine {
    register_table: RegisterTable,
    registers: Vec<Register>,
}

impl Default for Machine {
    /// Inicia com ambos X e Y zerados.
    fn default() -> Self {
        Self::new(BigUint::zero())
    }
}

impl Machine {
    /// Inicia um novo banco de regitradores com 2 registradores básicos (X e Y)
    /// e inicia contador: X: Registrador de entrada, receberá o valor
    /// desejado Y: Registrador de saída, armazenará o valor retornado ao fim
    /// da execução
    pub fn new(input: BigUint) -> Machine {
        let mut this = Self {
            register_table: RegisterTable::empty(),
            registers: Vec::new(),
        };
        assert_eq!(this.create_register("X", input), RegisterId::X);
        assert_eq!(this.create_register("Y", BigUint::zero()), RegisterId::Y);
        this
    }

    pub fn try_create_register(
        &mut self,
        name: String,
        data: BigUint,
    ) -> Result<RegisterId, RegisterId> {
        let id = self.register_table.try_create(name)?;
        self.registers.push(Register::new(data));
        Ok(id)
    }

    pub fn create_register(&mut self, name: &str, data: BigUint) -> RegisterId {
        let id = self.register_table.create(name);
        self.registers.push(Register::new(data));
        id
    }

    pub fn insert_register(
        &mut self,
        name: String,
        data: BigUint,
    ) -> RegisterId {
        let id = self.register_table.insert(name);
        self.registers.insert(id.index, Register::new(data));
        id
    }

    pub fn registers(&self) -> Registers {
        Registers {
            names: self.register_table.iter(),
            data: self.registers.iter(),
        }
    }

    pub fn register_table(&self) -> &RegisterTable {
        &self.register_table
    }

    /// Define o valor de entrada (AKA valor do registrador X).
    pub fn input(&mut self, data: BigUint) {
        self.register_mut(RegisterId::X).set_value(data);
    }

    /// Pega o valor de saída (AKA valor do registrador Y).
    pub fn output(&self) -> BigUint {
        self.value(RegisterId::Y)
    }

    /// Retorna se o registrador de dado nome já existe.
    pub fn register_exists(&self, reg_name: &str) -> bool {
        self.register_table.contains_symbol(reg_name)
    }

    /// Limpa todos registradores (define-os para zero).
    pub fn clear_all(&mut self) {
        for register in &mut self.registers {
            register.clear();
        }
    }

    /// Limpa o valor do dado registrador (define-o para zero).
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn clear(&mut self, reg_id: RegisterId) {
        self.register_mut(reg_id).clear();
    }

    /// Incrementa o valor de um registrador existente com nome `reg_name`.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn inc(&mut self, reg_id: RegisterId) {
        self.register_mut(reg_id).inc();
    }

    /// Decrementa o valor de um registrador existente com nome `reg_id`.
    /// Satura em zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn dec(&mut self, reg_id: RegisterId) {
        self.register_mut(reg_id).dec();
    }

    /// Performa uma adição entre registradores.
    ///
    /// É colocado em `dest` o resultado da adição `dest + src`, emulando o
    /// uso do registrador `tmp` como temporário/auxiliar, que será atualizado
    /// para zero.
    ///
    /// # Panics
    /// Invoca `panic!` se qualquer um dos registradores `dest`, `src` ou `tmp`
    /// não existir.
    pub fn add(&mut self, dest: RegisterId, src: RegisterId, tmp: RegisterId) {
        let operand = self.value(src);
        self.register_mut(dest).add(&operand);
        self.register_mut(tmp).clear();
    }

    /// Soma uma constante `constant` ao valor de um registrador existente com
    /// nome `reg_id`. Satura em zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn add_const(&mut self, reg_id: RegisterId, constant: &BigUint) {
        self.register_mut(reg_id).add(constant);
    }

    /// Performa uma subtração entre registradores.
    ///
    /// É colocado em `dest` o resultado da subtração `dest - src`, emulando o
    /// uso do registrador `tmp` como temporário/auxiliar, que será atualizado
    /// para zero.
    ///
    /// # Panics
    /// Invoca `panic!` se qualquer um dos registradores `dest`, `src` ou `tmp`
    /// não existir.
    pub fn sub(&mut self, dest: RegisterId, src: RegisterId, tmp: RegisterId) {
        let operand = self.value(src);
        self.register_mut(dest).sub(&operand);
        self.register_mut(tmp).clear();
    }

    /// Subtrai uma constante `constant` do valor de um registrador existente
    /// com nome `reg_id`. Satura em zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn sub_const(&mut self, reg_id: RegisterId, constant: &BigUint) {
        self.register_mut(reg_id).sub(constant);
    }

    /// Performa uma comparação entre registradores.
    ///
    /// Retorna a ordem (menor/igual/maior) entre `left` e `right`, emulando
    /// o uso do registrador `tmp` como temporário/auxiliar, que será
    /// atualizado para zero.
    ///
    /// # Panics
    /// Invoca `panic!` se qualquer um dos registradores `left`, `right` ou
    /// `tmp` não existir.
    pub fn cmp(
        &mut self,
        reg_left: RegisterId,
        reg_right: RegisterId,
        reg_tmp: RegisterId,
    ) -> Ordering {
        self.register_mut(reg_tmp).clear();
        self.register(reg_left).cmp(&self.register(reg_right).value)
    }

    /// Compara o valor do registrador existente de nome `reg_id` a uma
    /// constante `constant` com nome `reg_id`. Retorna se é menor, igual
    /// ou maior à constante.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn cmp_const(
        &mut self,
        reg_id: RegisterId,
        constant: &BigUint,
    ) -> Ordering {
        self.register(reg_id).cmp(constant)
    }

    /// Testa se o valor do registrador existente de nome `reg_id` é zero.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn is_zero(&self, reg_id: RegisterId) -> bool {
        self.register(reg_id).is_zero()
    }

    /// Retorna o valor de um registrador existente pela sua chave.
    ///
    /// # Panics
    /// Invoca `panic!` se o registrador não existir.
    pub fn value(&self, reg_id: RegisterId) -> BigUint {
        self.register(reg_id).value()
    }

    fn register(&self, reg_id: RegisterId) -> &Register {
        &self.registers[reg_id.index]
    }

    fn register_mut(&mut self, reg_id: RegisterId) -> &mut Register {
        &mut self.registers[reg_id.index]
    }

    /// Exporta os registradores em um mapa de
    /// `nome do registrador -> valor do registrador`, com valor renderizado em
    /// string, para ser exibido em front-end.
    pub fn export_registers(&mut self) -> IndexMap<String, String> {
        self.registers()
            .map(|(name, _, data)| (name.to_owned(), data.to_string()))
            .collect()
    }
}

#[derive(Debug)]
pub struct Registers<'machine> {
    names: table::Symbols<'machine, String, RegisterId>,
    data: slice::Iter<'machine, Register>,
}

impl<'machine> Iterator for Registers<'machine> {
    type Item = (&'machine str, RegisterId, BigUint);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.names.next(), self.data.next()) {
            (Some((name, id)), Some(register)) => {
                Some((name.as_str(), id, register.value()))
            }
            (None, None) => None,
            _ => inconsistent_register_table(),
        }
    }
}
