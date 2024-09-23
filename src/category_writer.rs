use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

use rayon::prelude::*;

use crate::data_type::Record;
use crate::error::CleanResult;

type ArcRwLock<T> = Arc<RwLock<T>>;
type ArcMutex<T> = Arc<Mutex<T>>;

#[derive(Debug)]
pub struct CategoryWriter {
    pub output_dir:      PathBuf,
    category_writer_map: Mutex<HashMap<String, ArcMutex<BufWriter<File>>>>,
    category_header_map: Mutex<HashMap<String, bool>>,
}

impl Default for CategoryWriter {
    fn default() -> Self {
        Self {
            output_dir:          Default::default(),
            category_writer_map: Default::default(),
            category_header_map: Default::default(),
        }
    }
}

impl CategoryWriter {
    pub fn new(output_dir: &Path) -> CategoryWriter {
        CategoryWriter {
            output_dir:          output_dir.into(),
            category_writer_map: (HashMap::new().into()),
            category_header_map: (HashMap::new().into()),
        }
    }

    pub fn flush(&self) -> CleanResult<()> {
        let writer_map = self.category_writer_map.lock().unwrap();

        writer_map
            .values()
            .try_for_each(|writer_lock| -> CleanResult<()> {
                let mut writer = writer_lock.lock().unwrap();
                Ok(writer.flush()?)
            })
    }

    pub fn process_records<I>(&mut self, records_par_iter: I) -> CleanResult<()>
    where
        I: IntoParallelIterator,
        I::Item: Borrow<Record>,
    {
        records_par_iter.into_par_iter().for_each(|record| {
            let record: &Record = record.borrow();
            let category = record.get_type();
            let writer_lock = self.fetch_writer(category);
            let mut writer = writer_lock.lock().unwrap();
            self.process_record(record, &mut writer).unwrap();
        });

        Ok(())
    }

    fn fetch_writer(&self, category: String) -> Arc<Mutex<BufWriter<File>>> {
        let write_lock = self.category_writer_map.lock().unwrap();
        let mut category_map = write_lock;
        match category_map.get(&category) {
            Some(found_writer) => found_writer.clone(),
            None => {
                let mut file_path = self.output_dir.clone();
                file_path.push(format!("{}.csv", &category));
                let writer: ArcMutex<_> = Arc::new(
                    BufWriter::new(File::create(file_path).unwrap()).into(),
                );

                category_map.insert(category, writer.clone());
                writer
            }
        }
    }

    pub fn process_record(
        &self,
        record: &Record,
        writer: &mut BufWriter<File>,
    ) -> CleanResult<usize> {
        let mut headless = true;
        let category = record.get_type();
        {
            let mut header_map = self.category_header_map.lock().unwrap();
            if header_map.get(&category).is_none() {
                headless = false;
                header_map.insert(category, true);
            }
        }

        if headless {
            Ok(writer.write(record.to_headless_string().as_bytes())?)
        } else {
            Ok(writer.write(record.to_string().as_bytes())?)
        }
    }
}
