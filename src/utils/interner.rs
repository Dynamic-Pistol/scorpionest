use lasso::ThreadedRodeo;
use once_cell::sync::Lazy;

pub static INTERNER: Lazy<ThreadedRodeo> = Lazy::new(ThreadedRodeo::new);
