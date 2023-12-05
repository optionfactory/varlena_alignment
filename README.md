The `pgrx::valena::set_varsize_4b` method casts a `varlena` into a `varattrib_4b`, and also includes a comment about the safety of this cast: "A `varlena` can be safely cast to a `varattrib_4b`".
However, this appears not to be the case, since the alignment of a `varlena` as determined by `std::mem::align_of` is 1, while the alignment of a `varattrib_4b` is 4. This results in undefined behavior.
This went unnoticed until recently since, starting with rust 1.70, alignment checks for pointer dereferences are inserted as debug assertions, manifesting the problem at runtime in debug mode.

Careful analysis of the code showed that:
* postgres' `varlena` has alignment requirement 1
* `pg_sys::varlena` correctly has the same alignment
* postgres' `varattrib_4b` has alignment 4, and requires the developer to verify alignment manually (see the definition of `VARSIZE_4B` and `SET_VARSIZE_4B` macros)
* `pg_sys::varattrib_4b` correctly has the same alignment
* `pgrx::set_varsize_4b` (and `pgrx::set_varsize` that delegates to it) incorrectly assume the passed `varlena` pointer to be 4-aligned and does not require the developer to ensure so in their SAFETY comments.

The tests in this repo aim to document the bug.