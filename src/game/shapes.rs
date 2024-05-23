
pub const L_SHAPE: [[[bool; 4]; 4]; 4] = [
    [
        [false, false, true , false],
        [true , true , true , false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, true , false, false],
        [false, true , false, false],
        [false, true , true , false],
        [false, false, false, false],
    ],
    [
        [false, false, false, false],
        [true , true , true , false],
        [true , false, false, false],
        [false, false, false, false],
    ],
    [
        [true , true , false, false],
        [false, true , false, false],
        [false, true , false, false],
        [false, false, false, false],
    ],
];

pub const BACK_L_SHAPE: [[[bool; 4]; 4]; 4] = [
    [
        [true , false, false, false],
        [true , true , true , false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, true , true , false],
        [false, true , false, false],
        [false, true , false, false],
        [false, false, false, false],
    ],
    [
        [false, false, false, false],
        [true , true , true , false],
        [false, false, true , false],
        [false, false, false, false],
    ],
    [
        [false, true , false, false],
        [false, true , false, false],
        [true , true , false, false],
        [false, false, false, false],
    ],
];

pub const LINE: [[[bool; 4]; 4]; 4] = [
    [
        [false, false, false, false],
        [true , true , true , true ],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, false, true , false],
        [false, false, true , false],
        [false, false, true , false],
        [false, false, true , false],
    ],
    [
        [false, false, false, false],
        [false, false, false, false],
        [true , true , true , true ],
        [false, false, false, false],
    ],
    [
        [false, true , false, false],
        [false, true , false, false],
        [false, true , false, false],
        [false, true , false, false],
    ],
];

pub const SQUARE: [[[bool; 4]; 4]; 4] = [
    [
        [false, true , true , false],
        [false, true , true , false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, true , true , false],
        [false, true , true , false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, true , true , false],
        [false, true , true , false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, true , true , false],
        [false, true , true , false],
        [false, false, false, false],
        [false, false, false, false],
    ],
];

pub const Z_SHAPE: [[[bool; 4]; 4]; 4] = [
    [
        [true,  true , false, false],
        [false, true , true , false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, false , true, false],
        [false, true , true , false],
        [false, true , false, false],
        [false, false, false, false],
    ],
    [
        [false, false, false, false],
        [true,  true , false, false],
        [false, true , true , false],
        [false, false, false, false],
    ],
    [
        [false , true, false, false],
        [true , true , false, false],
        [true , false, false, false],
        [false, false, false, false],
    ],
];

pub const BACK_Z_SHAPE: [[[bool; 4]; 4]; 4] = [
    [
        [false, true , true , false],
        [true , true , false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, true , false, false],
        [false, true , true , false],
        [false, false, true , false],
        [false, false, false, false],
    ],
    [
        [false, false, false, false],
        [false, true , true , false],
        [true , true , false, false],
        [false, false, false, false],
    ],
    [
        [true , false, false, false],
        [true , true , false, false],
        [false, true , false, false],
        [false, false, false, false],
    ],
];
