use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::{PgMemoryContexts, set_varsize};
    use pgrx::prelude::*;
    use pgrx::pg_sys::varlena;

    #[pg_test]
    fn test_varlena_alignment_1b() {
        // Take a bunch of palloced memory
        let ptr = unsafe { PgMemoryContexts::CurrentMemoryContext.palloc0(1024) };
        // Build an new pointer, offset by the alignment of varlena; this is a varlena pointer with valid alignment
        let offset_ptr = unsafe { ptr.offset(std::mem::align_of::<varlena>() as isize) } as *mut varlena;
        // set_varsize_1b succeeds, as it casts the pointer to varattrib_1b, which has the same alignment as varlena (1)
        unsafe { pgrx::set_varsize_1b(offset_ptr, 42); }
    }

    #[pg_test]
    fn test_varlena_alignment_4b() {
        // Same as above
        let ptr = unsafe { PgMemoryContexts::CurrentMemoryContext.palloc0(1024) };
        let offset_ptr = unsafe { ptr.offset(std::mem::align_of::<varlena>() as isize) } as *mut varlena;
        // set_varsize_4b raises an assertion error, as it casts the pointer to varattrib_4b, which has a stricter alignment requirement (4)
        unsafe { pgrx::set_varsize_4b(offset_ptr, 42); }
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
