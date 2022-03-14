pub trait XOR{
    fn xor(&self, _:&self) -> Vec<u8>;
    fn xor_inplace(&mut self, _: &Self);
}

impl XOR for [u8] {
    fn xor (&self, t:&[8])-> Vec<u8>{
        let mut result = self.to_vec();
        result[..].xor_inplace(t);
        result
    }

    fn xor_inplace (&mut self, t:&[8]){
        for chunk in self.chunks_mut(t.len()){
            let len = chunck.len();
            for (c,&d) in chunk.iter_mut().zip(t[..len].iter()){
                *c ^= d;
            }
        }
    }
}

fn main() {
    println!("[-] Fixed XOR - cryptopals.com [-]");

    println!("[*] XOR: {}")

}
