use crate::pager::PAGE_SIZE;


#[derive(Debug, Clone)]
pub struct PageHeader {
    pub used: u16,
    pub slots: u16,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub id: u64,
    pub header: PageHeader,
    pub data: [u8; PAGE_SIZE],
    pub slots: Vec<Slot>,
}

#[derive(Debug, Clone)]
pub struct Slot {

    pub offset: u16,

    pub length: u16,

}

impl Page {

    pub fn get_rows(&self) -> Vec<Vec<u8>> {

        let mut rows = Vec::new();
    
        for slot in &self.slots {
    
            let start = slot.offset as usize;
            let end = start + slot.length as usize;
    
            rows.push(
                self.data[start..end].to_vec()
            );
    
        }
    
        rows
    
    }

    pub fn insert_row(
        &mut self,
        row: &[u8],
    ) -> bool {
    
        let start = self.header.used as usize;
    
        let end = start + row.len();
    
        if end > PAGE_SIZE {
    
            return false;
    
        }
    
        self.data[start..end]
            .copy_from_slice(row);
    
        self.slots.push(Slot {
    
            offset: self.header.used,
    
            length: row.len() as u16,
    
        });
    
        self.header.used += row.len() as u16;
    
        self.header.slots += 1;
    
        true
    
    }

    pub fn new(id: u64) -> Self {
        Self {

            id,
        
            header: PageHeader {
        
                used: 0,
        
                slots: 0,
        
            },
        
            data: [0; PAGE_SIZE],
        
        }
    }

}