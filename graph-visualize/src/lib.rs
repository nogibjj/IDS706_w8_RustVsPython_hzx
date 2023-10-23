extern crate rasciigraph;

use rasciigraph::{plot, Config};

// cargo run --manifest-path graph-visualize/Cargo.toml

pub fn vis() {
    let cities = vec![
        "Lisbon", "Madrid", "Paris", "Berlin", "Copenhagen", "Stockholm", "Moscow",
        "Rome", "Athens", "Vienna", "Prague", "Warsaw", "London", "Dublin",
        "Amsterdam", "Brussels", "Ljubljana", "Budapest", "Zurich", "Oslo",
        "Riga", "Helsinki", "Reykjavik", "Sofia", "Bucharest", "Belgrade",
        "Skopje", "Tirana", "Bratislava", "Zagreb", "Vilnius", "Tallinn",
        "Valletta", "Nicosia", "Luxembourg", "Ljubljana", "Bern", "Andorra la Vella",
        "Monaco", "Vatican City"
    ];
    let distances_travelled = vec![
        0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35,
        1290.43, 3100.62, 1175.8, 886.81, 1593.29, 1768.68, 2258.41,
        431.55, 752.59, 545.02, 1461.13, 1768.94, 1135.99,
        373.12, 1050.78, 2381.09, 1489.97, 2087.67, 1855.2,
        191.16, 254.58, 335.74, 410.57, 395.59, 392.11,
        1890.46, 1578.03, 1103.49, 1122.68, 1543.82, 1444.15,
        1563.35, 147.68
    ];

    println!("{}", cities.join(" > "));

    println!(
        "{}",
        plot(
            distances_travelled.into_iter().map(|d| d as f64).collect(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Travelled distances (km)".to_string())
        )
    );
}
