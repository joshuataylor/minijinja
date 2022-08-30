// use std::collections::BTreeMap;
// use std::fmt;
// use std::sync::Arc;
// use crate::ast::Expr;
// use crate::{AutoEscape, Expression};
// use crate::compiler::Compiler;
//
// use crate::error::Error;
// use crate::key::Key::Str;
// use crate::parser::parse;
// use crate::value::{FunctionArgs, Object, Value};
// use crate::vm::{Context, Frame, FrameBase, simple_eval, State, Vm};
//
// type FuncFunc = dyn Fn(&State, Vec<Value>) -> Result<Value, Error> + Sync + Send + 'static;
//
// /// A boxed macro.
// #[derive(Clone)]
// pub(crate) struct BoxedMacro(pub(crate) Arc<FuncFunc>, pub(crate) &'static str);
//
// /// A utility trait that represents global macros.
// pub trait Macro<Rv = Value, Args = Vec<Value>>: Send + Sync + 'static {
//     /// Calls a macro with the given arguments.
//     fn invoke(&self, env: &State, args: Args) -> Result<Rv, Error>;
// }
//
// // impl BoxedMacro {
// //     /// Creates a value from a boxed Macro.
// //     pub fn to_value(&self) -> Value {
// //         Value::from_object(self.clone())
// //     }
// // }
//
// impl BoxedMacro {
//     /// Creates a new boxed filter.
//     pub fn new<Rv, Args>(name: &str) -> BoxedMacro
//         where
//             Args: FunctionArgs,
//     {
//         BoxedMacro(
//             Arc::new(move |env, args| -> Result<Value, Error> {
//                 // We can execute this here>
//
//                 // f.invoke(env, FunctionArgs::from_values(args)?)
//                 //     .map(Into::into)
//                 Ok(Value::from("x"))
//             }),
//             "x",
//         )
//     }
//
//     /// Invokes the function.
//     pub fn invoke(&self, state: &State, args: Vec<Value>) -> Result<Value, Error> {
//         (self.0)(state, args)
//     }
//
//     /// Creates a value from a boxed Macro.
//     pub fn to_value(&self) -> Value {
//         Value::from_object(self.clone())
//     }
// }
//
// impl fmt::Debug for BoxedMacro {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             if self.1.is_empty() {
//                 "BoxedMacro"
//             } else {
//                 self.1
//             }
//         )
//     }
// }
//
// impl fmt::Display for BoxedMacro {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }
//
// impl Object for BoxedMacro {
//     fn call(&self, state: &State, args: Vec<Value>) -> Result<Value, Error> {
//         let raw_macro = state.env().macros.get(self.1).unwrap();
//         let ast = parse(raw_macro, "x")?;
//
//         let mut compiler = Compiler::new("x", raw_macro);
//         compiler.compile_stmt(&ast).unwrap();
//         let (instructions, blocks, macros) = compiler.finish();
//
//         // should now have the macro.
//         let found_macro = macros.get(self.1).unwrap();
//
//         let mut sub_context = Context::default();
//         sub_context.push_frame(Frame::new(FrameBase::Context(&state.ctx)));
//
//         let mut sub_state = State {
//             env: state.env,
//             ctx: sub_context,
//             auto_escape: state.auto_escape,
//             current_block: Some(self.1),
//             name: self.1,
//             current_block_type: state.current_block_type,
//         };
//
//         for (index, (arg_name, expr)) in found_macro.args.iter().enumerate() {
//             // if this arg exists in the passed args, use it.
//             let value = match args.get(index) {
//                 None => {
//                     let bar = state.env.compile_expression_tem(expr)?;
//                     bar.eval(&())?
//                 }
//                 Some(x) => {
//                     x.clone()
//                 }
//             };
//             sub_state.ctx.store(&arg_name, value);
//         }
//
//         let mut output = String::new();
//
//         let vm = Vm::new(state.env);
//         vm.eval_state(
//             &mut sub_state,
//             &instructions,
//             BTreeMap::new(),
//             macros.clone(),
//             &mut output,
//         )?;
//         println!("output is {}", output);
//
//         Ok(Value::from(output))
//     }
// }
