use std::{collections::HashMap, thread::sleep, time::Duration};

use datetime::LocalDateTime;

use crate::{components::factory::PageFactory, data::DataStore, framebuffer::FrameBuffer, providers::factory::ProviderFactory};
pub struct Runner {
    page_factory: PageFactory,
    provider_factory: ProviderFactory,
    providers_next_run: HashMap<String, LocalDateTime>,
    data_store: DataStore
}

impl Runner {
    pub fn new() -> Self {
        let page_factory = PageFactory::new("./pages");
        let provider_factory = ProviderFactory::new("./providers");
        let data_store = DataStore::new("./data");
        let providers_next_run: HashMap<String, LocalDateTime> = HashMap::new();
        Runner{page_factory, provider_factory, providers_next_run, data_store}
    }

    pub fn run(&mut self, frame_buffer: &mut FrameBuffer) {
        loop {
            let now = LocalDateTime::now();
            let provider_list = self.provider_factory.list_providers();
            for provider_name in provider_list  {
                let next_run = self.providers_next_run.get(&provider_name).unwrap_or(&now);
                if next_run <= &now {
                    let mut provider = self.provider_factory.load_provider(&provider_name);
                    provider.provide(&mut self.data_store);
                    let _ = self.providers_next_run.insert(provider_name, now.add_seconds(300));
                }
            }

            let mut page_list = self.page_factory.list_pages();
            page_list.sort();
            for page_name in page_list {
                //println!("Showing {}", name);
                let page = self.page_factory.load_page(&page_name);
                frame_buffer.clear();
                page.produce(&self.data_store).paint_on(frame_buffer);
                frame_buffer.send();
                sleep(Duration::from_secs(page.seconds as u64));
            }
        }

    }

    pub fn save_page(&self, page: &str, frame_buffer: &mut FrameBuffer) {
        println!("Saving page {}", page);
        let page = self.page_factory.load_page(page);
        page.produce(&self.data_store).paint_on(frame_buffer);
        frame_buffer.save_png("out.png");
    }
}