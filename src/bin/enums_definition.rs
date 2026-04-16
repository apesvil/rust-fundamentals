#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    RiberaDelDuero,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: &WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn popularity(w: &WineRegions) -> String {
    match w {
        WineRegions::Bordeaux | WineRegions::Burgundy | WineRegions::Tuscany => String::from("Highly Popular!"),
        WineRegions::Champagne | WineRegions::Rioja | WineRegions::NapaValley => String::from("Popular!"),
        _ => String::from("Less Popular!"),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3: Wine = Wine {
        name: String::from("Pago de Carraovejas"),
        region: WineRegions::RiberaDelDuero,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
    supported_regions(&wine1.region);
    supported_regions(&WineRegions::Rioja);
    println!("Popularity of {}: {}", wine1.name, popularity(&wine1.region));
    println!("Popularity of {}: {}", wine2.name, popularity(&wine2.region));
    println!("Popularity of {}: {}", wine3.name, popularity(&wine3.region));
}
