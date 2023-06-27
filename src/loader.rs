use crate::GResult;
use crate::tile::TileInfo;

use std::path::Path;
use std::fs;
use serde_json::Value;

pub type Level = Vec<Vec<i16>>;
pub type Tileset = Vec<TileInfo>;


pub fn load_level(fp:&str) -> GResult<Level> {

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(fp)?;

    let mut buffer= Level::new();


    for record in reader.records() {

        let row = record?;

        let mut row_buff:Vec<i16> = Vec::new();

        for val in row.iter() {

            row_buff.push(val.parse::<i16>()?);

        }

        buffer.push(row_buff);
    }

    Ok(buffer)


}

fn load_json(fp:&str) -> GResult<Value> {

    let desc = Path::new(fp);

    if !desc.exists() || !desc.is_file() {

        bail!("{}: no file found",fp);

    }

    match desc.extension() {

        Some(ext) => {

            if ext != "json" {

                bail!("{} : description file need to be a json file",fp);

            }

        }
        None => bail!("{} description file need a json file extension", fp)

    }

    let desc_data = fs::read_to_string(desc)?;


    let json_value = serde_json::from_str(&desc_data)?;


    Ok(json_value)


}


pub fn load_tileset(tile_dir:&str) -> GResult<Tileset> {

    let desc_file = format!("{}/data.json",tile_dir);
    let data = load_json(&desc_file)?;

    let mut tileset  = Tileset::new();

    let tiles = match data["tiles"].as_array() {

        Some(ts) => ts,
        None => bail!("Tile data json is malformed, tiles key must be an array")

    };


    for tile in tiles.iter() {

        let id = match tile["id"].as_u64() {

            Some(i) => i as i16,
            None => bail!("tile data corrupted, 'id' key must be present and a number")

        };

        let img = match tile["image"].as_str() {

            Some(src) => src,
            None => bail!("tile data corrupted, 'image' key must be present and a string")

        };

        let wall = match tile["wall"].as_bool() {

            Some(w) => w,
            None =>  bail!("tile data corrupted, 'wall' key must be present and a boolean")

        };

        let pimg = format!("{}/{}",tile_dir,img);

        let tinfo = TileInfo::new(id,&pimg,wall);

        tileset.push(tinfo);


    }


    Ok(tileset)

}





