use gpx::read;
use kml::{
    types::{Coord, Element, Geometry, KmlDocument, LineString, LineStyle, Placemark, Style},
    Kml,
};

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: gpx-to-kml input.gpx output.kml");
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let gpx_file = File::open(input_file).expect("Unable to open GPX file");
    let gpx_data = read(gpx_file).expect("Unable to read GPX file");

    let mut coords: Vec<Coord> = Vec::new();
    for track in gpx_data.tracks.iter() {
        for segment in track.segments.iter() {
            for waypoint in segment.points.iter() {
                let geo_point = waypoint.point();
                coords.push(Coord::new(geo_point.x(), geo_point.y(), waypoint.elevation));
            }
        }
    }
    let line_string = LineString::from(coords);
    let geometry = Geometry::LineString(line_string);

    let line_style = LineStyle {
        color: "ff0000ff".to_string(),
        width: 4.0,
        ..Default::default()
    };
    let style: Kml = Kml::Style(Style {
        id: Some("myStyle".to_string()),
        line: Some(line_style),
        ..Default::default()
    });
    let style_url = Element {
        name: "styleUrl".to_string(),
        content: Some("#myStyle".to_string()),
        ..Default::default()
    };
    let placemark = Kml::Placemark(Placemark {
        name: Some("gpx-to-kml".to_string()),
        description: Some("gpx-to-kml".to_string()),
        geometry: Some(geometry),
        children: vec![style_url],
        ..Default::default()
    });

    let doc = Kml::Document {
        attrs: Default::default(),
        elements: vec![style, placemark],
    };

    let root = Kml::KmlDocument(KmlDocument {
        elements: vec![doc],
        ..Default::default()
    });

    let kml_string = root.to_string();
    let mut output_file = File::create(output_file).expect("Unable to create KML file");

    output_file
        .write_all(kml_string.as_bytes())
        .expect("Unable to write KML file");
}
