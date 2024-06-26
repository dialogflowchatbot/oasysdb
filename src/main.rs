use oasysdb::prelude::*;
fn main() -> Result<(), Error> {
    let d: std::vec::Vec<f32> = vec![
        -0.1830099,
        0.02942138,
        -0.03358929,
        0.22310138,
        -0.25760284,
        -0.082017876,
        0.7715998,
        0.019999444,
        0.27886766,
        0.23257127,
        0.22548328,
        -0.4144171,
        0.18545091,
        -0.08068526,
        0.21082862,
        0.04309499,
        0.40061507,
        -0.14869763,
        -0.47228408,
        -0.09803237,
        -0.62571687,
        0.48611817,
        0.23338655,
        -0.085471824,
        -0.14853927,
        0.28431016,
        0.14783347,
        -0.2006708,
        -0.38237795,
        -0.28495333,
        -0.35785016,
        0.031215405,
        0.23551492,
        0.09263659,
        -0.22409339,
        0.038884874,
        0.59170634,
        -0.14094098,
        -0.14969786,
        -0.08921243,
        0.011207024,
        0.26623997,
        0.36350393,
        0.23365681,
        -0.34233722,
        -0.014386821,
        0.20038544,
        -0.32821336,
        0.59632033,
        0.43396688,
        0.4021302,
        -0.0049807923,
        -0.22024737,
        -0.3318699,
        0.16507252,
        0.19523601,
        0.12788026,
        0.08568954,
        0.32117,
        -0.16688085,
        0.56661385,
        -0.5016454,
        -0.027904077,
        -0.13771854,
        0.28059053,
        -0.35248742,
        -0.31399903,
        -0.051390197,
        -0.19492958,
        -0.061612118,
        0.115382425,
        0.4658382,
        0.49496752,
        0.4434831,
        -0.06777578,
        0.22016814,
        0.39553764,
        -0.30675852,
        -0.33441877,
        0.27055174,
        -0.36135352,
        -0.20832248,
        -0.29752734,
        0.083454125,
        -0.21870697,
        -0.26446828,
        -0.21084201,
        0.02511774,
        -0.54655296,
        0.26561525,
        0.019533003,
        0.01934592,
        -0.038636904,
        0.241846,
        0.0051330435,
        0.077782035,
        0.28155136,
        -0.23462692,
        -0.33119568,
        0.8184128,
        0.16215521,
        0.19158235,
        -0.096541196,
        0.15729485,
        0.24193358,
        0.056056477,
        -0.8541212,
        -0.2585449,
        -0.0016051754,
        0.13957219,
        0.11313927,
        -0.21190093,
        -0.39179897,
        -0.0035618008,
        0.21148577,
        0.09679767,
        0.26241085,
        0.1391536,
        0.19620849,
        -0.30062142,
        0.057636738,
        0.52698946,
        -0.18735768,
        -0.061530318,
        -0.36131808,
        -0.12641357,
        0.2952623,
        -3.2852087e-32,
        -0.6891751,
        -0.10845464,
        0.11440267,
        -0.15141094,
        0.036053315,
        0.07214775,
        0.3105031,
        0.38653776,
        0.2798829,
        -0.14185435,
        -0.26538542,
        0.32252237,
        -0.50373507,
        0.11532271,
        0.55598617,
        -0.3929349,
        0.10832211,
        0.12790386,
        -0.56394666,
        -0.25181565,
        -0.31477812,
        -0.20440118,
        -0.19899382,
        0.3876612,
        -0.33283678,
        -0.74229914,
        0.50664234,
        -0.58183426,
        0.5048633,
        0.13730058,
        0.19310924,
        0.19407873,
        0.055544116,
        -0.41067272,
        -0.16000669,
        -0.13872647,
        0.3653455,
        0.29441088,
        0.08864061,
        0.3252824,
        -0.22979684,
        0.22042744,
        0.31749445,
        0.389439,
        0.69752073,
        0.12487006,
        0.21065593,
        0.31782675,
        -0.18017481,
        -0.067218326,
        -0.38901508,
        -0.34089288,
        0.35908258,
        0.5007467,
        -0.09035201,
        0.26363927,
        0.1658033,
        0.20540349,
        -0.51685137,
        -0.03246609,
        -0.48926812,
        0.43310896,
        0.15118192,
        -0.053145565,
        -0.5291988,
        0.8238501,
        -0.17985146,
        -0.056680858,
        -0.09948469,
        0.1557619,
        -0.44246194,
        0.18307951,
        -0.015372346,
        0.11987033,
        -0.6511833,
        -0.5208415,
        0.18051064,
        -0.13481341,
        -0.29864767,
        -0.19592279,
        0.012538335,
        -0.383213,
        -0.06904142,
        -0.11253575,
        0.25654918,
        0.5002516,
        -0.4025379,
        -0.34431896,
        0.36989245,
        -0.18801235,
        -0.082747534,
        0.050047863,
        0.44252536,
        -0.033619978,
        -0.7315233,
        2.9614544e-32,
        -0.1246826,
        -0.57832384,
        -0.063737884,
        0.122749545,
        -0.22925499,
        -0.3930482,
        -0.5141357,
        0.09760066,
        -0.33638355,
        -0.03118032,
        -0.09038236,
        -0.102473795,
        -0.21169019,
        -0.0002704084,
        0.10746553,
        0.12958649,
        0.10844101,
        0.4974557,
        -0.14679475,
        0.116022944,
        -0.091203414,
        0.09600222,
        -0.092632115,
        0.016700048,
        0.08168789,
        -0.22767694,
        -0.25409743,
        0.109938696,
        -0.14431901,
        0.42155704,
        0.2953095,
        -0.84722966,
        -0.3187078,
        -0.38936433,
        0.3962243,
        0.097058825,
        -0.5528917,
        -0.39466226,
        0.05712099,
        0.27721584,
        0.071323626,
        -0.05553736,
        0.19296275,
        0.62929213,
        0.24142575,
        0.5332514,
        -0.084936604,
        0.54710835,
        0.45921627,
        0.2790274,
        -0.1426755,
        0.09175613,
        -0.45166418,
        0.100595616,
        0.08671784,
        -0.16125877,
        -0.06133469,
        0.19618915,
        -0.22371767,
        -0.20083351,
        -0.48061475,
        0.4490703,
        0.044323463,
        0.110441625,
        0.19184317,
        -0.19398117,
        -0.21174577,
        -0.600405,
        -0.5259693,
        0.114364244,
        0.071500696,
        -0.3006565,
        -0.5270621,
        -0.1386822,
        -0.21220484,
        -0.23653701,
        -0.010678959,
        0.35969263,
        -0.65130943,
        0.122501306,
        0.0440534,
        0.6643638,
        -0.20192821,
        0.5381805,
        0.42857543,
        0.31889504,
        0.101279534,
        0.045418024,
        -0.31977946,
        0.35494545,
        0.23063903,
        0.1345756,
        0.06691901,
        -0.23718505,
        -0.18616207,
        -9.0948944e-8,
        -0.29252142,
        -0.09154548,
        0.6571724,
        -0.07041788,
        0.11094185,
        0.5397305,
        -0.8018832,
        0.2685106,
        0.070250385,
        -0.3270691,
        0.4957823,
        0.52010757,
        -0.73011345,
        0.44704342,
        0.46552277,
        -0.079507194,
        0.31771904,
        -0.1604198,
        0.08061316,
        0.3916689,
        -0.658667,
        0.24024847,
        -0.0062879114,
        0.3593991,
        -0.12530504,
        0.17882755,
        -0.027317679,
        -0.10123761,
        0.4140273,
        0.6390934,
        0.21372886,
        0.34134296,
        -0.4672962,
        0.3763638,
        -0.16035782,
        -0.3887697,
        -0.11936587,
        0.005652839,
        -0.07912116,
        -0.4816745,
        -0.2998837,
        -0.031177891,
        -0.04504003,
        -0.47831175,
        -0.6018633,
        -0.13740374,
        0.34118015,
        -0.07591482,
        -0.13885127,
        0.5593857,
        0.26959357,
        -0.017982675,
        0.5729473,
        0.6038925,
        0.3154397,
        -0.079095796,
        0.20187245,
        -0.42710972,
        0.06848629,
        0.4666702,
        0.72529286,
        0.3513997,
        0.6851375,
        0.07819547,
    ];
    let vector: Vector = d.into();
    let record: Record = Record::new(&vector, &"".into());
    let mut config = Config::default();
    config.distance = Distance::Cosine;
    let mut collection = Collection::new(&config);
    let r = collection.insert(&record)?;
    println!("id={}", r.to_usize());
    Ok(())
}
