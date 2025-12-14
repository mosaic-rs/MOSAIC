""""
Anchor Pairs:
    X BASED:
    [
        ("x_50", "x_52"),
        ("x_49", "x_53")
    ]

    Y BASED:
    [
        ("y_50", "y_58"),
        ("y_51", "y_57")
    ]

FOR CONSISTENCY: ORDER GOES AS FOLLOWING:
    TOP --> BOTTOM
    LEFT --> RIGHT
    FROM *CAMERA* PERSPECTIVE
"""

X_OUTER_BASED_ANCHOR_PAIRS = [
    ("x_50", "x_52"),
    ("x_49", "x_53"),
    ("x_48", "x_54"),
    ("x_59", "x_55"),
    ("x_58", "x_56")
]

X_INNER_BASED_ANCHOR_PAIRS = [
    ("x_61", "x_63"),
    ("x_60", "x_64"),
    ("x_67", "x_65")
]

Y_OUTER_BASED_ANCHOR_PAIRS = [
    ("y_49", "y_59"),
    ("y_50", "y_58"),
    ("y_51", "y_57"),
    ("y_52", "y_56"),
    ("y_53", "y_55")
]

Y_INNER_BASED_ANCHOR_PAIRS = [
    ("y_61", "y_67"),
    ("y_62", "y_66"),
    ("y_63", "y_65")
]

"""
Curvature Coordinate Lists
ORDER: OUTER -> INNER
"""

UPPER_OUTER_RIGHT_COORDS = [
    ("x_48", "y_48"),
    ("x_49", "y_49"),
    ("x_50", "y_50"),
    ("x_51", "y_51")
]

UPPER_OUTER_LEFT_COORDS = [
    ("x_54", "y_54"),
    ("x_53", "y_53"),
    ("x_52", "y_52"),
    ("x_51", "y_51")
]

LOWER_OUTER_RIGHT_COORDS = [
    ("x_48", "y_48"),
    ("x_59", "y_59"),
    ("x_58", "y_58"),
    ("x_57", "y_57")
]

LOWER_OUTER_LEFT_COORDS = [
    ("x_54", "y_54"),
    ("x_55", "y_55"),
    ("x_56", "y_56"),
    ("x_57", "y_57")
]

UPPER_INNER_RIGHT_COORDS = [
    ("x_60", "y_60"),
    ("x_61", "y_61"),
    ("x_62", "y_62")
]

UPPER_INNER_LEFT_COORDS = [
    ("x_64", "y_64"),
    ("x_63", "y_63"),
    ("x_62", "y_62")
]

LOWER_INNER_RIGHT_COORDS = [
    ("x_60", "y_60"),
    ("x_67", "y_67"),
    ("x_66", "y_66")
]

LOWER_INNER_LEFT_COORDS = [
    ("x_64", "y_64"),
    ("x_65", "y_65"),
    ("x_66", "y_66")
]

"""
ALL MOUTH LANDMARKS PAIRS

ORDER: STARTING FROM LEFT IMAGE COMMISSURE (TRUE POSITION: RIGHT) - CLOCKWISE AND THEN REPEATS FOR INNER POINTS
"""

LANDMARK_PAIRS = [
    ("x_48", "y_48"),
    ("x_49", "y_49"),
    ("x_50", "y_50"),
    ("x_51", "y_51"),
    ("x_52", "y_52"),
    ("x_53", "y_53"),
    ("x_54", "y_54"),
    ("x_55", "y_55"),
    ("x_56", "y_56"),
    ("x_57", "y_57"),
    ("x_58", "y_58"),
    ("x_59", "y_59"),
    ("x_60", "y_60"),
    ("x_61", "y_61"),
    ("x_62", "y_62"),
    ("x_63", "y_63"),
    ("x_64", "y_64"),
    ("x_65", "y_65"),
    ("x_66", "y_66"),
    ("x_67", "y_67")
]

