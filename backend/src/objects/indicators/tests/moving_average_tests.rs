use crate::objects::{
    indicators::{IndicatorTrait, MovingAverage},
    klines::KlineCollection,
};

#[test]
pub fn test_moving_average_calculation() {
    // Create a new kline collection
    let mut kline_collection = KlineCollection::new();

    // Add some klines
    let closes = vec![
        94215.8, 94164.3, 94094.8, 94224.0, 94129.1, 94134.4, 94098.6, 94122.1, 94165.9, 94160.5,
        94173.3, 94206.8, 94154.8, 94134.5, 94036.9,
    ];
    kline_collection.training_percentage = 0.75;
    for (i, close) in closes.iter().enumerate() {
        let kline = super::kline_dummy_new(0.0, 0.0, 0.0, *close);
        match i {
            0..7 => kline_collection.past.push(kline),
            7..13 => kline_collection.training.push(kline),
            13..15 => kline_collection.validation.push(kline),
            _ => (),
        }
    }

    // Create a new moving average indicator
    let mut ma = MovingAverage::new(7);

    // Set all the values to None
    for _ in 0..kline_collection.get_length() {
        ma.values.push(None);
    }

    // Calculate the moving average
    ma.calculate(&kline_collection);

    // Check the values - @checked 27/12/2024 on tradingview
    let expected_values = vec![
        Some(94138.18571428572),
        Some(94138.41428571429),
        Some(94147.8),
        Some(94140.55714285713),
        Some(94151.65714285713),
        Some(94154.57142857143),
        Some(94159.69999999998),
        Some(94147.52857142859),
    ];
    for i in 0..ma.values.len() {
        assert_eq!(ma.values[i], expected_values[i]);
    }
}
