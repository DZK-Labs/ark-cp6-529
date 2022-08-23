use ark_ec::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    short_weierstrass::{Affine, Projective},
};
use ark_ff::MontFp;

use crate::yafa_108::{Fq, Fr};

pub type G1Affine = Affine<Parameters>;
pub type G1Projective = Projective<Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl CurveConfig for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 600746412144547199455308148290259497480968394648582220356252480504778992479853140369795039873088
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0xe2bb1304f81fc840,
        0xdc7abc2bf0661157,
        0x17cdb96cb71d1e72,
        0x7f162be473a7d636,
        0x4800007536090574
    ];

    /// COFACTOR^(-1) mod r =
    /// 5972489437127511544806700411551093376860164837067817990754427465568390052649
    const COFACTOR_INV: Fr =
        MontFp!("5972489437127511544806700411551093376860164837067817990754427465568390052649");
}

impl SWCurveConfig for Parameters {
    /// COEFF_A = 1
    const COEFF_A: Fq = MontFp!("1");

    /// COEFF_B = 24825092645722084837053491286769396788746345430235617199502849526235422451408810304579294435438397372068681612124384564839990996789541081487217887150398262407540693500729711
    const COEFF_B: Fq = MontFp!("24825092645722084837053491286769396788746345430235617199502849526235422451408810304579294435438397372068681612124384564839990996789541081487217887150398262407540693500729711");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: G1Affine = G1Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);
}

/// G1_GENERATOR_X =
/// 33818160309418874668058552251830868485773349715471101193210471518827928948891517446804127304764388886806457581943404280820058646077888815702130541402825393360576989158150890
///
/// This is point (1, 89954879874184131962173155003143000224312674242089764422235984193539665944977752714860228929590799153556406517028021747258317067654256902010478133781614354963234816904945)
/// removing the cofactor by multiplying 600746412144547199455308148290259497480968394648582220356252480504778992479853140369795039873088
pub const G1_GENERATOR_X: Fq = MontFp!("33818160309418874668058552251830868485773349715471101193210471518827928948891517446804127304764388886806457581943404280820058646077888815702130541402825393360576989158150890");

/// G1_GENERATOR_Y =
/// 29848273268461784690330568480603374118611185507256267348450692492780836089310403360598168867867521739827995427185432667063725086023211304945695046674876637753935829831736123
pub const G1_GENERATOR_Y: Fq = MontFp!("29848273268461784690330568480603374118611185507256267348450692492780836089310403360598168867867521739827995427185432667063725086023211304945695046674876637753935829831736123");