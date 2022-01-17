mod apy_fetcher;
mod apy_structures;
mod rates_database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{ 
    /*println!("{}",apy_structures::return_text());
    rates_database::create_rates_databases();
    loop{
        rates_database::update_rates_databases().await;
    }*/
    //apy_fetcher::get_current_port_apys().await;
    //apy_fetcher::get_current_jet_apys().await;
    //apy_fetcher::get_current_katana_apys().await;
    //apy_fetcher::get_current_parrot_apys().await;
    //apy_fetcher::get_current_apricot_apys().await;
    apy_fetcher::get_current_solend_apys().await
}