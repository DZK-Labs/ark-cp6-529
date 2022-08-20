use crate::{Fq, Fq3, Fq3Config};
use ark_ff::{
    fields::fp6_2over3::{Fp6, Fp6Config},
    Field, MontFp,
};

pub type Fq6 = Fp6<Fq6Config>;

pub struct Fq6Config;

impl Fp6Config for Fq6Config {
    type Fp3Config = Fq3Config;

    /// NONRESIDUE = (0, 1, 0)
    #[rustfmt::skip]
    const NONRESIDUE: Fq3 = Fq3::new(Fq::ZERO, Fq::ONE, Fq::ZERO);

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C1: &'static [Fq] = &[
        MontFp!("1"),
        MontFp!("762455925000469635486958154862017323551451463354236306846386042139193650814920395342797265172002456953439664163417039361154927345608957538552061332385880578563"),
        MontFp!("762455925000469635486958154862017323551451463354236306846386042139193650814920395342797265172002456953439664163417039361154927345608957538552061332385880578562"),
        MontFp!("-1"),
        MontFp!("630671112143114611712935202121094239350294111583666017848575488629665946794047857881658061862511333644650561255352230185858099779685190181602079727385310409518"),
        MontFp!("630671112143114611712935202121094239350294111583666017848575488629665946794047857881658061862511333644650561255352230185858099779685190181602079727385310409519"),
    ];
}