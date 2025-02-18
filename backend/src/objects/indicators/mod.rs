mod combination;
mod implementations;
mod types;

#[cfg(test)]
mod tests;

// pub use implementations::*;
pub use combination::*;
use sqlx::postgres::PgRow;
pub use types::*;

use crate::metaheuristic::{Variable, VariableDefinition};
use crate::objects::{criteria::Criterion, klines::KlineCollection};

impl IndicatorTrait for Indicator {
    fn information(&self) -> IndicatorInformation {
        match self {
            Indicator::MovingAverage(indicator) => indicator.information(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.information(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.information(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.information(),
            Indicator::BollingerBands(indicator) => indicator.information(),
            Indicator::FibonacciRetracement(indicator) => indicator.information(),
            Indicator::StochasticOscillator(indicator) => indicator.information(),
            Indicator::OnBalanceVolume(indicator) => indicator.information(),
            Indicator::IchimokuCloud(indicator) => indicator.information(),
        }
    }

    fn get_params_variable_definitions(&self) -> Vec<VariableDefinition> {
        match self {
            Indicator::MovingAverage(indicator) => indicator.get_params_variable_definitions(),
            Indicator::ExponentialMovingAverage(indicator) => {
                indicator.get_params_variable_definitions()
            }
            Indicator::RelativeStrengthIndex(indicator) => {
                indicator.get_params_variable_definitions()
            }
            Indicator::MovingAverageConvergenceDivergence(indicator) => {
                indicator.get_params_variable_definitions()
            }
            Indicator::BollingerBands(indicator) => indicator.get_params_variable_definitions(),
            Indicator::FibonacciRetracement(indicator) => {
                indicator.get_params_variable_definitions()
            }
            Indicator::StochasticOscillator(indicator) => {
                indicator.get_params_variable_definitions()
            }
            Indicator::OnBalanceVolume(indicator) => indicator.get_params_variable_definitions(),
            Indicator::IchimokuCloud(indicator) => indicator.get_params_variable_definitions(),
        }
    }

    fn get_all_variable_definitions(&self) -> Vec<VariableDefinition> {
        match self {
            Indicator::MovingAverage(indicator) => indicator.get_all_variable_definitions(),
            Indicator::ExponentialMovingAverage(indicator) => {
                indicator.get_all_variable_definitions()
            }
            Indicator::RelativeStrengthIndex(indicator) => indicator.get_all_variable_definitions(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => {
                indicator.get_all_variable_definitions()
            }
            Indicator::BollingerBands(indicator) => indicator.get_all_variable_definitions(),
            Indicator::FibonacciRetracement(indicator) => indicator.get_all_variable_definitions(),
            Indicator::StochasticOscillator(indicator) => indicator.get_all_variable_definitions(),
            Indicator::OnBalanceVolume(indicator) => indicator.get_all_variable_definitions(),
            Indicator::IchimokuCloud(indicator) => indicator.get_all_variable_definitions(),
        }
    }

    fn column_names(&self) -> Vec<String> {
        match self {
            Indicator::MovingAverage(indicator) => indicator.column_names(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.column_names(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.column_names(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.column_names(),
            Indicator::BollingerBands(indicator) => indicator.column_names(),
            Indicator::FibonacciRetracement(indicator) => indicator.column_names(),
            Indicator::StochasticOscillator(indicator) => indicator.column_names(),
            Indicator::OnBalanceVolume(indicator) => indicator.column_names(),
            Indicator::IchimokuCloud(indicator) => indicator.column_names(),
        }
    }

    fn n_before_needed(&self) -> i32 {
        match self {
            Indicator::MovingAverage(indicator) => indicator.n_before_needed(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.n_before_needed(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.n_before_needed(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.n_before_needed(),
            Indicator::BollingerBands(indicator) => indicator.n_before_needed(),
            Indicator::FibonacciRetracement(indicator) => indicator.n_before_needed(),
            Indicator::StochasticOscillator(indicator) => indicator.n_before_needed(),
            Indicator::OnBalanceVolume(indicator) => indicator.n_before_needed(),
            Indicator::IchimokuCloud(indicator) => indicator.n_before_needed(),
        }
    }

    fn reserve_space(&mut self, n: i32) {
        match self {
            Indicator::MovingAverage(indicator) => indicator.reserve_space(n),
            Indicator::ExponentialMovingAverage(indicator) => indicator.reserve_space(n),
            Indicator::RelativeStrengthIndex(indicator) => indicator.reserve_space(n),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.reserve_space(n),
            Indicator::BollingerBands(indicator) => indicator.reserve_space(n),
            Indicator::FibonacciRetracement(indicator) => indicator.reserve_space(n),
            Indicator::StochasticOscillator(indicator) => indicator.reserve_space(n),
            Indicator::OnBalanceVolume(indicator) => indicator.reserve_space(n),
            Indicator::IchimokuCloud(indicator) => indicator.reserve_space(n),
        }
    }

    fn store_row(&mut self, row: &PgRow) {
        match self {
            Indicator::MovingAverage(indicator) => indicator.store_row(row),
            Indicator::ExponentialMovingAverage(indicator) => indicator.store_row(row),
            Indicator::RelativeStrengthIndex(indicator) => indicator.store_row(row),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.store_row(row),
            Indicator::BollingerBands(indicator) => indicator.store_row(row),
            Indicator::FibonacciRetracement(indicator) => indicator.store_row(row),
            Indicator::StochasticOscillator(indicator) => indicator.store_row(row),
            Indicator::OnBalanceVolume(indicator) => indicator.store_row(row),
            Indicator::IchimokuCloud(indicator) => indicator.store_row(row),
        }
    }

    fn get_missing_indices(&self) -> Vec<i32> {
        match self {
            Indicator::MovingAverage(indicator) => indicator.get_missing_indices(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.get_missing_indices(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.get_missing_indices(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => {
                indicator.get_missing_indices()
            }
            Indicator::BollingerBands(indicator) => indicator.get_missing_indices(),
            Indicator::FibonacciRetracement(indicator) => indicator.get_missing_indices(),
            Indicator::StochasticOscillator(indicator) => indicator.get_missing_indices(),
            Indicator::OnBalanceVolume(indicator) => indicator.get_missing_indices(),
            Indicator::IchimokuCloud(indicator) => indicator.get_missing_indices(),
        }
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        match self {
            Indicator::MovingAverage(indicator) => indicator.calculate(kline_collection),
            Indicator::ExponentialMovingAverage(indicator) => indicator.calculate(kline_collection),
            Indicator::RelativeStrengthIndex(indicator) => indicator.calculate(kline_collection),
            Indicator::MovingAverageConvergenceDivergence(indicator) => {
                indicator.calculate(kline_collection)
            }
            Indicator::BollingerBands(indicator) => indicator.calculate(kline_collection),
            Indicator::FibonacciRetracement(indicator) => indicator.calculate(kline_collection),
            Indicator::StochasticOscillator(indicator) => indicator.calculate(kline_collection),
            Indicator::OnBalanceVolume(indicator) => indicator.calculate(kline_collection),
            Indicator::IchimokuCloud(indicator) => indicator.calculate(kline_collection),
        }
    }

    fn get_values(&self) -> Vec<&Vec<Option<f64>>> {
        match self {
            Indicator::MovingAverage(indicator) => indicator.get_values(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.get_values(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.get_values(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.get_values(),
            Indicator::BollingerBands(indicator) => indicator.get_values(),
            Indicator::FibonacciRetracement(indicator) => indicator.get_values(),
            Indicator::StochasticOscillator(indicator) => indicator.get_values(),
            Indicator::OnBalanceVolume(indicator) => indicator.get_values(),
            Indicator::IchimokuCloud(indicator) => indicator.get_values(),
        }
    }

    fn calculate_criteria(&mut self, kline_collection: &KlineCollection) {
        match self {
            Indicator::MovingAverage(indicator) => indicator.calculate_criteria(kline_collection),
            Indicator::ExponentialMovingAverage(indicator) => {
                indicator.calculate_criteria(kline_collection)
            }
            Indicator::RelativeStrengthIndex(indicator) => {
                indicator.calculate_criteria(kline_collection)
            }
            Indicator::MovingAverageConvergenceDivergence(indicator) => {
                indicator.calculate_criteria(kline_collection)
            }
            Indicator::BollingerBands(indicator) => indicator.calculate_criteria(kline_collection),
            Indicator::FibonacciRetracement(indicator) => {
                indicator.calculate_criteria(kline_collection)
            }
            Indicator::StochasticOscillator(indicator) => {
                indicator.calculate_criteria(kline_collection)
            }
            Indicator::OnBalanceVolume(indicator) => indicator.calculate_criteria(kline_collection),
            Indicator::IchimokuCloud(indicator) => indicator.calculate_criteria(kline_collection),
        }
    }

    fn get_criteria(&self) -> &Vec<Criterion> {
        match self {
            Indicator::MovingAverage(indicator) => indicator.get_criteria(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.get_criteria(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.get_criteria(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.get_criteria(),
            Indicator::BollingerBands(indicator) => indicator.get_criteria(),
            Indicator::FibonacciRetracement(indicator) => indicator.get_criteria(),
            Indicator::StochasticOscillator(indicator) => indicator.get_criteria(),
            Indicator::OnBalanceVolume(indicator) => indicator.get_criteria(),
            Indicator::IchimokuCloud(indicator) => indicator.get_criteria(),
        }
    }

    fn get_criteria_count(&self) -> i32 {
        match self {
            Indicator::MovingAverage(indicator) => indicator.get_criteria_count(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.get_criteria_count(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.get_criteria_count(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => {
                indicator.get_criteria_count()
            }
            Indicator::BollingerBands(indicator) => indicator.get_criteria_count(),
            Indicator::FibonacciRetracement(indicator) => indicator.get_criteria_count(),
            Indicator::StochasticOscillator(indicator) => indicator.get_criteria_count(),
            Indicator::OnBalanceVolume(indicator) => indicator.get_criteria_count(),
            Indicator::IchimokuCloud(indicator) => indicator.get_criteria_count(),
        }
    }

    fn clone_with_new_parameters(&self, parameters: &[Variable]) -> Indicator {
        match self {
            Indicator::MovingAverage(indicator) => {
                Indicator::MovingAverage(indicator.clone_with_new_parameters(parameters))
            }
            Indicator::ExponentialMovingAverage(indicator) => {
                Indicator::ExponentialMovingAverage(indicator.clone_with_new_parameters(parameters))
            }
            Indicator::RelativeStrengthIndex(indicator) => {
                Indicator::RelativeStrengthIndex(indicator.clone_with_new_parameters(parameters))
            }
            Indicator::MovingAverageConvergenceDivergence(indicator) => {
                Indicator::MovingAverageConvergenceDivergence(
                    indicator.clone_with_new_parameters(parameters),
                )
            }
            Indicator::BollingerBands(indicator) => {
                Indicator::BollingerBands(indicator.clone_with_new_parameters(parameters))
            }
            Indicator::FibonacciRetracement(indicator) => {
                Indicator::FibonacciRetracement(indicator.clone_with_new_parameters(parameters))
            }
            Indicator::StochasticOscillator(indicator) => {
                Indicator::StochasticOscillator(indicator.clone_with_new_parameters(parameters))
            }
            Indicator::OnBalanceVolume(indicator) => {
                Indicator::OnBalanceVolume(indicator.clone_with_new_parameters(parameters))
            }
            Indicator::IchimokuCloud(indicator) => {
                Indicator::IchimokuCloud(indicator.clone_with_new_parameters(parameters))
            }
        }
    }
}
