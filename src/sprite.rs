use crate::vector::Vec2;


use crate::GResult;

#[derive(Copy, Clone,Debug)]
pub struct Frame {
    pub id:         u8,
    pub position:   Vec2,
    pub size:       Vec2
}

impl Frame {

    pub fn new(id:u8,x:f32,y:f32,size:Vec2) -> Self {

        Self {
            id,
            position: Vec2::new(x,y),
            size
        }

    }

}


pub struct Sprite {

    pub name: String,
    pub frames: Vec<Frame>

}

pub struct SpriteSheet {

    sprite_size:    Vec2,
    sprites:        Vec<Sprite>,
    current_name:   Option<String>,
    current_id:     u8,
    flipped:        bool
}

impl SpriteSheet {

    pub fn new(w:f32,h:f32) -> Self {

        Self {
            sprite_size:    Vec2::new(w,h),
            sprites:        Vec::new(),
            current_name:   None,
            current_id:     0,
            flipped:        false

        }

    }

    pub fn add_sprite(&mut self, name:&str, nb_frame:u8,sx:f32,sy:f32,offsetx:f32) -> GResult<()> {

        if self.sprite_name_exist(name) {
            bail!("Cant add a sprite {}, already exist",name);
        }


        let mut frames = vec![];
        let mut x:f32 = sx;

        for id in 0..nb_frame {

            frames.push(Frame::new(id,x,sy,self.sprite_size));

            x += self.sprite_size.x + offsetx;

        }

        // if the first sprite add it become the current
        if self.sprites.len() < 1 {
            self.current_name = Some(name.to_string());
        }

        self.sprites.push(Sprite { name: name.to_string(), frames});

        Ok(())

    }


    pub fn flip_sprite(&mut self) {
        self.flipped = !self.flipped;
    }

    pub fn should_flip(&self) -> bool { self.flipped }

    pub fn increment_current_sprite(&mut self) -> GResult<()> {

        match &self.current_name  {

            Some(n) => {
                let length = self.sprites[self.get_sprite_id(n).unwrap()].frames.len();

                if self.current_id as usize >= length - 1 {
                    self.current_id = 0;
                } else {

                    self.current_id += 1;

                }

               // println!("current id: {}",self.current_id);


            },
            None => bail!("no sprite have been added to the sprite sheet, add one first")

        }

        //println!("current: {}",self.current_id);

        Ok(())

    }

    pub fn change_current(&mut self,name:&str) -> GResult<()> {

        if !self.sprite_name_exist(name) {
            bail!("cant change current sprite to '{}'. it does not exist",name);
        }

        self.current_name = Some(name.to_string());
        self.current_id = 0;

        Ok(())

    }



    pub fn get_current_frame(&mut self) -> GResult<Frame> {
        match &self.current_name  {

            Some(n) => {
                let f = self.sprites[self.get_sprite_id(n)?].frames[self.current_id as usize];



                return Ok(f);
            },
            None => bail!("no sprite have been added to the sprite sheet, add one first")

        }

    }

    fn get_sprite_id(&self,name:&str) -> GResult<usize> {

        for (id,sprite)in self.sprites.iter().enumerate() {

            if sprite.name == name {
                return Ok(id);
            }

        }

        bail!("no sprite with the name {} dont exist",name);

    }

    fn sprite_name_exist(&self,name:&str) -> bool {

        let mut found = false;

        for sprite in self.sprites.iter() {

            if name == sprite.name {
                found = true;
                break;
            }
        }

        found

    }

}