LANDMARK_PAIRS_UNC = [
    ("x_48_unc", "y_48_unc"),
    ("x_49_unc", "y_49_unc"),
    ("x_50_unc", "y_50_unc"),
    ("x_51_unc", "y_51_unc"),
    ("x_52_unc", "y_52_unc"),
    ("x_53_unc", "y_53_unc"),
    ("x_54_unc", "y_54_unc"),
    ("x_55_unc", "y_55_unc"),
    ("x_56_unc", "y_56_unc"),
    ("x_57_unc", "y_57_unc"),
    ("x_58_unc", "y_58_unc"),
    ("x_59_unc", "y_59_unc"),
    ("x_60_unc", "y_60_unc"),
    ("x_61_unc", "y_61_unc"),
    ("x_62_unc", "y_62_unc"),
    ("x_63_unc", "y_63_unc"),
    ("x_64_unc", "y_64_unc"),
    ("x_65_unc", "y_65_unc"),
    ("x_66_unc", "y_66_unc"),
    ("x_67_unc", "y_67_unc")
]

"""
ANCHOR PAIRS
"""

ANCHOR_PAIRS = [

]

"""
Bezier Curves
"""
OUTER_BEZ_CURVE_LIST = [['UR_Ax', 'UR_Ay', 'UR_Bx', 'UR_By', 'UR_Cx', 'UR_Cy', 'UR_Dx', 'UR_Dy'], ['UL_Ax', 'UL_Ay', 'UL_Bx', 'UL_By', 'UL_Cx', 'UL_Cy', 'UL_Dx', 'UL_Dy'], ['LR_Ax', 'LR_Ay', 'LR_Bx', 'LR_By', 'LR_Cx', 'LR_Cy', 'LR_Dx', 'LR_Dy'], ['LL_Ax', 'LL_Ay', 'LL_Bx', 'LL_By', 'LL_Cx', 'LL_Cy', 'LL_Dx', 'LL_Dy']]

# OUTER_BEZ_CURVE_LIST = 'UR_Ax', 'UR_Ay' 'UR_Bx' 'UR_By' 'UR_Cx' 'UR_Cy' 'UR_Dx' 'UR_Dy' UR_X0 UR_X1 UR_X2 UR_X3 UR_Y0 UR_Y1 UR_Y2 UR_Y3 UL_Ax UL_Ay UL_Bx UL_By UL_Cx UL_Cy UL_Dx UL_Dy UL_X0 UL_X1 UL_X2 UL_X3 UL_Y0 UL_Y1 UL_Y2 UL_Y3 LR_Ax LR_Ay LR_Bx LR_By LR_Cx LR_Cy LR_Dx LR_Dy LR_X0 LR_X1 LR_X2 LR_X3 LR_Y0 LR_Y1 LR_Y2 LR_Y3 LL_Ax LL_Ay LL_Bx LL_By LL_Cx LL_Cy LL_Dx LL_Dy LL_X0 LL_X1 LL_X2 LL_X3 LL_Y0 LL_Y1 LL_Y2 LL_Y3

INNER_BEZ_CURVE_LIST = [['IUR_Ax', 'IUR_Ay', 'IUR_Bx', 'IUR_By', 'IUR_Cx', 'IUR_Cy'], ['IUL_Ax', 'IUL_Ay', 'IUL_Bx', 'IUL_By', 'IUL_Cx', 'IUL_Cy'], ['ILR_Ax', 'ILR_Ay', 'ILR_Bx', 'ILR_By', 'ILR_Cx', 'ILR_Cy'], ['ILL_Ax', 'ILL_Ay', 'ILL_Bx', 'ILL_By', 'ILL_Cx', 'ILL_Cy']]

# "IUR_Ax IUR_Ay IUR_Bx IUR_By IUR_Cx IUR_Cy IUR_X0 IUR_X1 IUR_X2 IUR_Y0 IUR_Y1 IUR_Y2 IUL_Ax IUL_Ay IUL_Bx IUL_By IUL_Cx IUL_Cy IUL_X0

"""
MOUTH AREA
"""

OUTER_QUAD_AREA = ["QUAD_O_Q1", "QUAD_O_Q2", "QUAD_O_Q3", "QUAD_O_Q4", "QUAD_O_total"]

INNER_QUAD_AREA = ["QUAD_I_Q1", "QUAD_I_Q2", "QUAD_I_Q3", "QUAD_I_Q4", "QUAD_I_total"]


OUTER_BIO_AREA = ["BIO_O_UR", "BIO_O_UL", "BIO_O_LR", "BIO_O_LL", "BIO_O_total"]

INNER_BIO_AREA = ["BIO_I_UR", "BIO_I_UL", "BIO_I_LR", "BIO_I_LL", "BIO_I_total"]