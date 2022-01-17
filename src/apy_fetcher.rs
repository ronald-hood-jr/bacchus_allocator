use crate::apy_structures::{self};
use fantoccini::{ClientBuilder, Locator};

pub async fn get_current_mango_apys() ->  Result<apy_structures::MangoApys, Box<dyn std::error::Error>> {
    let mut client = ClientBuilder::native()
    .connect("http://localhost:4444")
    .await?;
    client.goto("https://trade.mango.markets/stats").await?;


    client
    .wait()
    .for_element(Locator::Css(".text-th-green"))
    .await?;

    let green_text = client
    .find_all(Locator::Css(".text-th-green"))
    .await?;
    let mut cleaned_green_text: Vec<f32> = Vec::new();
    for mut element in green_text{
        cleaned_green_text.push(
            element
            .html(true)
            .await
            .unwrap()
            [..element
            .html(true)
            .await
            .unwrap()
            .len()-1]
            .parse::<f32>()
            .unwrap());
    }
    let deposit_rates = cleaned_green_text;
    

    client
    .wait()
    .for_element(Locator::Css(".text-th-red"))
    .await?;

    let red_text = client
    .find_all(Locator::Css(".text-th-red"))
    .await?;
    let mut cleaned_red_text: Vec<f32> = Vec::new();
    for mut element in red_text{
        cleaned_red_text.push(
            element
            .html(true)
            .await
            .unwrap()
            [..element
            .html(true)
            .await
            .unwrap()
            .len()-1]
            .parse::<f32>()
            .unwrap());
    }
    let borrow_rates = cleaned_red_text;

    client
    .wait()
    .for_element(Locator::Css(".undefined:nth-child(6)"))
    .await?;

    let white_text = client
    .find_all(Locator::Css(".undefined:nth-child(6)"))
    .await?;
    let mut cleaned_white_text: Vec<f32> = Vec::new();
    for mut element in white_text{
        cleaned_white_text.push(
            element
            .html(true)
            .await
            .unwrap()
            [..element
            .html(true)
            .await
            .unwrap()
            .len()-1]
            .parse::<f32>()
            .unwrap());
    }
    let utilization_rates = cleaned_white_text;
    

    let mango_holder = apy_structures::MangoApys{
        usdc: apy_structures::MoneyMarketData{deposit_rate: deposit_rates[0], borrow_rate: borrow_rates[0], utilization_rate: utilization_rates[0]},
        mngo:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[1], borrow_rate: borrow_rates[1], utilization_rate: utilization_rates[1]},
        btc:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[2], borrow_rate: borrow_rates[2], utilization_rate: utilization_rates[2]},
        eth:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[3], borrow_rate: borrow_rates[3], utilization_rate: utilization_rates[3]},
        sol:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[4], borrow_rate: borrow_rates[4], utilization_rate: utilization_rates[4]},
        usdt:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[5], borrow_rate: borrow_rates[5], utilization_rate: utilization_rates[5]},
        srm:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[6], borrow_rate: borrow_rates[6], utilization_rate: utilization_rates[6]},
        ray:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[7], borrow_rate: borrow_rates[7], utilization_rate: utilization_rates[7]},
        cope:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[8], borrow_rate: borrow_rates[8], utilization_rate: utilization_rates[8]},
        ftt:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[9], borrow_rate: borrow_rates[9], utilization_rate: utilization_rates[9]},
        msol:  apy_structures::MoneyMarketData{deposit_rate: deposit_rates[10], borrow_rate: borrow_rates[10], utilization_rate: utilization_rates[10]},
        bnb: apy_structures::MoneyMarketData{deposit_rate: deposit_rates[11], borrow_rate: borrow_rates[11], utilization_rate: utilization_rates[11]},
        avax: apy_structures::MoneyMarketData{deposit_rate: deposit_rates[12], borrow_rate: borrow_rates[12], utilization_rate: utilization_rates[12]},
        luna: apy_structures::MoneyMarketData{deposit_rate: deposit_rates[13], borrow_rate: borrow_rates[13], utilization_rate: utilization_rates[13]},
        protocol: "Mango".to_string()
        };

    client.close().await?;
    Ok(mango_holder)
}

pub async fn get_current_port_apys() ->  Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientBuilder::native()
    .connect("http://localhost:4444")
    .await?;
    client.goto("https://mainnet.port.finance/#/markets").await?;
    println!("Connected to Port");

    client
    .wait()
    .for_element(Locator::Css(".text-green em"))
    .await?;
    println!("We found the text element that you were looking for");

    let green_text = client
    .find_all(Locator::Css(".text-green em"))
    .await?;
    for mut element in green_text{
        dbg!(element.html(true).await.unwrap());
    }

    client
    .wait()
    .for_element(Locator::Css(".text-red em"))
    .await?;
    println!("We found the text element that you were looking for");

    let red_text = client
    .find_all(Locator::Css(".text-red em"))
    .await?;
    for mut element in red_text{
        dbg!(element.html(true).await.unwrap());
    }
    client.close().await?;
    Ok(())
}

