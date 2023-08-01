#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
fn main() {
    println!("Hello, world!");


    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    let transposed_matrix=matrix.transpose();
    println!("Transposed Matrix:\n{:?}", transposed_matrix);
    
}
impl Matrix{
    fn transpose(&self)->Matrix{
        Matrix(self.0,self.2,self.1,self.3)
    }
}

