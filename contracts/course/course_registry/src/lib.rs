pub mod functions;
pub mod schema;
#[cfg(test)]
mod test;

pub use functions::remove_module::course_registry_remove_module;
pub use schema::CourseModule;