pub async fn get_current_jet_apys() ->  Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientBuilder::native()
    .connect("http://localhost:4444")
    .await?;
    client.goto("https://app.jetprotocol.io/").await?;
    println!("Connected to Jet");

    client
    .wait()
    .for_element(Locator::XPath("/html/body/div[4]/div[4]/div[2]/table/tbody/tr[2]/td[2]"))
    .await?;
    println!("We found the text element that you were looking for");

    let green_text = client
    .find_all(Locator::XPath("/html/body/div[4]/div[4]/div[2]/table/tbody/tr[2]/td[2]"))
    .await?;
    for mut element in green_text{
        dbg!(element.text().await.unwrap());
    }

    client
    .wait()
    .for_element(Locator::XPath("/html/body/div[4]/div[4]/div[2]/table/tbody/tr[2]/td[2]"))
    .await?;
    println!("We found the text element that you were looking for");

    let red_text = client
    .find_all(Locator::XPath("/html/body/div[4]/div[4]/div[2]/table/tbody/tr[2]/td[2]"))
    .await?;
    for mut element in red_text{
        dbg!(element.html(true).await.unwrap());
    }
    client.close().await?;
    Ok(())
}

pub async fn get_current_katana_apys() ->  Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientBuilder::native()
    .connect("http://localhost:4444")
    .await?;
    client.goto("https://app.katana.so/vaults").await?;
    println!("Connected to Katana");

    client
    .wait()
    .for_element(Locator::Css(".Vault--card__stat"))
    .await?;
    println!("We found the text element that you were looking for");

    let green_text = client
    .find_all(Locator::Css(".Vault--card__stat"))
    .await?;
    for mut element in green_text{
        dbg!(element.text().await.unwrap());
    }

    client.close().await?;
    Ok(())
}

pub async fn get_current_parrot_apys() ->  Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientBuilder::native()
    .connect("http://localhost:4444")
    .await?;
    client.goto("https://parrot.fi/stability/").await?;
    println!("Connected to Parrot");

    client
    .wait()
    .for_element(Locator::Css(".text-brandPrimary.sm\\:text-mdx"))
    .await?;
    println!("We found the text element that you were looking for");

    let green_text = client
    .find_all(Locator::Css(".text-brandPrimary.sm\\:text-mdx"))
    .await?;
    for mut element in green_text{
        dbg!(element.text().await.unwrap());
    }

    client.close().await?;
    Ok(())
}

pub async fn get_current_apricot_apys() ->  Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientBuilder::native()
    .connect("http://localhost:4444")
    .await?;
    client.goto("https://app.apricot.one/").await?;
    println!("Connected to Apricot");

    let button = client
    .wait()
    .for_element(Locator::Css(".Disclaimer_btn__23uxr"))
    .await?;

    match button.click().await {
        Err(_) => println!("Nah bro"),
        _ => println!("nice button press")
    }


    client
    .wait()
    .for_element(Locator::Css(".Lend_deposit__96Dis .Lend_value__whZaz"))
    .await?;
    println!("We found the text element that you were looking for");

    let green_text = client
    .find_all(Locator::Css(".Lend_deposit__96Dis .Lend_value__whZaz"))
    .await?;
    for mut element in green_text{
        dbg!(element.text().await.unwrap());
    }

    client
    .wait()
    .for_element(Locator::Css(".Lend_borrow__335AN .Lend_value__whZaz"))
    .await?;
    println!("We found the text element that you were looking for");

    let red_text = client
    .find_all(Locator::Css(".Lend_borrow__335AN .Lend_value__whZaz"))
    .await?;
    for mut element in red_text{
        dbg!(element.html(true).await.unwrap());
    }
    client.close().await?;
    Ok(())
}

pub async fn get_current_solend_apys() ->  Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientBuilder::native()
    .connect("http://localhost:4444")
    .await?;
    client.goto("https://solend.fi/dashboard").await?;
    println!("Connected to Solend");

    client
    .wait()
    .for_element(Locator::Css(".Market_border__UQb4Z .Market_percent__1fdjc"))
    .await?;
    println!("We found the text element that you were looking for");

    let green_text = client
    .find_all(Locator::Css(".Market_border__UQb4Z .Market_percent__1fdjc"))
    .await?;
    for mut element in green_text{
        dbg!(element.text().await.unwrap());
    }

    client
    .wait()
    .for_element(Locator::Css(".Market_border__UQb4Z .Market_percent__1fdjc"))
    .await?;
    println!("We found the text element that you were looking for");

    let red_text = client
    .find_all(Locator::Css(".ant-table-cell:nth-child(5) .Market_percent__1fdjc"))
    .await?;
    for mut element in red_text{
        dbg!(element.html(true).await.unwrap());
    }
    client.close().await?;
    Ok(())
}