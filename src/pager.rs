use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::collections::HashMap;
use crate::page::Page;

pub const PAGE_SIZE: usize = 4096;

pub struct Pager {
    file: File,

    // page number -> page contents
    cache: HashMap<u64, Vec<u8>>,
}

impl Pager {

    pub fn new(filename: &str) -> Self {

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .expect("Failed to open database file");

            Self {

                file,
            
                cache: HashMap::new(),
            
            }
    }

    pub fn read_page(
        &mut self,
        page_number: u64,
    ) -> Vec<u8> {
    
        // Check cache first
        if let Some(page) = self.cache.get(&page_number) {
            return page.clone();
        }
    
        let mut buffer = vec![0; PAGE_SIZE];
    
        self.file
            .seek(SeekFrom::Start(page_number * PAGE_SIZE as u64))
            .unwrap();
    
        self.file
            .read_exact(&mut buffer)
            .ok();
    
        self.cache
            .insert(page_number, buffer.clone());
    
        buffer
    
    }

    pub fn write_page(
        &mut self,
        page_number: u64,
        data: &[u8],
    ) {
    
        self.cache
            .insert(page_number, data.to_vec());
    
        self.file
            .seek(SeekFrom::Start(page_number * PAGE_SIZE as u64))
            .unwrap();
    
        self.file
            .write_all(data)
            .unwrap();
    
        self.file
            .flush()
            .unwrap();
    
    }

    pub fn allocate_page(
        &mut self,
    ) -> u64 {
    
        let file_size = self.file
            .metadata()
            .unwrap()
            .len();
    
        let page_number = file_size / PAGE_SIZE as u64;
    
        let empty_page = vec![0; PAGE_SIZE];
    
        self.write_page(
            page_number,
            &empty_page,
        );
    
        page_number
    
    }

    pub fn load_page(
        &mut self,
        page_number: u64,
    ) -> Page {
    
        let bytes = self.read_page(page_number);
    
        let mut page = Page::new(page_number);
    
        page.data.copy_from_slice(&bytes);
    
        page
    
    }
    
    pub fn save_page(
        &mut self,
        page: &Page,
    ) {
    
        self.write_page(
            page.id,
            &page.data,
        );
    
    }

}