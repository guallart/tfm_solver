generate_config() {
    echo "#[allow(unused)]"                                                                   > $4
    echo "use f256::f256;"                                                                    >> $4
    echo ""                                                                                   >> $4
    echo "//***************************************************************************//"    >> $4
    echo ""                                                                                   >> $4
    echo "pub type Precision = $2;"                                                           >> $4
    echo "pub const L: usize = $1;"                                                           >> $4
    echo "pub const N_THREADS: usize = 16;"                                                   >> $4
    echo "pub const N_TRIES: usize = $3;"                                                     >> $4
    echo ""                                                                                   >> $4
    echo "//***************************************************************************//"    >> $4
    echo ""                                                                                   >> $4
    echo "pub const N_RES: usize = (L - 1) * (L - 2) + L * (L - 2) + 2 * L;"                  >> $4
    echo "pub const N_UNK: usize = L * (L - 2);"                                              >> $4
}

compile_cases() {
    generate_config $1 "f64" $2 "./solver/src/config.rs"
    cd solver
    cargo build --release
    cd ..
    mv "solver/target/release/solver" "weibull_case/solver_L$1_lprec"

    generate_config $1 "f256" $2 "./solver/src/config.rs"
    cd solver
    cargo build --release
    cd ..
    mv "solver/target/release/solver" "weibull_case/solver_L$1_hprec"
}

compile_cases 10 1000
compile_cases 15 1000
compile_cases 25 1000
compile_cases 35 1000
compile_cases 50 500
