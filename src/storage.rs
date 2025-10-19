use core::cell::RefCell;

use ekv::Database;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use esp_hal::peripherals::FLASH;
use esp_storage::FlashStorage;

pub const FLASH_SIZE: usize = 0x1F0000;

struct DbFlash<T: NorFlash + ReadNorFlash> {
    start: usize,
    size: usize,
    flash: T,
}

#[repr(C, align(4))]
struct AlignedBuf<const N: usize>([u8; N]);

impl<T: NorFlash + ReadNorFlash> ekv::flash::Flash for DbFlash<T> {
    type Error = T::Error;

    fn page_count(&self) -> usize {
        let possible = self.size / ekv::config::PAGE_SIZE;
        info!("Flash size: {}, possible pages: {}", self.size, possible);
        if possible > ekv::config::MAX_PAGE_COUNT {
            warn!(
                "Flash size allows more pages ({}) than ekv MAX_PAGE_COUNT ({}), limiting to MAX_PAGE_COUNT",
                possible,
                ekv::config::MAX_PAGE_COUNT
            );
            ekv::config::MAX_PAGE_COUNT
        } else {
            possible
        }
    }

    async fn erase(&mut self, page_id: ekv::flash::PageID) -> Result<(), <DbFlash<T> as ekv::flash::Flash>::Error> {
        self.flash.erase(
            (self.start + page_id.index() * ekv::config::PAGE_SIZE) as u32,
            (self.start + page_id.index() * ekv::config::PAGE_SIZE + ekv::config::PAGE_SIZE) as u32,
        )
    }

    async fn read(
        &mut self,
        page_id: ekv::flash::PageID,
        offset: usize,
        data: &mut [u8],
    ) -> Result<(), <DbFlash<T> as ekv::flash::Flash>::Error> {
        let address = self.start + page_id.index() * ekv::config::PAGE_SIZE + offset;
        let mut buf = AlignedBuf([0; ekv::config::PAGE_SIZE]);
        self.flash.read(address as u32, &mut buf.0[..data.len()])?;
        data.copy_from_slice(&buf.0[..data.len()]);
        Ok(())
    }

    async fn write(
        &mut self,
        page_id: ekv::flash::PageID,
        offset: usize,
        data: &[u8],
    ) -> Result<(), <DbFlash<T> as ekv::flash::Flash>::Error> {
        let address = self.start + page_id.index() * ekv::config::PAGE_SIZE + offset;
        let mut buf = AlignedBuf([0; ekv::config::PAGE_SIZE]);
        buf.0[..data.len()].copy_from_slice(data);
        self.flash.write(address as u32, &buf.0[..data.len()])
    }
}

pub async fn init(flash: FLASH<'static>) -> Storage {
    let mut flash = FlashStorage::new(flash).multicore_auto_park();
    let mut buffer = [0u8; esp_bootloader_esp_idf::partitions::PARTITION_TABLE_MAX_LEN];
    let pt = esp_bootloader_esp_idf::partitions::read_partition_table(&mut flash, &mut buffer).unwrap();
    let data = pt
        .iter()
        .find(|p| {
            info!("Partition: {:?}", p);
            p.label_as_str() == "data"
        })
        .expect("no data partition was found");

    info!("Initializing storage on partition: {:?}", data);
    //let flash = BlockingAsync::new(flash);

    let flash = DbFlash { flash, start: data.offset() as usize, size: data.len() as usize };

    let db: Database<DbFlash<FlashStorage<'_>>, CriticalSectionRawMutex> =
        Database::<_, CriticalSectionRawMutex>::new(flash, ekv::Config::default());

    match db.mount().await {
        Ok(_) => info!("Storage mounted successfully"),
        Err(e) => {
            warn!("Failed to mount storage: {:?}, formatting...", e);
            db.format().await.expect("failed to format storage");
            db.mount().await.expect("failed to mount storage after format");
        }
    }

    Storage { db: crate::mk_static::mk_static!(Database<DbFlash<FlashStorage<'static>>, CriticalSectionRawMutex>, db) }
}

#[derive(Clone, Copy)]
pub struct Storage {
    db: &'static Database<DbFlash<FlashStorage<'static>>, CriticalSectionRawMutex>,
}

impl Storage {}
