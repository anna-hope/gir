use env::Env;

mod function;
mod function_body;
mod general;
mod parameter;
mod return_value;
mod widget;
mod widgets;

pub fn generate(env: &Env) {
    widgets::generate(env);
}
