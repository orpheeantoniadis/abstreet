use crate::utils::{download, osmconvert};

fn input() {
    download(
        "../data/input/geneva/google_transit/",
        "https://gtfs.geops.ch/dl/gtfs_bus.zip",
    );
    download(
        "../data/input/geneva/osm/Geneva.osm",
        "https://download.bbbike.org/osm/extract/planet_6.0453,46.1574_6.2428,46.2569.osm.xz",
    );
}

pub fn osm_to_raw(name: &str) {
    input();
    osmconvert(
        "../data/input/geneva/osm/Geneva.osm",
        format!("../data/input/geneva/polygons/{}.poly", name),
        format!("../data/input/geneva/osm/{}.osm", name),
    );

    println!("- Running convert_osm");
    let map = convert_osm::convert(
        convert_osm::Options {
            osm_input: format!("../data/input/geneva/osm/{}.osm", name),
            city_name: "geneva".to_string(),
            name: name.to_string(),

            parking_shapes: None,
            public_offstreet_parking: None,
            private_offstreet_parking: convert_osm::PrivateOffstreetParking::FixedPerBldg(1),
            sidewalks: None,
            gtfs: Some("../data/input/geneva/google_transit".to_string()),
            elevation: None,
            clip: Some(format!("../data/input/geneva/polygons/{}.poly", name)),
            drive_on_right: true,
        },
        &mut abstutil::Timer::throwaway(),
    );
    let output = format!("../data/input/raw_maps/{}.bin", name);
    println!("- Saving {}", output);
    abstutil::write_binary(output, &map);
}
