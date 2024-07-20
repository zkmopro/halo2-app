// Here we're calling a macro exported by uniffi. This macro will
// write some functions and bind them to the uniffi UDL file. These
// functions will invoke the `get_circom_wtns_fn` generated below.
mopro_ffi::app!();

// This macro is used to define the circuits that will be used in the app.
// You can pass a list of comma separated tuples with the following format:
// ("circuit_proving_key_name.bin", prove_fn, "circuit_verifying_key_name.bin", verify_fn)
mopro_ffi::set_halo2_circuits! {
    ("fibonacci_pk.bin", halo2_fibonacci::prove, "fibonacci_vk.bin", halo2_fibonacci::verify),
}
