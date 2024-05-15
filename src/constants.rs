/// Temporal storage for constants used in builtin processing
/// TODO: remove this module

pub const KEV_COEFF_MAX: [[f32; 2]; 7] = [
    [0.059379287, 0.31509972],
    [0.060557768, 0.26772976],
    [0.06317734, 0.23027992],
    [0.062333938, 0.26050186],
    [0.062186483, 0.25954437],
    [0.06751788, 0.2222414],
    [0.05806803, 0.14519024],
];

// coeffs for (3,19)
// pub const KEV_COEFF_LIKHOVID: [[f32; 2]; 7] = [
//     [0.134678, 0.09647 ],
//     [0.141536, 0.060275],
//     [0.147718, 0.027412],
//     [0.150288, 0.038774],
//     [0.15131 , 0.071923],
//     [0.15336 , 0.029206],
//     [0.136762, 0.041848]
// ];

pub const KEV_COEFF_LIKHOVID: [[f32; 2]; 7] = [
    [
        0.3175972,
        0.071510315,
    ],
    [
        0.2723175,
        0.08074951,
    ],
    [
        0.2869933,
        0.082289696,
    ],
    [
        0.29424095,
        -0.0075092316,
    ],
    [
        0.29598197,
        0.06416798,
    ],
    [
        0.2869933,
        0.082289696,
    ],
    [
        0.26007754,
        -0.017463684,
    ],
];

// pub const KEV_COEFF_FIRST_PEAK: [[f32; 2]; 7] = [
//     [
//         0.30209273,
//         0.058135986,
//     ],
//     [
//         0.25891086,
//         -0.0007972717,
//     ],
//     [
//         0.2746626,
//         -0.036146164,
//     ],
//     [
//         0.27816013,
//         0.050985336,
//     ],
//     [
//         0.28441244,
//         -0.08033466,
//     ],
//     [
//         0.27044022,
//         0.05974865,
//     ],
//     [
//         0.2477852,
//         -0.06184864,
//     ],
// ];

// Calibration by Tritium_1 (set1-set4) (12-17 kev, step = 0.5 kev)
// pub const KEV_COEFF_FIRST_PEAK: [[f32; 2]; 7] = [
//     [
//         0.299_610_6,
//         0.059_213_154
//     ],
//     [
//         0.255_681_04,
//         0.144_415_51
//     ],
//     [
//         0.272_929_82,
//         0.010_163_521
//     ],
//     [
//         0.279_582_2,
//         -0.006_109_655
//     ],
//     [
//         0.281_541_05,
//         0.055_051_66
//     ],
//     [
//         0.269_443_1,
//         0.055_028_245
//     ],
//     [
//         0.246_677_89,
//         0.039_841_548
//     ]
// ];


// Calibration by 2023_11/Tritium_recalibration (14-17.5 kev, step = 0.5 kev)
pub const KEV_COEFF_FIRST_PEAK: [[f32; 2]; 7] = [
    [
        0.299658,
        -0.000544085
    ],
    [
        0.257471,
        0.00115133
    ],
    [
        0.272017,
        -0.0013688
    ],
    [
        0.272688,
        -0.000754553
    ],
    [
        0.283667,
        0.00380029
    ],
    [
        0.270379,
        0.000300277
    ],
    [
        0.242901,
        0.0005929
    ]
];

// Calibration by Tritium (12-17 kev, step = 0.5 kev)
// pub const KEV_COEFF_FIRST_PEAK: [[f32; 2]; 7] = [
//     [
//         0.298_225_46,
//         0.122_968_49
//     ],
//     [
//         0.255_483_93,
//         0.143_287_4
//     ],
//     [
//         0.271_925_8,
//         0.055_675_175
//     ],
//     [
//         0.278_973_28,
//         0.021_011_2
//     ],
//     [
//         0.280_811_55,
//         0.082_378_61
//     ],
//     [
//         0.269_180_54,
//         0.067_159_49
//     ],
//     [
//         0.246_528_28,
//         0.053_119_823
//     ]
// ];

// Calibration by Tritium (12-16 kev)
// pub const KEV_COEFF_FIRST_PEAK: [[f32; 2]; 7] = [
//     [0.298225, 0.122968],
//     [0.255484, 0.143287],
//     [0.271926, 0.0556752],
//     [0.278973, 0.0210112],
//     [0.280812, 0.0823786],
//     [0.269181, 0.0671595],
//     [0.246528, 0.0531198]
// ];

// calibration by Electrode_2
// pub const KEV_COEFF_FIRST_PEAK: [[f32; 2]; 7] = [
//     [0.30209273, -0.022],
//     [0.25891086, -0.0007972717],
//     [0.2746626, -0.036146164],
//     [0.27816013, 0.081],
//     [0.28441244, -0.0133],
//     [0.27044022, -0.01026],
//     [0.2477852, -0.0318],
// ];

// Calibration by Tritium_1 (set1-set4) (12-17 kev, step = 0.5 kev)
// pub const KEV_COEFF_TRAPEZIOD: [[f32; 2]; 7] = [
//     [
//         0.10422505,
//         0.12287712,
//     ],
//     [
//         0.10837995,
//         0.061881065,
//     ],
//     [
//         0.1055561,
//         0.06568909,
//     ],
//     [
//         0.10789265,
//         0.1550169,
//     ],
//     [
//         0.10604781,
//         0.14850807,
//     ],
//     [
//         0.10750465,
//         0.14304066,
//     ],
//     [
//         0.10402631,
//         0.141922,
//     ],
// ];

