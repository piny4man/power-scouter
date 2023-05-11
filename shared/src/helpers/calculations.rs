use crate::models::{Category, CompetitorInfo, Gendre, Movements, Score, Units};

fn calculate_ipfgl(
    body_weight: &f64,
    lifted_weight: &f64,
    is_female: bool,
    movements: &Movements,
    category: &Category,
) -> f64 {
    let men_equipped_full_meet_coeff: [f64; 3] = [1236.25115, 1449.21864, 0.01644];
    let men_raw_full_meet_coeff: [f64; 3] = [1199.72839, 1025.18162, 0.00921];
    let men_equipped_bench_coeff: [f64; 3] = [381.22073, 733.79378, 0.02398];
    let men_raw_bench_coeff: [f64; 3] = [320.98041, 281.40258, 0.01008];
    let women_equipped_full_meet_coeff: [f64; 3] = [758.63878, 949.31382, 0.02435];
    let women_raw_full_meet_coeff: [f64; 3] = [610.32796, 1045.59282, 0.03048];
    let women_equipped_bench_coeff: [f64; 3] = [221.82209, 357.00377, 0.02937];
    let women_raw_bench_coeff: [f64; 3] = [142.40398, 442.52671, 0.04724];

    let mut coeff: [f64; 3] = [0.0, 0.0, 0.0];

    // TODO Improve if/else logic
    if is_female {
        if let Category::Raw = category {
            if let Movements::FullMeet = movements {
                coeff = women_raw_full_meet_coeff;
            } else {
                coeff = women_raw_bench_coeff
            }
        } else {
            if let Movements::FullMeet = movements {
                coeff = women_equipped_full_meet_coeff;
            } else {
                coeff = women_equipped_bench_coeff
            }
        }
    } else {
        if let Category::Raw = category {
            if let Movements::FullMeet = movements {
                coeff = men_raw_full_meet_coeff;
            } else {
                coeff = men_raw_bench_coeff
            }
        } else {
            if let Movements::FullMeet = movements {
                coeff = men_equipped_full_meet_coeff;
            } else {
                coeff = men_equipped_bench_coeff
            }
        }
    }

    let power = -coeff[2] * body_weight;

    lifted_weight * (100.0 / (coeff[0] - coeff[1] * power.exp()))
}

fn calculate_ipf(
    body_weight: &f64,
    lifted_weight: &f64,
    is_female: bool,
    movements: &Movements,
    category: &Category,
) -> f64 {
    let men_equipped_full_meet_coeff: [f64; 4] = [387.265, 1121.28, 80.6324, 222.4896];
    let men_raw_full_meet_coeff: [f64; 4] = [310.67, 857.785, 53.216, 147.0835];
    let men_equipped_bench_coeff: [f64; 4] = [133.94, 441.465, 35.3938, 113.0057];
    let men_raw_bench_coeff: [f64; 4] = [86.4745, 259.155, 17.5785, 53.122];
    let women_equipped_full_meet_coeff: [f64; 4] = [176.58, 373.315, 48.4534, 110.0103];
    let women_raw_full_meet_coeff: [f64; 4] = [125.1435, 228.03, 34.5246, 86.8301];
    let women_equipped_bench_coeff: [f64; 4] = [49.106, 124.209, 23.199, 67.4926];
    let women_raw_bench_coeff: [f64; 4] = [25.0485, 43.848, 6.7172, 13.952];

    let mut coeff: [f64; 4] = [0.0, 0.0, 0.0, 0.0];

    // TODO Improve if/else logic
    if is_female {
        if let Category::Raw = category {
            if let Movements::FullMeet = movements {
                coeff = women_raw_full_meet_coeff;
            } else {
                coeff = women_raw_bench_coeff
            }
        } else {
            if let Movements::FullMeet = movements {
                coeff = women_equipped_full_meet_coeff;
            } else {
                coeff = women_equipped_bench_coeff
            }
        }
    } else {
        if let Category::Raw = category {
            if let Movements::FullMeet = movements {
                coeff = men_raw_full_meet_coeff;
            } else {
                coeff = men_raw_bench_coeff
            }
        } else {
            if let Movements::FullMeet = movements {
                coeff = men_equipped_full_meet_coeff;
            } else {
                coeff = men_equipped_bench_coeff
            }
        }
    }

    let body_weight_ln = body_weight.ln();

    500.0
        + 100.0 * (lifted_weight - (coeff[0] * body_weight_ln - coeff[1]))
            / (coeff[2] * body_weight_ln - coeff[3])
}

