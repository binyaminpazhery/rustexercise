#[derive(Debug, PartialEq)]

pub struct DNA(RNA);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let rna = dna
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                'G' => Ok('C'),
                'C' => Ok('G'),
                'T' => Ok('A'),
                'A' => Ok('U'),
                _ => Err(i),
            })
            .collect::<Result<String, usize>>()?;

        Ok(DNA(RNA(rna)))
    }

    pub fn into_rna(self) -> RNA {
        self.0
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if let Some(i) = rna
            .chars()
            .position(|c| c != 'A' && c != 'C' && c != 'G' && c != 'U')
        {
            Err(i)
        } else {
            Ok(RNA(rna.to_string()))
        }
    }
}