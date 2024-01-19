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
pub const KEV_COEFF_TRAPEZIOD: [[f32; 2]; 7] = [
    [
        0.10422505,
        0.12287712,
    ],
    [
        0.10837995,
        0.061881065,
    ],
    [
        0.1055561,
        0.06568909,
    ],
    [
        0.10789265,
        0.1550169,
    ],
    [
        0.10604781,
        0.14850807,
    ],
    [
        0.10750465,
        0.14304066,
    ],
    [
        0.10402631,
        0.141922,
    ],
];

pub const DETECTOR_BORDERS: [[usize; 2]; 8] = [
        [1, 3],
        [1, 4],
        [1, 7],
        [2, 3],
        [2, 5],
        [2, 7],
        [3, 4],
        [4, 5],
];
