


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