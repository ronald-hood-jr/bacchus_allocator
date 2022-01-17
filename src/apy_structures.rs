pub struct MoneyMarketData {
    pub deposit_rate: f32,
    pub borrow_rate: f32,
    pub utilization_rate: f32
}

/*pub struct UtilizationCurve{
        formula: String
    }*/

pub struct MangoApys {
    pub usdc: MoneyMarketData,
    pub mngo: MoneyMarketData,
    pub btc: MoneyMarketData,
    pub eth: MoneyMarketData,
    pub sol: MoneyMarketData,
    pub usdt: MoneyMarketData,
    pub srm: MoneyMarketData,
    pub ray: MoneyMarketData,
    pub cope: MoneyMarketData,
    pub ftt: MoneyMarketData,
    pub msol: MoneyMarketData,
    pub bnb: MoneyMarketData,
    pub avax: MoneyMarketData,
    pub luna: MoneyMarketData,
    pub protocol: String
}

pub fn return_text() -> String{
"

██████   █████   ██████  ██████ ██   ██ ██    ██ ███████      █████  ██      ██       ██████   ██████  █████  ████████  ██████  ██████  
██   ██ ██   ██ ██      ██      ██   ██ ██    ██ ██          ██   ██ ██      ██      ██    ██ ██      ██   ██    ██    ██    ██ ██   ██ 
██████  ███████ ██      ██      ███████ ██    ██ ███████     ███████ ██      ██      ██    ██ ██      ███████    ██    ██    ██ ██████  
██   ██ ██   ██ ██      ██      ██   ██ ██    ██      ██     ██   ██ ██      ██      ██    ██ ██      ██   ██    ██    ██    ██ ██   ██ 
██████  ██   ██  ██████  ██████ ██   ██  ██████  ███████     ██   ██ ███████ ███████  ██████   ██████ ██   ██    ██     ██████  ██   ██ 
                                                                                                                                        
                                                                                                                                        

".to_string()
}