fn calculate_old_wilks(body_weight: &f64, lifted_weight: &f64, is_female: bool) -> f64 {
    let male_coeff: [f64; 6] = [
        -216.0475144,
        16.2606339,
        -0.002388645,
        -0.00113732,
        7.01863e-6,
        -1.291e-8,
    ];
    let female_coeff: [f64; 6] = [
        594.31747775582,
        -27.23842536447,
        0.82112226871,
        -0.00930733913,
        4.731582e-5,
        -9.054e-8,
    ];
    let mut denominator = if is_female {
        female_coeff[0]
    } else {
        male_coeff[0]
    };
    let coefficient = if is_female { female_coeff } else { male_coeff };
    let min_body_weight = if is_female { 26.51 } else { 40.0 };
    let max_body_weight = if is_female { 154.53 } else { 201.9 };
    let final_body_weight = if body_weight < &min_body_weight {
        min_body_weight
    } else if body_weight > &max_body_weight {
        max_body_weight
    } else {
        *body_weight
    };

    for (i, _coeff) in coefficient.iter().enumerate().skip(1) {
        denominator += coefficient[i] * final_body_weight.powi(i.try_into().unwrap());
    }

    (500.0 / denominator) * lifted_weight
}

fn calculate_new_wilks(body_weight: &f64, lifted_weight: &f64, is_female: bool) -> f64 {
    let male_coeff: [f64; 6] = [
        47.4617885411949,
        8.47206137941125,
        0.073694103462609,
        -1.39583381094385e-3,
        7.07665973070743e-6,
        -1.20804336482315e-8,
    ];
    let female_coeff: [f64; 6] = [
        -125.425539779509,
        13.7121941940668,
        -0.0330725063103405,
        -1.0504000506583e-3,
        9.38773881462799e-6,
        -2.3334613884954e-8,
    ];
    let mut denominator = if is_female {
        female_coeff[0]
    } else {
        male_coeff[0]
    };
    let coefficient = if is_female { female_coeff } else { male_coeff };
    let min_body_weight = 40.0;
    let max_body_weight = if is_female { 150.95 } else { 200.95 };
    let final_body_weight = if body_weight < &min_body_weight {
        min_body_weight
    } else if body_weight > &max_body_weight {
        max_body_weight
    } else {
        *body_weight
    };

    for (i, _coeff) in coefficient.iter().enumerate().skip(1) {
        denominator += coefficient[i] * final_body_weight.powi(i.try_into().unwrap());
    }

    (600.0 / denominator) * lifted_weight
}

fn calculate_dots(body_weight: &f64, lifted_weight: &f64, is_female: bool) -> f64 {
    let male_coeff: [f64; 5] = [
        -307.75076,
        24.0900756,
        -0.1918759221,
        0.0007391293,
        -0.000001093,
    ];
    let female_coeff: [f64; 5] = [
        -57.96288,
        13.6175032,
        -0.1126655495,
        0.0005158568,
        -0.0000010706,
    ];

    let mut denominator = if is_female {
        female_coeff[0]
    } else {
        male_coeff[0]
    };
    let coefficient = if is_female { female_coeff } else { male_coeff };
    let max_body_weight = if is_female { 150.0 } else { 210.0 };
    let final_body_weight = if body_weight > &max_body_weight {
        max_body_weight
    } else {
        *body_weight
    };

    for (i, _coeff) in coefficient.iter().enumerate().skip(1) {
        denominator += coefficient[i] * final_body_weight.powi(i.try_into().unwrap());
    }

    (500.0 / denominator) * lifted_weight
}

pub fn calculate_score(competitor_info: CompetitorInfo) -> Score {
    let weight_coefficient = 0.45359237;
    let are_units_kg = matches!(competitor_info.units, Units::Kg);
    let is_competitor_female = matches!(competitor_info.gendre, Gendre::Female);
    let body_weight_corrected = if are_units_kg {
        competitor_info.body_weight.parse::<f64>().unwrap()
    } else {
        competitor_info.body_weight.parse::<f64>().unwrap() * weight_coefficient
    };
    let lifted_weight_corrected = if are_units_kg {
        competitor_info.lifted_weight.parse::<f64>().unwrap()
    } else {
        competitor_info.lifted_weight.parse::<f64>().unwrap() * weight_coefficient
    };

    Score {
        body_weight: body_weight_corrected,
        lifted_weight: lifted_weight_corrected,
        unit: competitor_info.units,
        ipfgl: calculate_ipfgl(
            &body_weight_corrected,
            &lifted_weight_corrected,
            is_competitor_female,
            &competitor_info.movements,
            &competitor_info.category,
        ),
        ipf: calculate_ipf(
            &body_weight_corrected,
            &lifted_weight_corrected,
            is_competitor_female,
            &competitor_info.movements,
            &competitor_info.category,
        ),
        old_wilks: calculate_old_wilks(
            &body_weight_corrected,
            &lifted_weight_corrected,
            is_competitor_female,
        ),
        new_wilks: calculate_new_wilks(
            &body_weight_corrected,
            &lifted_weight_corrected,
            is_competitor_female,
        ),
        dots: calculate_dots(
            &body_weight_corrected,
            &lifted_weight_corrected,
            is_competitor_female,
        ),
    }
}
