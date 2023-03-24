mod models;

use models::{Category, CompetitorInfo, Gendre, Movements, Score, Units};

fn calculate_ipfgl(body_weight: f64, lifted_weight: f64, is_female: bool, movements: Movements, category: Category) -> f64 {
  let men_equipped_full_meet_coeff: [f64; 3] = [1236.25115, 1449.21864, 0.01644];
  let men_raw_full_meet_coeff: [f64; 3] = [1199.72839, 1025.18162, 0.00921];
  let men_equipped_bench_coeff: [f64; 3] = [381.22073, 733.79378, 0.02398];
  let men_raw_bench_coeff: [f64; 3] = [320.98041, 281.40258, 0.01008];
  let women_equipped_full_meet_coeff: [f64; 3] = [758.63878, 949.31382, 0.02435];
  let women_raw_full_meet_coeff: [f64; 3] = [610.32796, 1045.59282, 0.03048];
  let women_equipped_bench_coeff: [f64; 3] = [221.82209, 357.00377, 0.02937];
  let women_raw_bench_coeff: [f64; 3] = [142.40398, 442.52671, 0.04724];


  10.10
}

fn calculate_ipf() -> f64 {
  10.10
}

fn calculate_old_wilks() -> f64 {
  10.10
}

fn calculate_new_wilks() -> f64 {
  10.10
}

fn calculate_dots() -> f64 {
  10.10
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
          body_weight_corrected,
          lifted_weight_corrected,
          is_competitor_female,
          competitor_info.movements,
          competitor_info.category
      ),
      ipf: calculate_ipf(),
      old_wilks: calculate_old_wilks(),
      new_wilks: calculate_new_wilks(),
      dots: calculate_dots(),
  }
}