pub const KEV_COEFF_TRAPEZIOD: [[f32; 2]; 7] = [
    [
        1.0,
        0.0,
    ],
    [
        0.12669839,
        0.6464157,
    ],
    [
        0.12724328,
        0.5693121,
    ],
    [
        1.0,
        0.0,
    ],
    [
        0.12736997,
        0.6286917,
    ],
    [
        0.12902132,
        0.63895607,
    ],
    [
        0.12902132,
        0.63895607,
    ],
];

// TODO: calibrate
pub const KEV_COEFF_LONGDIFF: [[f32; 2]; 7] = [
    [
        1.0,
        0.0,
    ],
    [
        1.0,
        0.0,
    ],
    [
        1.0,
        0.0,
    ],
    [
        1.0,
        0.0,
    ],
    [
        1.0,
        0.0,
    ],
    [
        1.0,
        0.0,
    ],
    [
        1.0,
        0.0,
    ],
];

pub const DETECTOR_BORDERS: [[u8; 2]; 6] = [
    [0, 1],
    [0, 4],
    [1, 3],
    [2, 4],
    [2, 6],
    [3, 6],
];



// calcluate baseline for 2024_03 data set
// from Electrode_3 to Tritium_12
// constant extrapolation
pub fn baseline_2024_03(time: u64, ch_id: u8) -> f32 {

    const TIMES: [u64; 18] = [
        1711317108386830736,
        1711360670893782872,
        1711545489473656016,
        1711566484845656048,
        1711586624318707280,
        1711670437758368480,
        1711741174694678576,
        1711827604635200456,
        1711827604635200456,
        1711911668527591712,
        1712010402813050312,
        1712081950822779920,
        1712195913038798528,
        1712273043300465992,
        1712384266593392328,
        1712466632900589048,
        1712549828134914720,
        1712623401464114520
    ];

    const CH2: [f32; 18] = [
        30.25,
        27.25,
        23.25,
        23.25,
        22.25,
        21.25,
        20.75,
        20.75,
        20.75,
        20.75,
        20.75,
        21.25,
        18.75,
        17.25,
        16.25,
        15.75,
        15.25,
        15.75
    ];

    const CH3: [f32; 18] = [
        26.75,
        25.25,
        21.25,
        20.75,
        20.25,
        19.25,
        18.75,
        18.75,
        18.75,
        18.75,
        18.75,
        19.25,
        17.25,
        15.75,
        14.75,
        14.25,
        13.75,
        14.25,
    ];

    const CH5: [f32; 18] = [
        26.25,
        25.25,
        21.25,
        21.25,
        20.25,
        19.25,
        19.25,
        18.75,
        18.75,
        18.75,
        18.75,
        19.25,
        17.25,
        15.75,
        14.75,
        14.25,
        13.75,
        14.25
    ];

    const CH6: [f32; 18] = [
        28.75,
        26.25,
        23.25,
        22.75,
        21.75,
        20.75,
        20.25,
        20.25,
        20.25,
        20.25,
        20.25,
        20.75,
        18.25,
        16.75,
        15.75,
        15.25,
        14.75,
        15.25
    ];

    const CH7: [f32; 18] = [
        25.75,
        23.75,
        20.25,
        20.25,
        19.25,
        18.25,
        18.25,
        17.75,
        17.75,
        18.25,
        17.75,
        18.25,
        16.25,
        14.75,
        13.75,
        13.25,
        13.25,
        13.75
    ];

    
    let mut sector = 0;
    if time < TIMES[0]  {
        return match ch_id {
            0 => 0.0,
            1 => CH2[0],
            2 => CH3[0],
            3 => 0.0,
            4 => CH5[0],
            5 => CH6[0],
            6 => CH7[0],
            _ => 0.0
        }
    } else if time > TIMES[17] {
        return match ch_id {
            0 => 0.0,
            1 => CH2[17],
            2 => CH3[17],
            3 => 0.0,
            4 => CH5[17],
            5 => CH6[17],
            6 => CH7[17],
            _ => 0.0
        }
    } else {
        for i in 0..17 {
            if time >= TIMES[i] && time < TIMES[i + 1] {
                sector = i;
                break;
            }
        }
    }

    let x1 = TIMES[sector]; let x2 = TIMES[sector + 1];
    let y1 = match ch_id {
        0 => 0.0,
        1 => CH2[sector],
        2 => CH3[sector],
        3 => 0.0,
        4 => CH5[sector],
        5 => CH6[sector],
        6 => CH7[sector],
        _ => 0.0
    };
    let y2 = match ch_id {
        0 => 0.0,
        1 => CH2[sector + 1],
        2 => CH3[sector + 1],
        3 => 0.0,
        4 => CH5[sector + 1],
        5 => CH6[sector + 1],
        6 => CH7[sector + 1],
        _ => 0.0
    };

    y1 + (y2 - y1) * (time - x1) as f32 / (x2 - x1) as f32

}