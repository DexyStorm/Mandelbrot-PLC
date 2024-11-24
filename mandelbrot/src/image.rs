// implement the Pixel struct and traits below

#[derive(Debug, Copy, Clone, PartialEq, Display)]
struct Pixel
{
   r: u8;
   g: u8;
   b: u8;
}

impl fmt::Display for Pixel
{
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
   {
      write!(f, "{}, {}, {}", self.r, self.g, self.b);
   }

}

// implement the Image struct and traits below
struct Image
{
   width: u16;
   height: u16;
   data: Vec<Pixel>
}

impl Image 
{
   
   pub fn new(width: usize, height: usize) -> Image
   {
      self.data = vec![Pixel(0, 0, 0); width*height];
   }

   pub fn get(&self, x: usize, y: usize) -> Option<&Pixel>
   {

   }
}