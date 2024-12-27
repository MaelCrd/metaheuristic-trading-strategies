#[cfg(test)]
use crate::objects::{
    indicators::{IndicatorTrait, StochasticOscillator},
    klines::KlineCollection,
};

#[test]
fn test_stochastic_oscillator_calculation() {
    // Create a new kline collection
    let mut kline_collection = KlineCollection::new();

    //   open_time   |  open   |  high   |   low   |  close  |     i_so_5_3_k      |     i_so_5_3_d
    // --------------+---------+---------+---------+---------+---------------------+---------------------
    // 1735325040000 | 94576.6 | 94612.7 |   94567 | 94588.5 |  0.3298429319371876 | 0.28711387710183117
    // 1735324980000 |   94620 | 94620.1 | 94544.4 | 94576.7 | 0.19876923076924868 |  0.3017636566332195

    // Add some klines
    let highs = vec![
        94565.8, 94562.2, 94588.0, 94623.9, 94602.8, 94668.8, 94668.9, 94741.5, 94719.0, 94730.6,
        94706.9, 94669.0, 94678.1, 94667.5, 94620.1, 94612.7,
    ];
    let lows = vec![
        94513.3, 94465.7, 94498.2, 94554.4, 94562.6, 94520.2, 94585.6, 94639.2, 94692.7, 94689.0,
        94579.9, 94565.0, 94626.8, 94620.0, 94544.4, 94567.0,
    ];
    let closes = vec![
        94542.7, 94522.4, 94560.1, 94580.0, 94595.2, 94667.8, 94639.2, 94697.5, 94700.0, 94706.9,
        94579.9, 94657.3, 94626.9, 94620.1, 94576.7, 94588.5,
    ];
    kline_collection.training_percentage = 0.75;
    for i in 0..closes.len() {
        let kline = super::kline_dummy_new(0.0, highs[i], lows[i], closes[i]);
        match i {
            0..8 => kline_collection.past.push(kline),
            8..14 => kline_collection.training.push(kline),
            14..16 => kline_collection.validation.push(kline),
            _ => (),
        }
    }

    // Create a new stochastic oscillator indicator
    let mut so = StochasticOscillator {
        k_period: 5,
        d_period: 3,
        k_values: vec![],
        d_values: vec![],
    };

    // Set all the values to None
    for _ in 0..kline_collection.get_length() {
        so.k_values.push(None);
        so.d_values.push(None);
    }

    // Calculate the stochastic oscillator
    so.calculate(&kline_collection);

    // Check the values - @checked 27/12/2024 on tradingview
    let expected_k_values = vec![
        Some(0.812471757794851),
        Some(0.843651152281946),
        Some(0.0),
        Some(0.5229461756374103),
        Some(0.37379227053135267),
        Some(0.3327294685990573),
        Some(0.19876923076924868),
        Some(0.3298429319371876),
    ];
    let expected_d_values = vec![
        Some(0.8132190594482104),
        Some(0.8190992619370322),
        Some(0.5520409700255989),
        Some(0.4555324426397854),
        Some(0.29891281538958764),
        Some(0.40982263825594006),
        Some(0.3017636566332195),
        Some(0.28711387710183117),
    ];
    for i in 0..so.k_values.len() {
        assert_eq!(so.k_values[i], expected_k_values[i]);
        assert_eq!(so.d_values[i], expected_d_values[i]);
    }
}
